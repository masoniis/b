use crate::{core::graphics::types::vertex::Vertex, prelude::*};
use bevy_ecs::prelude::Resource;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Asset {
    fn name(&self) -> &str;
}

// INFO: --------------------
//        Asset types
// --------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockAppearance {
    pub top_face_texture_index: u32,
    pub bottom_face_texture_index: u32,

    pub front_face_texture_index: u32,
    pub back_face_texture_index: u32,

    pub left_face_texture_index: u32,
    pub right_face_texture_index: u32,
}

// TODO: Add a block def asset storage to the ECS world and use
// it during chunk generation for stuff like adding textures to blocks

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

#[derive(Debug, PartialEq, Eq, Hash)]
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
    name_to_id: HashMap<String, AssetId>,
}

impl<T> Default for AssetStorageResource<T> {
    fn default() -> Self {
        Self {
            storage: HashMap::new(),
            next_id: 0,
            name_to_id: HashMap::new(),
        }
    }
}

impl<T> AssetStorageResource<T> {
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.storage.get(&handle.id)
    }
}

impl<T: Asset> AssetStorageResource<T> {
    /// Adds an asset to the storage, returning a handle to it.
    ///
    /// If an asset with the same name already exists, the new asset is rejected,
    /// and the handle to that name is returned instead (with a warning log).
    pub fn add(&mut self, asset: T) -> Handle<T> {
        let asset_name = asset.name().to_string();

        match self.name_to_id.entry(asset_name) {
            Entry::Vacant(entry) => {
                let id = self.next_id;
                entry.insert(id);
                self.storage.insert(id, asset);
                self.next_id += 1;
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

    pub fn get_by_id(&self, name: &str) -> Option<&T> {
        self.name_to_id
            .get(name)
            .and_then(|id| self.storage.get(id))
    }
}
