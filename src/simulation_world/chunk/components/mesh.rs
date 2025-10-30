use crate::simulation_world::{asset_management::asset_storage::Handle, MeshAsset};
use bevy_ecs::prelude::Component;

#[derive(Component, Debug)]
pub struct OpaqueMeshComponent {
    pub mesh_handle: Handle<MeshAsset>,
}

impl OpaqueMeshComponent {
    /// Creates a new opaque-rendered mesh from raw vertex and index data.
    pub fn new(mesh_handle: Handle<MeshAsset>) -> Self {
        Self { mesh_handle }
    }
}

#[derive(Component, Debug)]
pub struct TransparentMeshComponent {
    pub mesh_handle: Handle<MeshAsset>,
}

impl TransparentMeshComponent {
    /// Creates a new transparent-rendered mesh from raw vertex and index data.
    pub fn new(mesh_handle: Handle<MeshAsset>) -> Self {
        Self { mesh_handle }
    }
}
