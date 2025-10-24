use crate::prelude::*;
use crate::simulation_world::asset_management::AssetStorageResource;
use crate::simulation_world::chunk::MeshComponent;
use crate::{
    render_world::types::Vertex,
    simulation_world::asset_management::{Asset, Handle},
};
use bevy_ecs::prelude::*;
use std::collections::{hash_map::Entry, HashMap};

// INFO: -----------------------------
//         Types and resources
// -----------------------------------

/// A 3D mesh asset consisting of vertices and indices.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshAsset {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
impl Asset for MeshAsset {
    fn name(&self) -> &str {
        &self.name
    }
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
//         Update system
// -----------------------------

/// A message requesting deletion of a mesh asset from the asset storage.
#[derive(Message)]
pub struct MeshDeletionRequest {
    pub mesh_handle: Handle<MeshAsset>,
}

/// Observer that increments mesh ref-counts when a component is added.
#[instrument(skip_all)]
pub fn mesh_ref_count_add_observer(
    trigger: On<Add, MeshComponent>,

    // Input
    mesh_query: Query<&MeshComponent>,

    // Output (update ref counts)
    mut mesh_ref_counts: ResMut<MeshRefCounts>,
) {
    if let Ok(mesh_component) = mesh_query.get(trigger.entity) {
        let handle = mesh_component.mesh_handle;
        mesh_ref_counts.increment(handle);
    }
}

/// Observer that decrements mesh ref-counts when a component is removed.
#[instrument(skip_all)]
pub fn mesh_ref_count_remove_observer(
    trigger: On<Remove, MeshComponent>,

    // Input
    mesh_query: Query<&MeshComponent>,

    // Output (update ref counts and request deletions)
    mut mesh_ref_counts: ResMut<MeshRefCounts>,
    mut stale_mesh_writer: MessageWriter<MeshDeletionRequest>,
) {
    if let Ok(mesh_component) = mesh_query.get(trigger.entity) {
        let handle = mesh_component.mesh_handle;

        if let Some(new_count) = mesh_ref_counts.decrement(handle) {
            // send deletion request if count is zero
            if new_count == 0 {
                debug!(
                    target: "asset_management",
                    "Ref count for mesh {:?} is zero. Sending deletion request.",
                    handle.id()
                );
                stale_mesh_writer.write(MeshDeletionRequest {
                    mesh_handle: handle,
                });
            }
        }
    }
}

/// A system that reads RemovedMesh events and deletes any mesh assets.
pub fn delete_stale_mesh_assets(
    asset_storage: Res<AssetStorageResource<MeshAsset>>,
    mut event_reader: MessageReader<MeshDeletionRequest>,
) {
    for event in event_reader.read() {
        let handle = event.mesh_handle;

        if asset_storage.remove(handle).is_none() {
            error!(
                asset_id = handle.id(),
                "Attempted to remove mesh asset that does not exist in storage."
            );
        }
    }
}
