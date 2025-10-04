use crate::{core::graphics::types::mesh::GpuMesh, ecs_resources::asset_storage::AssetId};
use bevy_ecs::prelude::Resource;
use std::{collections::HashMap, sync::Arc};

#[derive(Resource, Default)]
pub struct RenderMeshStorageResource {
    pub meshes: HashMap<AssetId, Arc<GpuMesh>>,
}
