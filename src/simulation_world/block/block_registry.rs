use crate::prelude::*;
use crate::render_world::types::TextureId;
use crate::simulation_world::block::{load_block_from_str, BlockFaceTextures, BlockProperties};
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, sync::Arc};

pub type BlockId = u8;
/// ID of the default "air" block.
pub const AIR_BLOCK_ID: BlockId = 0;
/// ID of a default solid block guaranteed to exist (probably stone).
pub const SOLID_BLOCK_ID: BlockId = 1;

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

/// A system that is built to run once at startup. It scans the block directory and
/// loads all definitions found into the `BlockRegistryResource` for global access.
#[instrument(skip_all)]
pub fn initialize_block_registry_system(mut commands: Commands) {
    let registry = load_block_defs_from_disk();
    commands.insert_resource(registry);
}

/// A util that scans the block asset directory and loads all valid block definitions
/// found into a `BlockRegistryResource` struct.
#[instrument(skip_all)]
pub fn load_block_defs_from_disk() -> BlockRegistryResource {
    info!("Loading block definitions...");

    let mut block_properties: Vec<BlockProperties> = Vec::new();
    let mut name_to_id: HashMap<String, BlockId> = HashMap::new();
    let block_dir = Path::new("assets/blocks");

    // helper closure for local registration
    let mut register = |name: String, properties: BlockProperties| -> BlockId {
        let id = block_properties.len() as BlockId;
        block_properties.push(properties);
        name_to_id.insert(name.to_lowercase(), id);
        id
    };

    // register airblock to always be id 0
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

    let air_id = register("air".to_string(), air_properties);
    if air_id != 0 {
        panic!("Critical: Air block was not registered as ID 0.");
    }
    info!("Registered default block 'air' as ID 0");

    // parse the rest of the blocks from disk
    if block_dir.is_dir() {
        for entry in fs::read_dir(block_dir).unwrap_or_else(|e| {
            panic!("Failed to read block directory {:?}: {}", block_dir, e);
        }) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    warn!("Failed to read entry in block directory: {}", e);
                    continue;
                }
            };
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |s| s == "ron") {
                // name is the file stem
                let name = match path.file_stem().and_then(|s| s.to_str()) {
                    Some(name_str) => name_str.to_string(),
                    None => {
                        warn!(
                            "Skipping block definition with invalid filename: {:?}",
                            path.file_name()
                        );
                        continue;
                    }
                };

                // skip reserved names or _ files
                if name == "air" {
                    error!("Skipping 'air.ron' block definition since it is reserved.");
                    continue;
                } else if name.starts_with("_") {
                    continue;
                }

                let ron_string = match fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed to read block file {:?}: {}", path, e);
                        continue;
                    }
                };

                // construct concrete block definition object
                match load_block_from_str(&ron_string) {
                    Ok(properties) => {
                        let id = register(name.clone(), properties);
                        info!("Loaded block '{}' (runtime id={})", name, id);
                    }
                    Err(e) => {
                        error!("Failed to parse block file {:?}: {}", path, e);
                    }
                }
            }
        }
    } else {
        warn!(
            "Block directory not found at: {:?}. Only default 'Air' block was loaded.",
            block_dir
        );
    }

    let registry = BlockRegistryResource {
        block_properties: Arc::new(block_properties),
        name_to_id: Arc::new(name_to_id),
    };

    return registry;
}
