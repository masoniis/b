use crate::{prelude::*, render_world::extract::utils::run_extract_schedule::GameWorld};
use bevy_ecs::prelude::*;

/// A generic system that clones a resource of type `T` from the main world
/// and inserts it into the render world.
///
/// Typically we only want to clone resources that are tiny or clone "smartly"
/// such as Arc.
pub fn clone_resource_system<T: Resource + Clone>(
    mut commands: Commands,
    main_world: Res<GameWorld>,
) {
    if let Some(resource_to_clone) = main_world.val.get_resource::<T>() {
        commands.insert_resource(resource_to_clone.clone());
    } else {
        warn!(
            "Resource of type {} not found in main world; cannot clone to render world.",
            std::any::type_name::<T>()
        );
    }
}
