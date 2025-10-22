use crate::prelude::*;
use bevy_ecs::prelude::Resource;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, RwLock,
};

pub type AssetId = u32;
pub trait Asset {
    fn name(&self) -> &str;
}

/// A handle to an asset stored in the `AssetStorageResource`.
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

// INFO: -------------------------
//      The storage itself
// -------------------------------

/// A thread-safe, reference-counted asset storage resource.
/// Cloning this resource is cheap and allows it to be shared across threads.
#[derive(Resource, Clone)]
pub struct AssetStorageResource<T> {
    storage: Arc<RwLock<HashMap<AssetId, T>>>,
    next_id: Arc<AtomicU32>,
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
                debug!(
                    target : "duplicate_assets",
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

    /// Removes an asset from the storage using its handle.
    /// Acquires write locks. Returns the removed asset if it existed.
    pub fn remove(&self, handle: Handle<T>) -> Option<T> {
        // get the name for logging
        let name = {
            let storage_read = self.storage.read().unwrap();
            storage_read
                .get(&handle.id())
                .map(|asset| asset.name().to_string())
        };

        // remove the asset from storage
        let removed_asset = {
            let mut storage_write = self.storage.write().unwrap();
            storage_write.remove(&handle.id())
        };

        // log and clean up the name map
        if let Some(asset_name) = name {
            if removed_asset.is_some() {
                let mut name_write = self.name_to_id.write().unwrap();
                name_write.remove(&asset_name);
                info!("Removed asset '{}' (ID: {})", asset_name, handle.id());
            }
        } else if removed_asset.is_some() {
            warn!(
                "Removed asset ID {} but couldn't find its name in the name map!",
                handle.id()
            );
        }

        removed_asset
    }
}
