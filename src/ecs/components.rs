use bevy_ecs::prelude::Component;
use glam::{Mat4, Quat, Vec3};

#[derive(Component)]
pub struct Mesh {
    // For simplicity, let's assume a simple vertex buffer for now.
    // In a real application, this would likely be an ID to a GPU buffer.
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

#[derive(Component)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}
