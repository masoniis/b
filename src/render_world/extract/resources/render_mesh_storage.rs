use crate::{
    game_world::global_resources::asset_storage::AssetId, render_world::types::mesh::GpuMesh,
};
use bevy_ecs::prelude::Resource;
use std::{collections::HashMap, sync::Arc};

#[derive(Resource, Default)]
pub struct RenderMeshStorageResource {
    pub meshes: HashMap<AssetId, Arc<GpuMesh>>,
}
