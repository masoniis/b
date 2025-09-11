use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct Input {
    pub pressed_keys: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }
}
