use glam::{DVec2, Vec2};
use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default, Debug)]
pub struct InputResource {
    pub pressed_keys: HashSet<KeyCode>,
    pub mouse_delta: DVec2,
    pub scroll_delta: Vec2,
}

impl InputResource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_key_pressed(&self, key_code: &KeyCode) -> bool {
        self.pressed_keys.contains(key_code)
    }
}
