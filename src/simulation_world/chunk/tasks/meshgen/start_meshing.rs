use crate::prelude::*;
use crate::simulation_world::chunk::chunk_state_manager::NEIGHBOR_OFFSETS;
use crate::simulation_world::chunk::mesh::TransparentMeshComponent;
use crate::simulation_world::chunk::padded_chunk_view::{ChunkDataOption, PaddedChunkView};
use crate::simulation_world::chunk::{
    CheckForMeshing, ChunkMeshingTaskComponent, ChunkState, WantsMeshing,
};
use crate::simulation_world::{
    asset_management::{texture_map_registry::TextureMapResource, AssetStorageResource, MeshAsset},
    block::BlockRegistryResource,
    chunk::{
        build_chunk_mesh, ChunkBlocksComponent, ChunkCoord, ChunkStateManager, OpaqueMeshComponent,
    },
};
use bevy_ecs::prelude::*;
use crossbeam::channel::unbounded;

/// Queries for chunks needing meshing and starts a limited number of tasks per frame.
#[instrument(skip_all)]
pub fn start_pending_meshing_tasks_system(
    // Input
    mut pending_chunks_query: Query<
        (Entity, &ChunkBlocksComponent, &ChunkCoord),
        (
            With<WantsMeshing>,
            With<CheckForMeshing>,
            Without<ChunkMeshingTaskComponent>,
        ),
    >,
    all_generated_chunks: Query<&ChunkBlocksComponent>, // for finding neighbors

    // Resources needed to start meshing
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkStateManager>,
    texture_map: Res<TextureMapResource>,
    block_registry: Res<BlockRegistryResource>,
    mesh_assets: Res<AssetStorageResource<MeshAsset>>,
) {
    'chunk_loop: for (entity, chunk_comp, chunk_coord) in pending_chunks_query.iter_mut() {
        // check for cancellation
        match chunk_manager.get_state(chunk_coord.pos) {
            Some(ChunkState::WantsMeshing(state_entity)) if state_entity == entity => {
                // state is correct, proceed to start meshing
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Chunk {} marked NeedsMeshing but manager state is not NeedsMeshing({:?}). Assuming cancelled.",
                    chunk_coord.pos, entity
                );
                continue;
            }
        }

        // INFO: ----------------------------------------------
        //         Ensure neighbors have been generated
        // ----------------------------------------------------

        let get_neighbor = |offset: IVec3| -> Option<ChunkDataOption> {
            let neighbor_coord = chunk_coord.pos + offset;

            if !ChunkStateManager::is_in_bounds(neighbor_coord) {
                return Some(ChunkDataOption::OutOfBounds);
            }

            match chunk_manager.get_state(neighbor_coord) {
                Some(ChunkState::Loaded(None)) => Some(ChunkDataOption::Empty),
                Some(ChunkState::Loaded(Some(entity)))
                | Some(ChunkState::DataReady(entity))
                | Some(ChunkState::WantsMeshing(entity))
                | Some(ChunkState::Meshing(entity)) => {
                    let blocks = all_generated_chunks.get(entity).unwrap();
                    Some(ChunkDataOption::Generated(blocks.clone()))
                }
                _ => None, // neighbor not generated
            }
        };

        let mut chunks: [[[ChunkDataOption; 3]; 3]; 3] = Default::default();
        chunks[1][1][1] = ChunkDataOption::Generated(chunk_comp.clone()); // center chunk

        for chunk in NEIGHBOR_OFFSETS {
            let neighbor_data = match get_neighbor(chunk) {
                Some(data) => data,
                None => {
                    commands.entity(entity).remove::<CheckForMeshing>();
                    continue 'chunk_loop; // abort as neighbor not generated
                }
            };

            // map offset (e.g., -1, 0, 1) to array index (e.g., 0, 1, 2)
            let idx_x = (chunk.x + 1) as usize;
            let idx_y = (chunk.y + 1) as usize;
            let idx_z = (chunk.z + 1) as usize;

            chunks[idx_x][idx_y][idx_z] = neighbor_data;
        }

        trace!(target: "chunk_loading", "Starting meshing task for {}.", chunk_coord.pos);

        // INFO: -----------------------------
        //         Spawn the mesh task
        // -----------------------------------

        let texture_map_clone = texture_map.clone();
        let block_registry_clone = block_registry.clone();
        let mesh_assets_clone = mesh_assets.clone();
        let coord_clone = chunk_coord.clone();
        let padded_view = PaddedChunkView::new(chunks);

        let (sender, receiver) = unbounded();
        rayon::spawn(move || {
            let (opaque_mesh_option, transparent_mesh_option) = build_chunk_mesh(
                &coord_clone.to_string(),
                padded_view,
                &texture_map_clone,
                &block_registry_clone,
            );

            let omesh = if let Some(opaque_mesh) = opaque_mesh_option {
                let mesh_handle = mesh_assets_clone.add(opaque_mesh);
                Some(OpaqueMeshComponent::new(mesh_handle))
            } else {
                None
            };

            let tmesh = if let Some(transparent_mesh) = transparent_mesh_option {
                let mesh_handle = mesh_assets_clone.add(transparent_mesh);
                Some(TransparentMeshComponent::new(mesh_handle))
            } else {
                None
            };

            let _ = sender.send((omesh, tmesh));
        });

        // update entity and manager
        commands
            .entity(entity)
            .insert(ChunkMeshingTaskComponent { receiver })
            .remove::<CheckForMeshing>()
            .remove::<WantsMeshing>();

        chunk_manager.mark_as_meshing(chunk_coord.pos, entity);
    }
}
