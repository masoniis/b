use crate::prelude::*;
use crate::simulation_world::chunk::mesh::TransparentMeshComponent;
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

/// Neighboring chunk data needed for meshing.
pub struct ChunkNeighborData {
    pub right: Option<ChunkBlocksComponent>,  // +X
    pub left: Option<ChunkBlocksComponent>,   // -X
    pub top: Option<ChunkBlocksComponent>,    // +Y
    pub bottom: Option<ChunkBlocksComponent>, // -Y
    pub front: Option<ChunkBlocksComponent>,  // +Z
    pub back: Option<ChunkBlocksComponent>,   // -Z
}

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

        // TODO: try to use a more concise method like iter neighbors here.
        // also depensd on how we are constructing padded chunk data which
        // is currently TBD

        // for chunk in chunk_manager.iter_neighbors(chunk_coord.pos) {}

        let get_neighbor = |offset: IVec3| -> Option<Option<ChunkBlocksComponent>> {
            let neighbor_coord = chunk_coord.pos + offset;
            match chunk_manager.get_entity(neighbor_coord) {
                Some(entity) => match all_generated_chunks.get(entity) {
                    Ok(blocks) => Some(Some(blocks.clone())), // found data
                    Err(_) => None,                           // must wait for generation
                },
                None => Some(None), // is out of bounds
            }
        };

        let right = match get_neighbor(IVec3::new(1, 0, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let left = match get_neighbor(IVec3::new(-1, 0, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let top = match get_neighbor(IVec3::new(0, 1, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let bottom = match get_neighbor(IVec3::new(0, -1, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let front = match get_neighbor(IVec3::new(0, 0, 1)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let back = match get_neighbor(IVec3::new(0, 0, -1)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };

        let neighbor_data_for_task = ChunkNeighborData {
            right,
            left,
            top,
            bottom,
            front,
            back,
        };

        trace!(target: "chunk_loading", "Starting meshing task for {}.", chunk_coord.pos);

        // INFO: -----------------------------
        //         Spawn the mesh task
        // -----------------------------------

        let texture_map_clone = texture_map.clone();
        let block_registry_clone = block_registry.clone();
        let mesh_assets_clone = mesh_assets.clone();
        let chunk_component_for_task = chunk_comp.clone();
        let coord_clone = chunk_coord.clone();

        let (sender, receiver) = unbounded();
        rayon::spawn(move || {
            let (opaque_mesh_option, transparent_mesh_option) = build_chunk_mesh(
                &coord_clone.to_string(),
                &chunk_component_for_task,
                &neighbor_data_for_task,
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
