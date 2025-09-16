use crate::graphics::buffers::Buffer;
use bevy_ecs::prelude::Component;
use glam::Vec2;

#[derive(Component)]
pub struct MeshComponent {
    pub buffer: Buffer,
    pub atlas_id: String,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
}

impl MeshComponent {
    /// Creates a new mesh from raw vertex and index data.
    pub fn new(
        vertices: &[f32],
        indices: &[u32],
        atlas_id: String,
        uv_min: Vec2,
        uv_max: Vec2,
    ) -> Self {
        Self {
            buffer: Buffer::new(vertices, indices),
            atlas_id,
            uv_min,
            uv_max,
        }
    }

    /// Creates a new mesh with the geometry of a 1x1x1 cube centered at the origin.
    pub fn new_cube(atlas_id: String, uv_min: Vec2, uv_max: Vec2) -> Self {
        // Define the 8 vertices of the cube with position and texture coordinates
        let vertices: &[f32] = &[
            // positions      // texture coords
            // Front face
            -0.5, -0.5, 0.5, uv_min.x, uv_min.y, // 0
            0.5, -0.5, 0.5, uv_max.x, uv_min.y, // 1
            0.5, 0.5, 0.5, uv_max.x, uv_max.y, // 2
            -0.5, 0.5, 0.5, uv_min.x, uv_max.y, // 3
            // Back face (order changed to match Python)
            0.5, -0.5, -0.5, uv_min.x, uv_min.y, // 4
            -0.5, -0.5, -0.5, uv_max.x, uv_min.y, // 5
            -0.5, 0.5, -0.5, uv_max.x, uv_max.y, // 6
            0.5, 0.5, -0.5, uv_min.x, uv_max.y, // 7
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

        return Self {
            buffer: Buffer::new(vertices, indices),
            atlas_id,
            uv_min,
            uv_max,
        };
    }
}
