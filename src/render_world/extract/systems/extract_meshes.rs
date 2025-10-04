use crate::{
    game_world::graphics::components::{mesh::MeshComponent, transform::TransformComponent},
    prelude::*,
};
use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct RenderMeshComponent {
    pub mesh_handle:
        crate::ecs_resources::asset_storage::Handle<crate::ecs_resources::asset_storage::MeshAsset>,
}

// #[derive(Component, Clone)]
// pub struct RenderTransformComponent {
//     pub transform: crate::game_world::graphics::components::transform::Transform,
// }

pub fn extract_meshes_system(
    mut commands: Commands,
    meshes: Query<(Entity, &MeshComponent, &TransformComponent)>,
) {
    // let mut render_entities = Vec::new();
    // for (entity, mesh, transform) in meshes.iter() {
    //     render_entities.push((
    //         entity,
    //         (
    //             RenderMeshComponent {
    //                 mesh_handle: mesh.mesh_handle.clone(),
    //             },
    //             RenderTransformComponent {
    //                 transform: transform.transform.clone(),
    //             },
    //         ),
    //     ));
    // }
    //
    // if !render_entities.is_empty() {
    //     commands.insert_resource(ExtractedMeshes(render_entities));
    // }
    info!("Extracted no meshes");
}

// #[derive(Resource)]
// pub struct ExtractedMeshes(pub Vec<(Entity, (RenderMeshComponent, RenderTransformComponent))>);
