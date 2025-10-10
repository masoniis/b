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

    /// Extracts the resource from the main world, compares it to the target in the
    /// render world, and inserts or updates it only if necessary.
    fn extract_and_update(
        commands: &mut Commands,
        source: &Self::Source,
        target: Option<ResMut<Self::Output>>,
    );
}

/// A generic system that extracts resources using the `ExtractResource` trait.
/// It delegates the update logic to the trait's implementation.
pub fn extract_resource_system<T: ExtractResource>(
    mut commands: Commands,
    main_world: Res<GameWorld>,
    target: Option<ResMut<T::Output>>,
    _phantom: PhantomData<T>,
) {
    if let Some(source_resource) = main_world.val.get_resource::<T::Source>() {
        T::extract_and_update(&mut commands, source_resource, target);
    } else {
        warn!(
            "Source resource of type {} not found in main world.",
            std::any::type_name::<T::Source>()
        );
    }
}
