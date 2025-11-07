use crate::prelude::*;
use crate::render_world::types::TextureId;
use crate::simulation_world::block::{load_block_from_str, BlockFaceTextures, BlockProperties};
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::{fs, sync::Arc};

pub type BlockId = u8;
pub const AIR_BLOCK_ID: BlockId = 0;

#[derive(Resource, Default, Clone)]
pub struct BlockRegistryResource {
    /// Stores properties indexed by the runtime ID.
    pub block_properties: Arc<Vec<BlockProperties>>,

    /// Maps a string name to the runtime ID.
    name_to_id: Arc<HashMap<String, BlockId>>,
}

impl BlockRegistryResource {
    /// Gets the properties for a given block ID.
    ///
    /// Always returns a valid property (defaults to Air).
    pub fn get(&self, id: BlockId) -> &BlockProperties {
        self.block_properties
            .get(id as usize)
            .unwrap_or(&self.block_properties[0]) // air is 0
    }

    /// Gets the numeric ID for a given block name.
    ///
    /// The string name of a block is based on its file name
    /// without the extension. Eg: "grass.ron" -> "grass".
    pub fn get_id_by_name(&self, name: &str) -> Option<BlockId> {
        self.name_to_id.get(&name.to_lowercase()).copied()
    }

    /// Helper to get a `Block` struct directly from a name (which should be the ronfile name).
    ///
    /// Defaults to ID 0 (Air) if not found.
    pub fn get_block_by_name(&self, name: &str) -> BlockId {
        // Assume ID 0 is "air"
        self.get_id_by_name(&name.to_lowercase())
            .unwrap_or(0 as BlockId)
    }

    /// Helper to get `BlockProperties` directly from a name.
    /// Defaults to Air's properties if not found.
    pub fn get_properties_by_name(&self, name: &str) -> &BlockProperties {
        let id = self.get_id_by_name(name).unwrap_or(0 as BlockId);
        self.get(id)
    }
}

// INFO: ------------------------------
//         System to load files
// ------------------------------------

/// Runs at startup, loads all block definitions from `assets/blocks/`.
#[instrument(skip_all)]
pub fn load_block_definitions_system(mut commands: Commands) {
    info!("Loading block definitions...");

    let mut block_properties: Vec<BlockProperties> = Vec::new();
    let mut name_to_id: HashMap<String, BlockId> = HashMap::new();

    // helper closure for local registration
    let mut register = |name: String, properties: BlockProperties| -> BlockId {
        let id = block_properties.len() as BlockId;
        block_properties.push(properties);
        name_to_id.insert(name.to_lowercase(), id);
        id
    };

    let air_properties = BlockProperties {
        display_name: "Air".to_string(),
        is_transparent: true,
        textures: BlockFaceTextures {
            front: TextureId::Missing,
            back: TextureId::Missing,
            right: TextureId::Missing,
            left: TextureId::Missing,
            top: TextureId::Missing,
            bottom: TextureId::Missing,
        },
    };
    register("air".to_string(), air_properties);

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
            } else if name.starts_with("_") {
                continue;
            }

            let ron_string = fs::read_to_string(&path).unwrap();
            let properties = load_block_from_str(&ron_string).unwrap();

            let id = register(name.clone(), properties);

            info!("Loaded block '{}' (runtime id={})", name, id);
        }
    }

    let registry = BlockRegistryResource {
        block_properties: Arc::new(block_properties),
        name_to_id: Arc::new(name_to_id),
    };

    commands.insert_resource(registry);
}
