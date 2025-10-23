use bevy_ecs::{entity::Entity, prelude::Resource};

/// A resource that holds the currently active camera entity (regarding rendering)
#[derive(Resource)]
pub struct ActiveCamera(pub Entity);
