use crate::prelude::*;
use crate::render_world::global_extract::utils::run_extract_schedule::SimulationWorld;
use bevy_ecs::prelude::*;
use bevy_ecs::query::{QueryData, QueryFilter};
use std::collections::HashSet;

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

/// A trait that defines how to extract data from a set of components in the SimulationWorld.
pub trait ExtractComponent: Send + Sync + 'static {
    /// The final data structure that will be stored in the RenderWorld.
    type Extracted: Send + Sync + 'static + ContainsEntity;

    /// The tuple of components to query for in the SimulationWorld.
    /// Example: `(&'static Transform, &'static Handle<Mesh>)`
    type QueryComponents: QueryData;

    /// A filter to apply to the query.
    /// Example: `With<Visible>`
    type QueryFilter: QueryFilter;

    /// A filter to determine if the component has changed.
    /// Example: `Changed<Transform>`
    type ChangeTracked: QueryFilter;

    /// The component whose removal signals that this extracted item is stale.
    type RemovalMarker: Component;

    /// The function that maps the queried components to the final extracted data structure.
    fn extract(
        entity: Entity,
        components: <<Self::QueryComponents as QueryData>::ReadOnly as QueryData>::Item<'_, '_>,
    ) -> Self::Extracted;

    /// A stable key for the entity, used for change detection.
    fn entity_key(extracted: &Self::Extracted) -> Entity {
        extracted.entity()
    }
}

/// A generic system that extracts components from the SimulationWorld and stores them in the RenderWorld.
#[instrument(skip_all)]
pub fn extract_component_system<T: ExtractComponent>(
    mut simulation_world: ResMut<SimulationWorld>,
    mut extracted: ResMut<ExtractedBy<T>>,
) {
    // Handle removals
    let removal_marker_id = simulation_world
        .val
        .component_id::<T::RemovalMarker>()
        .unwrap();

    let removed_entities: HashSet<Entity> = simulation_world
        .val
        .removed_with_id(removal_marker_id)
        .collect();

    if !removed_entities.is_empty() {
        info!(
            "ExtractComponent: Removed {} entities for {}",
            removed_entities.len(),
            std::any::type_name::<T>()
        );

        extracted.items.retain(|item| {
            let entity_key = T::entity_key(item);
            !removed_entities.contains(&entity_key)
        });
    }

    // Query for entities with changed components that still exist.
    let mut query = simulation_world
        .val
        .query_filtered::<(Entity, T::QueryComponents), (T::QueryFilter, T::ChangeTracked)>();

    for (entity, components) in query.iter(&simulation_world.val) {
        let extracted_item = T::extract(entity, components);

        // Find and update an existing item, or push a new one.
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
}
