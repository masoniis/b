use crate::graphics::buffers::Buffer;
use bevy_ecs::prelude::Component;
use glam::{Mat4, Quat, Vec3};

#[derive(Component)]
pub struct Mesh {
    pub buffer: Buffer,
}

impl Mesh {
    /// Creates a new mesh from raw vertex and index data.
    pub fn new(vertices: &[f32], indices: &[u32]) -> Self {
        Self {
            buffer: Buffer::new(vertices, indices),
        }
    }

    /// Creates a new mesh with the geometry of a 1x1x1 cube centered at the origin.
    pub fn new_cube() -> Self {
        // Define the 8 vertices of the cube
        let vertices: &[f32] = &[
            // Position          // Normal (optional, for lighting)
            // Front face
            -0.5, -0.5, 0.5, // 0
            0.5, -0.5, 0.5, // 1
            0.5, 0.5, 0.5, // 2
            -0.5, 0.5, 0.5, // 3
            // Back face
            -0.5, -0.5, -0.5, // 4
            0.5, -0.5, -0.5, // 5
            0.5, 0.5, -0.5, // 6
            -0.5, 0.5, -0.5, // 7
        ];

        // Define the 12 triangles (2 for each face) using the vertex indices
        let indices: &[u32] = &[
            // Front
            0, 1, 2, 2, 3, 0, // Right
            1, 5, 6, 6, 2, 1, // Back
            7, 6, 5, 5, 4, 7, // Left
            4, 0, 3, 3, 7, 4, // Top
            3, 2, 6, 6, 7, 3, // Bottom
            4, 5, 1, 1, 0, 4,
        ];

        Self {
            buffer: Buffer::new(vertices, indices),
        }
    }
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

impl Transform {
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}
