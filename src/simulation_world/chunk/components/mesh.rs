use crate::simulation_world::{asset_management::asset_storage::Handle, MeshAsset};
use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct MeshComponent {
    pub mesh_handle: Handle<MeshAsset>,
}

impl MeshComponent {
    /// Creates a new mesh from raw vertex and index data.
    pub fn new(mesh_handle: Handle<MeshAsset>) -> Self {
        Self { mesh_handle }
    }
}
