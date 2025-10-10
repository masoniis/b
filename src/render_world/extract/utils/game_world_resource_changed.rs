use crate::prelude::*;
use crate::render_world::extract::utils::run_extract_schedule::GameWorld;
use bevy_ecs::prelude::*; // Your path might differ

/// A custom run condition that returns `true` if a resource of type `T` has been added
/// or changed in the **game world** since the last time this condition was checked.
pub fn game_world_resource_changed<T: Resource>(game_world: Res<GameWorld>) -> bool {
    let world = &game_world.val;

    // info!(
    //     "Resource of type {} changed: {}",
    //     std::any::type_name::<T>(),
    //     world.is_resource_changed::<T>()
    // );

    // Check if the resource exists and if its "changed" flag is set.
    world.is_resource_changed::<T>()
}
