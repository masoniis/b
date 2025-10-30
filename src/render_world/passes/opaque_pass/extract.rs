use crate::{
    render_world::global_extract::MirrorableComponent,
    simulation_world::{
        asset_management::{asset_storage::Handle, MeshAsset},
        chunk::{OpaqueMeshComponent, TransformComponent},
    },
};
use bevy_ecs::prelude::*;
use glam::Mat4;

// INFO: --------------------------------
//         RenderWorld components
// --------------------------------------

/// A component in the render world holding the extracted mesh handle.
#[derive(Component, Clone)]
pub struct OpaqueRenderMeshComponent {
    pub mesh_handle: Handle<MeshAsset>,
}

// A component representing a transform on a mesh
#[derive(Component, Clone)]
pub struct RenderTransformComponent {
    pub transform: Mat4,
}

// INFO: ------------------------------------
//         GameWorld extraction logic
// ------------------------------------------

// We want to mirror properties of `MeshComponent` from the simulation world
impl MirrorableComponent for OpaqueMeshComponent {
    type Dependencies = &'static TransformComponent;
    type RenderBundle = (OpaqueRenderMeshComponent, RenderTransformComponent);

    type Filter = Or<(
        Added<OpaqueMeshComponent>,
        Changed<OpaqueMeshComponent>,
        Changed<TransformComponent>,
    )>;

    fn to_render_bundle(&self, transform: &TransformComponent) -> Self::RenderBundle {
        (
            OpaqueRenderMeshComponent {
                mesh_handle: self.mesh_handle,
            },
            RenderTransformComponent {
                transform: transform.to_matrix(),
            },
        )
    }
}
