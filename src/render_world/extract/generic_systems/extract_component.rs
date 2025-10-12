use crate::render_world::extract::run_extract_schedule::GameWorld;
use bevy_ecs::prelude::*;
use bevy_ecs::query::{QueryData, QueryFilter};

/// A resource in the RenderWorld that will store the extracted component data.
/// The `T` is a marker to make each collection of extracted items a unique resource type.
#[derive(Resource)]
pub struct ExtractedBy<T: ExtractComponent> {
    pub items: Vec<T::Extracted>,
}

impl<T: ExtractComponent> Default for ExtractedBy<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

/// A trait that defines how to extract data from a set of components in the GameWorld.
pub trait ExtractComponent: Send + Sync + 'static {
    /// The final data structure that will be stored in the RenderWorld.
    type Extracted: Send + Sync + 'static + ContainsEntity;

    /// The tuple of components to query for in the GameWorld.
    /// Example: `(&'static Transform, &'static Handle<Mesh>)`
    type QueryComponents: QueryData;

    /// A filter to apply to the query.
    /// Example: `With<Visible>`
    type QueryFilter: QueryFilter;

    /// A filter to determine if the component has changed.
    /// Example: `Changed<Transform>`
    type ChangeTracked: QueryFilter;

    /// The function that maps the queried components to the final extracted data structure.
    fn extract(
        entity: Entity,
        components: <<Self::QueryComponents as QueryData>::ReadOnly as QueryData>::Item<'_>,
    ) -> Self::Extracted;

    /// A stable key for the entity, used for change detection.
    fn entity_key(extracted: &Self::Extracted) -> Entity {
        extracted.entity()
    }
}

/// A generic system that extracts components from the GameWorld and stores them in the RenderWorld.
pub fn extract_component_system<T: ExtractComponent>(
    mut main_world: ResMut<GameWorld>,
    mut extracted: ResMut<ExtractedBy<T>>,
) {
    // Query only changed components
    let mut query = main_world
        .val
        .query_filtered::<(Entity, T::QueryComponents), (T::QueryFilter, T::ChangeTracked)>();

    // Update existing items by entity lookup
    for (entity, components) in query.iter(&main_world.val) {
        let extracted_item = T::extract(entity, components);

        // Find and update existing, or push new
        if let Some(existing) = extracted
            .items
            .iter_mut()
            .find(|item| T::entity_key(item) == entity)
        {
            *existing = extracted_item;
        } else {
            extracted.items.push(extracted_item);
        }
    }

    // Handle removals separately if needed
}
