use bevy_ecs::prelude::Resource;

#[derive(Debug, Resource)]
pub struct WindowResource {
    pub width: u32,
    pub height: u32,
}

impl WindowResource {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

impl Default for WindowResource {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}
