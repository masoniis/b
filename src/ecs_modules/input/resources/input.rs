use bevy_ecs::prelude::Resource;
use glam::{DVec2, Vec2};
use std::collections::HashSet;
use winit::keyboard::PhysicalKey;

#[derive(Debug, Resource)]
pub struct InputResource {
    pub previous_keys: HashSet<PhysicalKey>,
    pub current_keys: HashSet<PhysicalKey>,

    pub mouse_delta: DVec2,
    pub scroll_delta: Vec2,
}

impl InputResource {
    pub fn new() -> Self {
        Self {
            previous_keys: HashSet::new(),
            current_keys: HashSet::new(),
            mouse_delta: DVec2::ZERO,
            scroll_delta: Vec2::ZERO,
        }
    }

    /// Was this key just pressed in this frame?
    pub fn was_key_pressed(&self, key_code: PhysicalKey) -> bool {
        self.current_keys.contains(&key_code) && !self.previous_keys.contains(&key_code)
    }

    /// Is this key currently held down? (a key can be held down and "just pressed")
    pub fn is_key_down(&self, key_code: PhysicalKey) -> bool {
        self.current_keys.contains(&key_code)
    }

    /// Was this key just released in this frame?
    pub fn was_key_released(&self, key_code: PhysicalKey) -> bool {
        !self.current_keys.contains(&key_code) && self.previous_keys.contains(&key_code)
    }
}
