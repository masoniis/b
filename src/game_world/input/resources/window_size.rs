use bevy_ecs::prelude::Resource;
use winit::dpi::PhysicalSize;

/// WindowSize is considered an Input resource as it is received
/// from the OS directly, and the user can adjust it as well.
#[derive(Debug, Resource)]
pub struct WindowSizeResource {
    pub width: u32,
    pub height: u32,
}

impl WindowSizeResource {
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
