use crate::render_world::types::TextureId;
use crate::simulation_world::block::property_loading::{BlockFaceTextures, BlockProperties};
use crate::simulation_world::chunk::block::Block;
use crate::{prelude::*, simulation_world::block::load_block_from_str};
use bevy_ecs::prelude::Resource;
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::fs;

#[derive(Resource, Default)]
pub struct BlockRegistryResource {
    /// Stores properties indexed by the runtime `u16` ID.
    pub block_properties: Vec<BlockProperties>,

    /// Maps a string name (e.g., "grass") to the runtime `u16` ID.
    name_to_id: HashMap<String, u16>,
}

impl BlockRegistryResource {
    /// Gets the properties for a given block ID.
    ///
    /// Always returns a valid property (defaults to Air).
    pub fn get(&self, id: u16) -> &BlockProperties {
        self.block_properties
            .get(id as usize)
            .unwrap_or(&self.block_properties[0]) // air is 0
    }

    /// (Internal) Registers a block, assigning it a new ID.
    fn register(&mut self, name: String, properties: BlockProperties) -> u16 {
        let id = self.block_properties.len() as u16;
        self.block_properties.push(properties);
        self.name_to_id.insert(name.to_lowercase(), id);
        id
    }

    /// Gets the numeric ID for a given block name.
    /// Used during world-gen or setup.
    pub fn get_id_by_name(&self, name: &str) -> Option<u16> {
        self.name_to_id.get(&name.to_lowercase()).copied()
    }

    /// Helper to get a `Block` struct directly from a name (which should be the ronfile name).
    ///
    /// Defaults to ID 0 (Air) if not found.
    pub fn get_block_by_name(&self, name: &str) -> Block {
        // Assume ID 0 is "air"
        let id = self.get_id_by_name(&name.to_lowercase()).unwrap_or(0);
        Block { id: id.into() }
    }

    /// Helper to get `BlockProperties` directly from a name.
    /// Defaults to Air's properties if not found.
    pub fn get_properties_by_name(&self, name: &str) -> &BlockProperties {
        let id = self.get_id_by_name(name).unwrap_or(0);
        self.get(id)
    }
}

/// Runs at startup, loads all block definitions from `assets/blocks/`.
#[instrument(skip_all)]
pub fn load_block_definitions_system(mut commands: Commands) {
    info!("Loading block definitions...");

    let mut registry = BlockRegistryResource::default();

    let air_properties = BlockProperties {
        display_name: "Air".to_string(),
        is_transparent: true,
        textures: BlockFaceTextures {
            north: TextureId::Missing,
            south: TextureId::Missing,
            east: TextureId::Missing,
            west: TextureId::Missing,
            top: TextureId::Missing,
            bottom: TextureId::Missing,
        },
    };
    registry.register("air".to_string(), air_properties);

    for entry in fs::read_dir("assets/blocks").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().map_or(false, |s| s == "ron") {
            let name = match path.file_stem().and_then(|s| s.to_str()) {
                Some(name) => name.to_string(),
                None => {
                    warn!(
                        "Skipping block definition with invalid filename: {:?}",
                        path.file_name()
                    );
                    continue;
                }
            };

            if name == "air" {
                error!("Skipping 'air.ron' block definition since it's reserved.");
                continue;
            }

            let ron_string = fs::read_to_string(&path).unwrap();
            let properties = load_block_from_str(&ron_string).unwrap();

            let id = registry.register(name.clone(), properties);

            info!("Loaded block '{}' (runtime id={})", name, id);
        }
    }

    commands.insert_resource(registry);
}
