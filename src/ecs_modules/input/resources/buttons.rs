use bevy_ecs::prelude::Resource;
use std::collections::hash_set::{HashSet, Iter};
use std::hash::Hash;

#[derive(Debug, Resource)]
pub struct Buttons<T: Copy + Eq + Hash + Send + Sync + 'static> {
    previous: HashSet<T>,
    current: HashSet<T>,
}

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Default for Buttons<T> {
    fn default() -> Self {
        Self {
            previous: HashSet::new(),
            current: HashSet::new(),
        }
    }
}

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Buttons<T> {
    pub fn press(&mut self, button: T) {
        self.current.insert(button);
    }

    pub fn release(&mut self, button: T) {
        self.current.remove(&button);
    }

    pub fn swap_previous(&mut self) {
        self.previous = self.current.clone();
    }

    pub fn was_pressed(&self, button: T) -> bool {
        self.current.contains(&button) && !self.previous.contains(&button)
    }

    pub fn is_down(&self, button: T) -> bool {
        self.current.contains(&button)
    }

    pub fn was_released(&self, button: T) -> bool {
        !self.current.contains(&button) && self.previous.contains(&button)
    }

    pub fn iter_current<'a>(&'a self) -> Iter<'a, T> {
        self.current.iter()
    }

    pub fn iter_previous<'a>(&'a self) -> Iter<'a, T> {
        self.previous.iter()
    }
}
