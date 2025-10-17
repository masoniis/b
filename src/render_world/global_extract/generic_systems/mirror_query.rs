use crate::prelude::*;
use crate::render_world::global_extract::utils::run_extract_schedule::SimulationWorld;
use crate::render_world::scheduling::RenderSchedule;
use crate::{EcsBuilder, Plugin};
use bevy_ecs::prelude::*;
use bevy_ecs::query::{QueryData, QueryFilter};
use std::collections::HashMap;
use std::marker::PhantomData;

// INFO: -------------------------------------------
//         Trait for components to implement
// -------------------------------------------------

/// A trait for components in the SimulationWorld that should be mirrored to the RenderWorld.
pub trait MirrorableComponent: Component {
    /// Other components from the SimulationWorld entity that are needed to create the RenderBundle.
    type Dependencies: QueryData;

    /// The bundle of components to spawn in the RenderWorld.
    type RenderBundle: Bundle;

    /// An arbitrary query to run across the SimulationWorld to find entities needing extraction.
    type Filter: QueryFilter;

    /// Creates the RenderBundle from the SimulationWorld components.
    fn to_render_bundle(
        &self,
        dependencies: <<<Self as MirrorableComponent>::Dependencies as QueryData>::ReadOnly as QueryData>::Item<'_>,
    ) -> Self::RenderBundle;
}

// INFO: ------------------------
//         Output resource
// ------------------------------

/// A generic resource that maps a main world component type `T` to its render world entity.
#[derive(Resource)]
pub struct EntityMap<T: MirrorableComponent>(pub HashMap<Entity, Entity>, PhantomData<T>);

impl<T: MirrorableComponent> Default for EntityMap<T> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

// INFO: ------------------------------
//         Plugin to set all up
// ------------------------------------

/// A generic plugin that sets up the extraction for any component implementing `MirrorableComponent`.
pub struct ExtractComponentPlugin<T: MirrorableComponent>(pub PhantomData<T>);

impl<T: MirrorableComponent> Default for ExtractComponentPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: MirrorableComponent> Plugin for ExtractComponentPlugin<T> {
    fn build(&self, builder: &mut EcsBuilder) {
        // Register the entity map resource for this component type
        builder.add_resource(EntityMap::<T>::default());

        // Add the generic extraction system to the 'Extract' schedule
        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems(extract_mirrorable_components_system::<T>);
    }
}

/// Generic system that performs the stateful, mirroring extraction for a component.
#[instrument(skip_all)]
fn extract_mirrorable_components_system<T: MirrorableComponent>(
    mut commands: Commands,
    mut entity_map: ResMut<EntityMap<T>>,
    mut simulation_world: ResMut<SimulationWorld>,
    mut removed_components: RemovedComponents<T>,
) {
    // Despawn entities when the MirrorableComponent is removed

    // FIXME? Entities won't be despawned when a dependency is removed,
    // only when the main part is removed. This could lead to bugs down the line
    // but I don't have any good solutions that don't require iterating over all the
    // mirrored components and that is pretty expensive when it comes to meshes.
    for main_entity in removed_components.read() {
        if let Some(render_entity) = entity_map.0.remove(&main_entity) {
            // Use `get_entity` and `despawn` for safe despawning
            if let Ok(mut entity_commands) = commands.get_entity(render_entity) {
                entity_commands.despawn();
            } else {
                warn!(
                    "Render entity {:?} not found for despawning (main entity {:?})",
                    render_entity, main_entity
                );
            }
        }
    }

    let mut query = simulation_world
        .val
        .query_filtered::<(Entity, &T, T::Dependencies), T::Filter>();

    for (main_entity, main_component, dependencies) in query.iter(&simulation_world.val) {
        let render_bundle = main_component.to_render_bundle(dependencies);

        // Decide whether to spawn a new entity or update an existing one.
        if let Some(&render_entity) = entity_map.0.get(&main_entity) {
            // UPDATE: The entity already exists, so just update its bundle of components.
            if let Ok(mut entity_commands) = commands.get_entity(render_entity) {
                entity_commands.insert(render_bundle);
            }
        } else {
            // SPAWN: This is a new entity. Spawn it in the render world and map it.
            let render_entity = commands.spawn(render_bundle).id();
            entity_map.0.insert(main_entity, render_entity);
        }
    }
}
