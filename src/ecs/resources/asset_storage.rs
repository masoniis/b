use bevy_ecs::prelude::Resource;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::graphics::Vertex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MeshAsset {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

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

// Here is the manual implementation of the `Clone` trait.
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        // As you can see, cloning a Handle just copies its ID.
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

// It is a "marker" trait, so its implementation block is empty.
impl<T> Copy for Handle<T> {}

pub type AssetId = u64; // Small abstraction for ID that can easily be hotswapped

#[derive(Resource)]
pub struct AssetStorageResource<T> {
    storage: HashMap<AssetId, T>,
    next_id: AssetId,
}

impl<T> Default for AssetStorageResource<T> {
    fn default() -> Self {
        Self {
            storage: HashMap::new(),
            next_id: 0,
        }
    }
}

impl<T> AssetStorageResource<T> {
    pub fn add(&mut self, asset: T) -> Handle<T> {
        let id = self.next_id;
        self.storage.insert(id, asset);
        self.next_id += 1;
        Handle::new(id)
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.storage.get(&handle.id)
    }
}
