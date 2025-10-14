use crate::{
    render_world::extract::MirrorableComponent,
    simulation_world::{
        global_resources::asset_storage::{Handle, MeshAsset},
        graphics_old::{MeshComponent, TransformComponent},
    },
};
use bevy_ecs::prelude::*;
use glam::Mat4;

// INFO: --------------------------------
//         RenderWorld components
// --------------------------------------

/// A component in the render world holding the extracted mesh handle.
#[derive(Component, Clone)]
pub struct RenderMeshComponent {
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
impl MirrorableComponent for MeshComponent {
    type Dependencies = &'static TransformComponent;
    type RenderBundle = (RenderMeshComponent, RenderTransformComponent);

    type Filter = Or<(
        Added<MeshComponent>,
        Changed<MeshComponent>,
        Changed<TransformComponent>,
    )>;

    fn to_render_bundle(&self, transform: &TransformComponent) -> Self::RenderBundle {
        (
            RenderMeshComponent {
                mesh_handle: self.mesh_handle,
            },
            RenderTransformComponent {
                transform: transform.to_matrix(),
            },
        )
    }
}
