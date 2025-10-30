use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderMeshStorageResource,
        graphics_context::resources::RenderDevice,
        passes::opaque_pass::extract::OpaqueRenderMeshComponent,
        types::mesh::create_gpu_mesh_from_data,
    },
    simulation_world::asset_management::{AssetStorageResource, MeshAsset},
};
use bevy_ecs::prelude::*;
use std::sync::Arc;
use tracing::warn;

#[instrument(skip_all)]
pub fn prepare_opaque_meshes_system(
    device: Res<RenderDevice>,
    cpu_mesh_assets: Res<AssetStorageResource<MeshAsset>>,
    meshes_to_prepare: Query<&OpaqueRenderMeshComponent>,

    // Output (storage insertion)
    mut gpu_mesh_storage: ResMut<RenderMeshStorageResource>,
) {
    for render_mesh in meshes_to_prepare.iter() {
        let handle = render_mesh.mesh_handle;

        // if the GPU mesh for this handle doesn't exist yet, create it.
        if gpu_mesh_storage.meshes.get(&handle.id()).is_none() {
            // get the asset data
            if let Some(mesh_asset) = cpu_mesh_assets.get(handle) {
                // create the GPU buffer
                let gpu_mesh =
                    create_gpu_mesh_from_data(&device, &mesh_asset.vertices, &mesh_asset.indices);

                debug!(
                    target : "gpu_mesh_prepared",
                    "Prepared opaque GPU mesh for handle ID {}",
                    handle.id()
                );

                gpu_mesh_storage
                    .meshes
                    .insert(handle.id(), Arc::new(gpu_mesh));
            } else {
                warn!(
                    "Mesh asset for handle ID {} not found in AssetStorage (opaque pass).",
                    handle.id()
                );
            }
        }
    }
}
