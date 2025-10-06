use crate::{
    game_world::global_resources::{AssetStorageResource, MeshAsset},
    prelude::*,
    render_world::types::mesh::create_gpu_mesh_from_data,
    render_world::{
        extract::{mesh::RenderMeshComponent, resources::RenderMeshStorageResource},
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;
use std::sync::Arc;
use tracing::warn;

pub fn prepare_meshes_system(
    gfx_context: Res<GraphicsContextResource>,
    // This is a clone of the main world's asset storage.
    // It's cheap to clone because it uses Arcs internally.
    cpu_mesh_assets: Res<AssetStorageResource<MeshAsset>>,
    mut gpu_mesh_storage: ResMut<RenderMeshStorageResource>,
    // Query for all entities with a mesh component in the render world.
    meshes_to_prepare: Query<&RenderMeshComponent>,
) {
    for render_mesh in meshes_to_prepare.iter() {
        let handle = render_mesh.mesh_handle;

        // If the GPU mesh for this handle doesn't exist yet, create it.
        if gpu_mesh_storage.meshes.get(&handle.id()).is_none() {
            // Get the CPU-side asset data.
            if let Some(mesh_asset) = cpu_mesh_assets.get(handle) {
                // Perform the GPU buffer creation.
                let gpu_mesh = create_gpu_mesh_from_data(
                    &gfx_context.context.device,
                    &mesh_asset.vertices,
                    &mesh_asset.indices,
                );
                info!("Prepared GPU mesh for handle ID {}", handle.id());
                gpu_mesh_storage
                    .meshes
                    .insert(handle.id(), Arc::new(gpu_mesh));
            } else {
                warn!(
                    "Mesh asset for handle ID {} not found in AssetStorage.",
                    handle.id()
                );
            }
        }
    }
}
