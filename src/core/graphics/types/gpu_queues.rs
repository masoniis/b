use crate::ecs_resources::asset_storage::{Handle, MeshAsset};
use bevy_ecs::prelude::Entity;
use glyphon::Color;

pub struct QueuedDraw {
    pub entity: Entity, // requires entity for mapping removal in the queue
    pub mesh_handle: Handle<MeshAsset>,
    pub instance_count: u32,
    pub transform: glam::Mat4,
}

pub struct QueuedText {
    pub text: String,
    pub position: glam::Vec2,
    pub color: Color,
    pub font_size: f32,
}

impl Default for QueuedText {
    fn default() -> Self {
        Self {
            text: String::default(),
            position: glam::Vec2::default(),
            color: Color::rgb(0xFF, 0xFF, 0xFF),
            font_size: f32::default(),
        }
    }
}
