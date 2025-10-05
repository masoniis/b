use crate::game_world::global_resources::asset_storage::{Handle, MeshAsset};
use crate::game_world::graphics_old::components::{
    mesh::MeshComponent, transform::TransformComponent,
};
use crate::prelude::*;
use crate::render_world::extract::utils::run_extract_schedule::GameWorld;
use bevy_ecs::prelude::*;
use std::collections::HashMap;

// --- Components for the Render World ---

/// A component in the render world holding the extracted mesh handle.
#[derive(Component, Clone)]
pub struct RenderMeshComponent {
    pub mesh_handle: Handle<MeshAsset>,
}

/// A component in the render world holding the extracted transform.
#[derive(Component, Clone)]
pub struct RenderTransformComponent {
    pub transform: Mat4,
}

// --- Entity Mapping Resource ---

/// A resource that maps entities from the main world to the render world.
#[derive(Resource, Default)]
pub struct MeshEntityMap(pub HashMap<Entity, Entity>);

// --- The (Corrected) Extract System ---

/// Extracts meshes and transforms from the main world into the render world.
///
/// This system's ONLY job is to quickly replicate the state of mesh-related
/// components from the main world to the render world. It performs NO GPU operations.
pub fn extract_meshes_system(
    mut commands: Commands,
    mut entity_map: ResMut<MeshEntityMap>,
    mut main_world: ResMut<GameWorld>,
    // We also need to handle entities that were removed in the main world.
    // removed_mesh_components: Res<RemovedComponents<MeshComponent>>,
) {
    // 1. Handle removals: Despawn render entities corresponding to main world entities
    //    that had their MeshComponent removed.
    // for main_entity in removed_mesh_components.iter() {
    //     if let Some(render_entity) = entity_map.0.remove(&main_entity) {
    //         commands.entity(render_entity).despawn();
    //     }
    // }

    // 2. Query for added or changed meshes in the main world.
    let mut query = main_world.val.query_filtered::<(Entity, &MeshComponent, &TransformComponent), Or<(
        Added<MeshComponent>,
        Changed<TransformComponent>,
    )>>();

    // We collect commands to apply them all at once, which is more efficient.
    let mut commands_to_apply = Vec::new();

    for (main_entity, mesh, transform) in query.iter(&main_world.val) {
        let render_components = (
            RenderMeshComponent {
                mesh_handle: mesh.mesh_handle,
            },
            RenderTransformComponent {
                transform: transform.to_matrix(),
            },
        );

        // 3. Decide whether to update an existing entity or spawn a new one.
        if let Some(&render_entity) = entity_map.0.get(&main_entity) {
            // UPDATE: This entity already exists in the render world, so just update its components.
            commands_to_apply.push((render_entity, render_components));
        } else {
            // SPAWN: This is a new entity. Spawn it in the render world and map it.
            let render_entity = commands.spawn(render_components).id();
            entity_map.0.insert(main_entity, render_entity);
        }
    }

    // Apply the updates for existing entities.
    for (render_entity, components) in commands_to_apply {
        commands.entity(render_entity).insert(components);
    }
}
