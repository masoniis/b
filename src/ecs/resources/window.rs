use bevy_ecs::prelude::Resource;
use winit::dpi::PhysicalSize;

#[derive(Debug, Resource)]
pub struct WindowResource {
    pub width: u32,
    pub height: u32,
}

impl WindowResource {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
