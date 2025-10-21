use crate::prelude::*;
use crate::{
    render_world::types::Vertex,
    simulation_world::asset_management::{Asset, Handle},
};
use bevy_ecs::message::Message;
use bevy_ecs::prelude::Resource;
use std::collections::{hash_map::Entry, HashMap};

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

/// A message indicating that a mesh has been removed and no longer exists.
#[derive(Message)]
pub struct MeshRemovedMessage {
    pub mesh_handle: Handle<MeshAsset>,
}

/// A resource that tracks reference counts for mesh assets. Used to determine
/// when to remove meshes that are no longer in use.
#[derive(Resource, Default, Debug)]
pub struct MeshRefCounts {
    counts: HashMap<Handle<MeshAsset>, u32>,
}

impl MeshRefCounts {
    /// Increments the count for the given handle and returns the new count
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
