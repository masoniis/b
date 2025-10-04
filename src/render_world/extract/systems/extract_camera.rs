use crate::{
    ecs_resources::CameraResource, prelude::*,
    render_world::extract::utils::run_extract_schedule::GameWorld,
};
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

pub fn extract_camera_system(mut _commands: Commands, game_world: Res<GameWorld>) {
    info!("Extracting camera...");
}
