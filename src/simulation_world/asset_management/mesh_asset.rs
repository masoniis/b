use crate::ecs_core::SimToRenderSender;
use crate::prelude::*;
use crate::{
    render_world::types::PackedFace,
    simulation_world::{
        asset_management::AssetStorageResource,
        asset_management::{Asset, Handle},
        chunk::{OpaqueMeshComponent, TransparentMeshComponent},
    },
};
use bevy_ecs::prelude::*;
use std::collections::{hash_map::Entry, HashMap};

// INFO: -----------------------------
//         types and resources
// -----------------------------------

/// A 3D mesh asset consisting of vertices and indices.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshAsset {
    pub name: String,
    pub faces: Vec<PackedFace>,
}
impl Asset for MeshAsset {
    fn name(&self) -> &str {
        &self.name
    }
}

/// Tracks the last-known mesh handle for each entity (Opaque).
#[derive(Resource, Default, Debug)]
pub struct OpaqueMeshShadow {
    pub entity_to_handle: HashMap<Entity, Handle<MeshAsset>>,
}

/// Tracks the last-known mesh handle for each entity (Transparent).
#[derive(Resource, Default, Debug)]
pub struct TransparentMeshShadow {
    pub entity_to_handle: HashMap<Entity, Handle<MeshAsset>>,
}

/// A resource that tracks reference counts for mesh assets. Used to determine
/// when to remove meshes from asset storage that are no longer in use.
#[derive(Resource, Default, Debug)]
pub struct MeshRefCounts {
    counts: HashMap<Handle<MeshAsset>, u32>,
}

impl MeshRefCounts {
    /// Increments the count for the given handle and returns the new count
    ///
    /// If the handle is not already tracked, it is added with an initial count of 1.
    pub fn increment(&mut self, handle: Handle<MeshAsset>) -> u32 {
        let count = self.counts.entry(handle).or_insert(0);
        *count += 1;
        *count
    }

    /// Returns the new count, or None if the handle wasn't tracked
    pub fn decrement(&mut self, handle: Handle<MeshAsset>) -> Option<u32> {
        match self.counts.entry(handle) {
            Entry::Occupied(mut entry) => {
                let count = entry.get_mut();
                *count = count.saturating_sub(1); // prevent underflow
                let current_count = *count;
                if current_count == 0 {
                    entry.remove();
                }
                Some(current_count)
            }
            Entry::Vacant(_) => {
                warn!(
                    "Decremented ref count for untracked mesh handle: {:?}",
                    handle.id()
                );
                None
            }
        }
    }
}

// INFO: -----------------------
//         update system
// -----------------------------

/// A message requesting deletion of a mesh asset from the asset storage.
#[derive(Message, Clone)]
pub struct MeshDeletionRequest {
    pub mesh_handle: Handle<MeshAsset>,
}

/// Observer that increments mesh ref-counts when a component is added.
#[instrument(skip_all)]
pub fn opaque_mesh_added_observer(
    trigger: On<Add, OpaqueMeshComponent>,

    // Input
    mesh_query: Query<&OpaqueMeshComponent>,

    // Output (update ref counts)
    mut mesh_ref_counts: ResMut<MeshRefCounts>,
    mut shadow: ResMut<OpaqueMeshShadow>,
) {
    if let Ok(mesh_component) = mesh_query.get(trigger.entity) {
        let handle = mesh_component.mesh_handle;
        mesh_ref_counts.increment(handle);
        shadow.entity_to_handle.insert(trigger.entity, handle);
    }
}

/// Observer that decrements mesh ref-counts when a component is removed.
#[instrument(skip_all)]
pub fn opaque_mesh_removed_observer(
    trigger: On<Remove, OpaqueMeshComponent>,

    // Input
    mut shadow: ResMut<OpaqueMeshShadow>,

    // Output
    mut mesh_ref_counts: ResMut<MeshRefCounts>,
    mut stale_mesh_writer: MessageWriter<MeshDeletionRequest>,
) {
    let entity = trigger.entity;

    if let Some(handle) = shadow.entity_to_handle.remove(&entity) {
        if let Some(new_count) = mesh_ref_counts.decrement(handle) {
            if new_count == 0 {
                stale_mesh_writer.write(MeshDeletionRequest {
                    mesh_handle: handle,
                });
            }
        }
    } else {
        warn!(
            "Opaque mesh removed from {:?}, but no shadow handle found.",
            entity
        );
    }
}

#[instrument(skip_all)]
pub fn transparent_mesh_added_observer(
    trigger: On<Add, TransparentMeshComponent>,

    // Input
    mesh_query: Query<&TransparentMeshComponent>,

    // Output (update ref counts)
    mut mesh_ref_counts: ResMut<MeshRefCounts>,
    mut shadow: ResMut<TransparentMeshShadow>,
) {
    if let Ok(mesh_component) = mesh_query.get(trigger.entity) {
        let handle = mesh_component.mesh_handle;
        mesh_ref_counts.increment(handle);
        shadow.entity_to_handle.insert(trigger.entity, handle);
    }
}

#[instrument(skip_all)]
pub fn transparent_mesh_removed_observer(
    trigger: On<Remove, TransparentMeshComponent>,

    // Input
    mut shadow: ResMut<TransparentMeshShadow>,

    // Output
    mut mesh_ref_counts: ResMut<MeshRefCounts>,
    mut stale_mesh_writer: MessageWriter<MeshDeletionRequest>,
) {
    let entity = trigger.entity;

    if let Some(handle) = shadow.entity_to_handle.remove(&entity) {
        if let Some(new_count) = mesh_ref_counts.decrement(handle) {
            if new_count == 0 {
                debug!(target: "asset_management", "Ref count zero for Transparent mesh {:?}. Deleting.", handle.id());
                stale_mesh_writer.write(MeshDeletionRequest {
                    mesh_handle: handle,
                });
            }
        }
    } else {
        warn!(
            "TransparentMeshComponent removed from {:?}, but no shadow handle found.",
            entity
        );
    }
}

/// A system that reads RemovedMesh events and deletes any mesh assets.
pub fn delete_stale_mesh_assets(
    asset_storage: Res<AssetStorageResource<MeshAsset>>,
    sender: Res<SimToRenderSender>,
    mut event_reader: MessageReader<MeshDeletionRequest>,
) {
    for event in event_reader.read() {
        let handle = event.mesh_handle;

        match sender.0.send(event.clone()) {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "MEMORY LEAK SOURCE: Render World channel disconnected while sending deletion request: {:?}",
                    e
                );
            }
        }

        if asset_storage.remove(handle).is_none() {
            error!(
                asset_id = handle.id(),
                "Attempted to remove mesh asset that does not exist in storage."
            );
        } else {
            debug!(
                target: "asset_management",
                asset_id = handle.id(),
                "Sim: Removed asset from CPU storage."
            );
        }
    }
}
