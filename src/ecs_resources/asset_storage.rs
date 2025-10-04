use crate::{core::graphics::types::vertex::Vertex, prelude::*};
use bevy_ecs::prelude::Resource;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, RwLock,
};

pub trait Asset {
    fn name(&self) -> &str;
}

// --- Asset types (Unchanged) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockAppearance {
    pub top_face_texture_index: u32,
    pub bottom_face_texture_index: u32,
    pub front_face_texture_index: u32,
    pub back_face_texture_index: u32,
    pub left_face_texture_index: u32,
    pub right_face_texture_index: u32,
}
#[derive(Debug, Clone)]
pub struct BlockDefAsset {
    pub name: String,
    pub appearance: BlockAppearance,
    pub is_transparent: bool,
}
impl Asset for BlockDefAsset {
    fn name(&self) -> &str {
        &self.name
    }
}
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

// --- Handle (Unchanged) ---
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    id: AssetId,
    _phantom: PhantomData<T>,
}
impl<T> Handle<T> {
    pub fn new(id: AssetId) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    pub fn id(&self) -> AssetId {
        self.id
    }
}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}
impl<T> Copy for Handle<T> {}
pub type AssetId = u32;

// INFO: ---------------------------
//      The storage itself (Refactored for Thread Safety)
// ---------------------------------

/// A thread-safe, reference-counted asset storage resource.
/// Cloning this resource is cheap and allows it to be shared across threads.
#[derive(Resource, Clone)]
pub struct AssetStorageResource<T> {
    storage: Arc<RwLock<HashMap<AssetId, T>>>,
    next_id: Arc<AtomicU32>, // Atomic for lock-free increments
    name_to_id: Arc<RwLock<HashMap<String, AssetId>>>,
}

impl<T> Default for AssetStorageResource<T> {
    fn default() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicU32::new(0)),
            name_to_id: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<T> AssetStorageResource<T> {
    /// Gets a cloned copy of an asset.
    ///
    /// This is a convenience method for small assets that are cheap to `Clone`.
    /// For large, non-cloneable assets like `MeshAsset`, use the `.with()` method.
    pub fn get(&self, handle: Handle<T>) -> Option<T>
    where
        T: Clone,
    {
        let storage = self.storage.read().unwrap();
        storage.get(&handle.id).cloned()
    }

    /// Executes a closure with an immutable reference to an asset's data.
    ///
    /// This is the primary way to safely access data for any asset. The provided
    /// closure `f` is executed while a read lock is held on the asset storage,
    /// ensuring safe access without needing to clone the asset.
    pub fn with<F, R>(&self, handle: Handle<T>, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        let storage = self.storage.read().unwrap();
        storage.get(&handle.id).map(f)
    }
}

impl<T: Asset + Send + Sync + 'static> AssetStorageResource<T> {
    /// Adds an asset to the storage, returning a handle to it.
    /// This will acquire a write lock. Only one thread can add at a time.
    pub fn add(&self, asset: T) -> Handle<T> {
        let asset_name = asset.name().to_string();

        // Acquire a write lock on the name map. This is a short lock.
        let mut name_to_id = self.name_to_id.write().unwrap();
        match name_to_id.entry(asset_name) {
            Entry::Vacant(entry) => {
                // Get a new, unique ID atomically.
                let id = self.next_id.fetch_add(1, Ordering::Relaxed);
                entry.insert(id);

                // Now, acquire a write lock on the main storage to insert the asset.
                let mut storage = self.storage.write().unwrap();
                storage.insert(id, asset);
                Handle::new(id)
            }
            Entry::Occupied(entry) => {
                let existing_id = *entry.get();
                warn!(
                    "Attempted to add a duplicate asset with name: '{}'. \
                    The new asset was rejected. Returning a handle to the existing asset (ID: {}).",
                    entry.key(),
                    existing_id
                );
                Handle::new(existing_id)
            }
        }
    }

    pub fn get_by_name(&self, name: &str) -> Option<Handle<T>> {
        let name_to_id = self.name_to_id.read().unwrap();
        name_to_id.get(name).map(|id| Handle::new(*id))
    }
}
