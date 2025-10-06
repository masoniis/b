use crate::render_world::extract::run_extract_schedule::GameWorld;
use bevy_ecs::prelude::*;
use bevy_ecs::query::{QueryData, QueryFilter};

/// A resource in the RenderWorld that will store the extracted component data.
/// The `T` is a marker to make each collection of extracted items a unique resource type.
#[derive(Resource)]
pub struct ExtractedItems<T: ExtractComponent> {
    pub items: Vec<T::Extracted>,
}

impl<T: ExtractComponent> Default for ExtractedItems<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

/// A trait that defines how to extract data from a set of components in the GameWorld.
pub trait ExtractComponent: Send + Sync + 'static {
    /// The final data structure that will be stored in the RenderWorld.
    type Extracted: Send + Sync + 'static;

    /// The tuple of components to query for in the GameWorld.
    /// Example: `(&'static Transform, &'static Handle<Mesh>)`
    type QueryComponents: QueryData;

    /// A filter to apply to the query.
    /// Example: `With<Visible>`
    type QueryFilter: QueryFilter;

    /// The function that maps the queried components to the final extracted data structure.
    fn extract(
        entity: Entity,
        components: <<Self::QueryComponents as QueryData>::ReadOnly as QueryData>::Item<'_>,
    ) -> Self::Extracted;
}

/// A generic system that extracts components from the GameWorld and stores them in the RenderWorld.
pub fn extract_component_system<T: ExtractComponent>(
    mut main_world: ResMut<GameWorld>,
    mut extracted: ResMut<ExtractedItems<T>>,
) {
    // Clear the vector from the previous frame's data.
    extracted.items.clear();

    let mut query = main_world
        .val
        .query_filtered::<(Entity, T::QueryComponents), T::QueryFilter>();

    for (entity, components) in query.iter(&main_world.val) {
        let extracted_item = T::extract(entity, components);
        extracted.items.push(extracted_item);
    }
}
