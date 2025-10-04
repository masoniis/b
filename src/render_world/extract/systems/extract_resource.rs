use crate::render_world::extract::utils::{run_extract_schedule::GameWorld, ExtractResource};
use bevy_ecs::prelude::*;
use std::marker::PhantomData;

/// A generic system that extracts all resources implementing the `ExtractResource` trait.
pub fn extract_resource_system<T: ExtractResource>(
    mut commands: Commands,
    main_world: Res<GameWorld>,
    _phantom: PhantomData<T>, // Marker to make this a unique system for each type T
) {
    if let Some(source_resource) = main_world.val.get_resource::<T::Source>() {
        let extracted = T::extract_resource(source_resource);
        commands.insert_resource(extracted);
    }
}
