use crate::{
    render_world::passes::world::gpu_resources::VoxelMesh,
    simulation_world::asset_management::asset_storage::AssetId,
};
use bevy_ecs::prelude::Resource;
use std::{collections::HashMap, sync::Arc};

#[derive(Resource, Default)]
pub struct RenderMeshStorageResource {
    pub meshes: HashMap<AssetId, Arc<VoxelMesh>>,
}

impl std::fmt::Debug for RenderMeshStorageResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderMeshStorageResource")
            .field("mesh_count", &self.meshes.len())
            .finish()
    }
}
