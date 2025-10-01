use bevy_ecs::prelude::Resource;
use glam::{DVec2, Vec2};
use std::collections::hash_set::{HashSet, Iter};
use winit::{event::MouseButton, keyboard::PhysicalKey};

#[derive(Debug, Resource, Default)]
pub struct InputResource {
    previous_keys: HashSet<PhysicalKey>,
    current_keys: HashSet<PhysicalKey>,

    previous_mouse_buttons: HashSet<MouseButton>,
    current_mouse_buttons: HashSet<MouseButton>,

    mouse_delta: DVec2,
    scroll_delta: Vec2,
}

impl InputResource {
    // INFO: ---------------------------
    //        State manipulation
    // ---------------------------------

    pub fn key_press(&mut self, key_code: PhysicalKey) {
        self.current_keys.insert(key_code);
    }

    pub fn key_release(&mut self, key_code: PhysicalKey) {
        self.current_keys.remove(&key_code);
    }

    pub fn mouse_press(&mut self, button: MouseButton) {
        self.current_mouse_buttons.insert(button);
    }

    pub fn mouse_release(&mut self, button: MouseButton) {
        self.current_mouse_buttons.remove(&button);
    }

    pub fn adjust_mouse_delta(&mut self, delta: DVec2) {
        self.mouse_delta += delta;
    }

    pub fn adjust_scroll_delta(&mut self, delta: Vec2) {
        self.scroll_delta += delta;
    }

    /// Clones the current state into the previous state
    /// and resets mouse deltas for the new frame.
    pub fn swap_previous_and_reset_deltas(&mut self) {
        self.previous_keys = self.current_keys.clone();
        self.previous_mouse_buttons = self.current_mouse_buttons.clone();
        self.mouse_delta = DVec2::ZERO;
        self.scroll_delta = Vec2::ZERO;
    }

    // INFO: -----------------------
    //        State checking
    // -----------------------------

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

    // INFO: -----------------------
    //        State fetching
    // -----------------------------

    // ========= Getters

    pub fn get_current_keys(&self) -> &HashSet<PhysicalKey> {
        &self.current_keys
    }

    pub fn get_previous_keys(&self) -> &HashSet<PhysicalKey> {
        &self.previous_keys
    }

    pub fn get_current_mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.current_mouse_buttons
    }

    pub fn get_previous_mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.previous_mouse_buttons
    }

    pub fn get_mouse_delta(&self) -> DVec2 {
        self.mouse_delta
    }

    pub fn get_scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    // ========= Iterators

    /// Provides an iterator over all the currently pressed keys.
    pub fn iter_current_keys<'a>(&'a self) -> Iter<'a, PhysicalKey> {
        self.current_keys.iter()
    }

    /// Provides an iterator over all the pressed keys of the previous frame.
    pub fn iter_previous_keys<'a>(&'a self) -> Iter<'a, PhysicalKey> {
        self.previous_keys.iter()
    }

    /// Provides an iterator over all the currently pressed mouse buttons.
    pub fn iter_current_mouse_buttons<'a>(&'a self) -> Iter<'a, MouseButton> {
        self.current_mouse_buttons.iter()
    }

    /// Provides an iterator over all the pressed mouse buttons of the previous frame.
    pub fn iter_previous_mouse_buttons<'a>(&'a self) -> Iter<'a, MouseButton> {
        self.previous_mouse_buttons.iter()
    }
}
