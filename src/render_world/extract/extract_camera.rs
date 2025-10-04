use crate::{ecs_resources::CameraResource, prelude::*};
use bevy_ecs::prelude::*;
use glam::{Mat4, Vec3};

/// A simplified, render-ready representation of the main world's camera.
/// This will be a resource in the RenderWorld.
#[derive(Resource, Default)]
pub struct RenderCamera {
    pub projection: Mat4,
    pub view: Mat4,
    pub position: Vec3,
}

// pub fn extract_camera_system(
//     // `Commands` here will operate on the RenderWorld.
//     mut commands: Commands,
//     // We get read-only access to a resource from the MainWorld.
//     // The application runner is responsible for making this resource available.
//     main_world_camera: Res<CameraResource>,
// ) {
//     commands.insert_resource(RenderCamera {
//         projection: main_world_camera.projection_matrix(),
//         view: main_world_camera.view_matrix(),
//         position: main_world_camera.position,
//     });
// }
