use crate::core::graphics::types::vertex::Vertex;
use bevy_ecs::prelude::Resource;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

// INFO: --------------------
//        Asset types
// --------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockAppearance {
    pub top_face_texture_index: u32,
    pub bottom_face_texture_index: u32,
    pub side_face_texture_index: u32,
}

#[derive(Debug, Clone)]
pub struct BlockDefinition {
    pub name: String,
    pub appearance: BlockAppearance,
    pub is_transparent: bool,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MeshAsset {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TextureAsset {
    pub dimensions: (u32, u32),
    pub bytes: Vec<u8>, // The raw pixel data (e.g., RGBA8)
}

// INFO: ---------------------------
//        The storage itself
// ---------------------------------

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

// Clone for handle just copies the id
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

// "marker" trait, so its implementation block is empty.
impl<T> Copy for Handle<T> {}

pub type AssetId = u32; // Small abstraction for ID that can easily be hotswapped

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
