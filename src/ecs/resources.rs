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

pub struct DeltaTime(pub f32);

impl Default for DeltaTime {
    fn default() -> Self {
        Self(0.0)
    }
}
