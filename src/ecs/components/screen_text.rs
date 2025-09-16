use bevy_ecs::prelude::Component;
use glam::Vec2;

#[derive(Component)]
pub struct ScreenTextComponent {
    pub text: String,
    pub position: Vec2,
    pub font_size: f32,
}
