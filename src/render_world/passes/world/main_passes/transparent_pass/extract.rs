use crate::{
    render_world::{
        global_extract::MirrorableComponent,
        passes::world::main_passes::opaque_pass::extract::RenderTransformComponent,
    },
    simulation_world::{
        asset_management::{asset_storage::Handle, MeshAsset},
        chunk::{mesh::TransparentMeshComponent, TransformComponent},
    },
};
use bevy_ecs::prelude::*;

// INFO: --------------------------------
//         RenderWorld components
// --------------------------------------

/// A component in the render world holding the extracted mesh handle.
#[derive(Component, Clone)]
pub struct TransparentRenderMeshComponent {
    pub mesh_handle: Handle<MeshAsset>,
}

// INFO: ------------------------------------
//         GameWorld extraction logic
// ------------------------------------------

// We want to mirror properties of `MeshComponent` from the simulation world
impl MirrorableComponent for TransparentMeshComponent {
    type Dependencies = &'static TransformComponent;
    type RenderBundle = (TransparentRenderMeshComponent, RenderTransformComponent);

    type Filter = Or<(
        Added<TransparentMeshComponent>,
        Changed<TransparentMeshComponent>,
        Changed<TransformComponent>,
    )>;

    fn to_render_bundle(&self, transform: &TransformComponent) -> Self::RenderBundle {
        (
            TransparentRenderMeshComponent {
                mesh_handle: self.mesh_handle,
            },
            RenderTransformComponent {
                transform: transform.to_matrix(),
            },
        )
    }
}
