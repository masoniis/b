use bevy_ecs::prelude::Resource;
use glam::{Mat4, Vec3};

#[derive(Resource)]
pub struct CameraResource {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,

    pub yaw: f32,
    pub pitch: f32,

    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,

    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,

    // Dirty bits for system to read so they know
    // they need to update the camera resource.
    pub projection_dirty: bool,
}

impl Default for CameraResource {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),

            yaw: -90.0,
            pitch: 0.0,

            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,

            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,

            projection_dirty: true,
        }
    }
}

impl CameraResource {
    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }
}
