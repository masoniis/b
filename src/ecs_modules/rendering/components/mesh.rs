use crate::ecs_resources::asset_storage::Handle;
use crate::ecs_resources::asset_storage::MeshAsset;
use bevy_ecs::prelude::Component;
use glam::Vec2;

#[derive(Component)]
pub struct MeshComponent {
    pub mesh_handle: Handle<MeshAsset>,

    pub atlas_id: String,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
}

impl MeshComponent {
    /// Creates a new mesh from raw vertex and index data.
    pub fn new(
        atlas_id: String,
        uv_min: Vec2,
        uv_max: Vec2,
        mesh_handle: Handle<MeshAsset>,
    ) -> Self {
        Self {
            atlas_id,
            uv_min,
            uv_max,
            mesh_handle,
        }
    }
}
