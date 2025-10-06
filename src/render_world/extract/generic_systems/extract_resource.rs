use crate::prelude::*;
use crate::render_world::extract::utils::run_extract_schedule::GameWorld;
use bevy_ecs::prelude::*;
use bevy_ecs::resource::Resource;
use std::marker::PhantomData;

/// A trait for a resource that can be extracted from the game world into the render world.
///
/// The `Source` is the resource that exists in the game world.
/// The `Output` is the resource that will be created in the render world.
pub trait ExtractResource {
    type Source: Resource;
    type Output: Resource;

    /// Extracts the resource from the main world and returns the render world version.
    fn extract_resource(source: &Self::Source) -> Self::Output;
}

/// A generic system that extracts all resources implementing the `ExtractResource` trait.
pub fn extract_resource_system<T: ExtractResource>(
    mut commands: Commands,
    main_world: Res<GameWorld>,
    _phantom: PhantomData<T>, // needed to make this a unique system for each type T
) {
    if let Some(source_resource) = main_world.val.get_resource::<T::Source>() {
        let extracted = T::extract_resource(source_resource);
        commands.insert_resource(extracted);
    } else {
        warn!(
            "Source resource of type {} not found in main world. This system likely was placed incorrectly.",
            std::any::type_name::<T::Source>()
        );
    }
}
