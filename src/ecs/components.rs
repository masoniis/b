use crate::graphics::buffers::Buffer;
use crate::graphics::textures::Texture;
use bevy_ecs::prelude::Component;
use glam::{Mat4, Quat, Vec3};

#[derive(Component)]
pub struct Mesh {
    pub buffer: Buffer,
    pub texture: Option<Texture>,
}

impl Mesh {
    /// Creates a new mesh from raw vertex and index data.
    pub fn new(vertices: &[f32], indices: &[u32], texture: Option<Texture>) -> Self {
        Self {
            buffer: Buffer::new(vertices, indices),
            texture,
        }
    }

    /// Creates a new mesh with the geometry of a 1x1x1 cube centered at the origin.
    pub fn new_cube(texture_path: &str) -> Self {
        // Define the 8 vertices of the cube with position and texture coordinates
        let vertices: &[f32] = &[
            // positions      // texture coords
            // Front face
            -0.5, -0.5, 0.5, 0.0, 0.0, // 0
            0.5, -0.5, 0.5, 1.0, 0.0, // 1
            0.5, 0.5, 0.5, 1.0, 1.0, // 2
            -0.5, 0.5, 0.5, 0.0, 1.0, // 3
            // Back face (order changed to match Python)
            0.5, -0.5, -0.5, 0.0, 0.0, // 4
            -0.5, -0.5, -0.5, 1.0, 0.0, // 5
            -0.5, 0.5, -0.5, 1.0, 1.0, // 6
            0.5, 0.5, -0.5, 0.0, 1.0, // 7
        ];

        let indices: &[u32] = &[
            // Front face
            0, 1, 2, 2, 3, 0, // Back face
            4, 5, 6, 6, 7, 4, // Top face
            3, 2, 7, 7, 6, 3, // Bottom face
            5, 4, 1, 1, 0, 5, // Right face
            1, 4, 7, 7, 2, 1, // Left face
            5, 0, 3, 3, 6, 5,
        ];

        let texture = crate::graphics::textures::Texture::new(texture_path)
            .expect("Failed to load texture for cube");

        return Self {
            buffer: Buffer::new(vertices, indices),
            texture: Some(texture),
        };
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
