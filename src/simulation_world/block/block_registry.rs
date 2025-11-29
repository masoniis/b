use crate::{
    prelude::*,
    render_world::textures::{registry::TextureId, TextureRegistryResource},
    simulation_world::block::{
        load_block_from_str, BlockDescription, BlockFaceTextures, BlockRenderData,
    },
};
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, sync::Arc};

pub type BlockId = u8;
/// ID of the default "air" block.
pub const AIR_BLOCK_ID: BlockId = 0;
/// ID of a default solid block guaranteed to exist (probably stone).
pub const SOLID_BLOCK_ID: BlockId = 1;

#[derive(Resource, Clone)]
pub struct BlockRegistryResource {
    /// All loaded block render data from disc.
    render_data: Arc<Vec<BlockRenderData>>,
    /// Direct access to transparency data from BlockRenderData
    /// to optimize super hot loops (meshing).
    transparency_lut: Arc<Vec<bool>>,
    /// Direct access to slices of TextureIds (Hot Array)
    /// to optimize super hot loops (meshing).
    /// Layout: [Top, Bottom, Left, Right, Front, Back]
    texture_lut: Arc<Vec<[TextureId; 6]>>,

    /// All loaded block descriptors from disc.
    descriptions: Arc<Vec<BlockDescription>>,

    /// Maps a string name to the runtime ID.
    name_to_id: Arc<HashMap<String, BlockId>>,
}

impl BlockRegistryResource {
    /// Gets the render data for a given block ID.
    ///
    /// Will have undefined behavior if the block ID is not within bounds.
    #[inline(always)]
    pub fn get_render_data(&self, id: BlockId) -> &BlockRenderData {
        unsafe { &self.render_data.get_unchecked(id as usize) }
    }

    /// Gets the description/metadata for a given block ID.
    ///
    /// Will have undefined behavior if the block ID is not within bounds.
    #[inline(always)]
    pub fn get_description(&self, id: BlockId) -> &BlockDescription {
        unsafe { &self.descriptions.get_unchecked(id as usize) }
    }

    /// Gets the numeric ID for a given block name.
    ///
    /// The string name of a block is based on the block ron-file name
    /// without the extension. Eg: "grass.ron" -> "grass".
    #[inline(always)]
    pub fn get_block_id_by_name(&self, name: &str) -> Option<BlockId> {
        self.name_to_id.get(&name.to_lowercase()).copied()
    }

    /// Returns a slice of booleans representing the transparency state of all blocks.
    /// Index is BlockId.
    ///
    /// Use this for AO calculation to maximize cache hit rate.
    #[inline(always)]
    pub fn get_transparency_lut(&self) -> &[bool] {
        &self.transparency_lut
    }

    /// Returns a slice of texture arrays for all blocks.
    /// Index is BlockId.
    ///
    /// Use this for meshing to ensure O(1) array indexing without branching.
    #[inline(always)]
    pub fn get_texture_lut(&self) -> &[[TextureId; 6]] {
        &self.texture_lut
    }
}

impl FromWorld for BlockRegistryResource {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        info!("Loading block definitions from disk...");

        let texture_registry = world.get_resource::<TextureRegistryResource>().unwrap();

        let mut render_data_vec: Vec<BlockRenderData<TextureId>> = Vec::new();
        let mut descriptions_vec: Vec<BlockDescription> = Vec::new();
        // This vector stores the hot path [u16; 6] arrays
        let mut texture_lut_vec: Vec<[TextureId; 6]> = Vec::new();
        let mut name_to_id: HashMap<String, BlockId> = HashMap::new();

        let block_dir = Path::new("assets/blocks");

        // closure to insert split data into parallel vectors
        let mut register =
            |name: String, render: BlockRenderData<String>, desc: BlockDescription| -> BlockId {
                let id = render_data_vec.len() as BlockId;

                // resolve strings from parsed ron to textures
                let resolved_textures = render.textures.map(|n| texture_registry.get_id(&n));

                let render_with_ids = BlockRenderData {
                    is_transparent: render.is_transparent,
                    textures: resolved_textures.clone(),
                };

                // hot texture array
                let texture_array = [
                    resolved_textures.top,
                    resolved_textures.bottom,
                    resolved_textures.right,
                    resolved_textures.left,
                    resolved_textures.front,
                    resolved_textures.back,
                ];

                render_data_vec.push(render_with_ids);
                descriptions_vec.push(desc);
                texture_lut_vec.push(texture_array);

                name_to_id.insert(name.to_lowercase(), id);

                id
            };

        // INFO: ---------------------------------------
        //         manual air block registration
        // ---------------------------------------------

        let air_render = BlockRenderData {
            is_transparent: true,
            textures: BlockFaceTextures {
                front: "missing".to_string(),
                back: "missing".to_string(),
                right: "missing".to_string(),
                left: "missing".to_string(),
                top: "missing".to_string(),
                bottom: "missing".to_string(),
            },
        };

        let air_desc = BlockDescription {
            display_name: "Air".to_string(),
        };

        let air_id = register("air".to_string(), air_render, air_desc);
        if air_id != 0 {
            panic!("Critical: Air block was not registered as ID 0.");
        }
        info!("Registered default block 'air' as ID 0");

        // INFO: ------------------------------------------
        //         parse remaining blocks from disc
        // ------------------------------------------------

        if block_dir.is_dir() {
            let entries = fs::read_dir(block_dir).unwrap_or_else(|e| {
                panic!("Failed to read block directory {:?}: {}", block_dir, e);
            });

            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        warn!("Failed to read entry in block directory: {}", e);
                        continue;
                    }
                };
                let path = entry.path();

                // ignore non-ron files
                if path.is_file() && path.extension().map_or(false, |s| s == "ron") {
                    let name = match path.file_stem().and_then(|s| s.to_str()) {
                        Some(name_str) => name_str.to_string(),
                        None => {
                            warn!("Skipping invalid filename: {:?}", path.file_name());
                            continue;
                        }
                    };

                    // skip reserved names
                    if name == "air" {
                        error!("Skipping 'air.ron' (Reserved).");
                        continue;
                    } else if name.starts_with('_') {
                        continue;
                    }

                    // read & register
                    let ron_string = match fs::read_to_string(&path) {
                        Ok(s) => s,
                        Err(e) => {
                            error!("Failed to read {:?}: {}", path, e);
                            continue;
                        }
                    };

                    match load_block_from_str(&ron_string) {
                        Ok((render_props, desc_props)) => {
                            let id = register(name.clone(), render_props, desc_props);
                            info!("Loaded block '{}' (id={})", name, id);
                        }
                        Err(e) => {
                            error!("Failed to parse {:?}: {}", path, e);
                        }
                    }
                }
            }
        } else {
            warn!(
                "Block directory not found at: {:?}. Only 'Air' loaded.",
                block_dir
            );
        }

        let transparency_lut: Vec<bool> =
            render_data_vec.iter().map(|d| d.is_transparent).collect();

        Self {
            render_data: Arc::new(render_data_vec),
            transparency_lut: Arc::new(transparency_lut),
            texture_lut: Arc::new(texture_lut_vec),
            descriptions: Arc::new(descriptions_vec),
            name_to_id: Arc::new(name_to_id),
        }
    }
}
