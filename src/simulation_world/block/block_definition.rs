use crate::render_world::types::TextureId;
use serde::Deserialize;

/// Loads a block definition from string and returns two hot/cold split structs
pub fn load_block_from_str(
    ron_string: &str,
) -> Result<(BlockRenderData, BlockDescription), ron::Error> {
    let raw_properties: raw::BlockProperties = ron::from_str(ron_string)?;
    Ok(raw_properties.split_into_components())
}

/// Optimized hot block data required for meshing and rendering.
#[derive(Debug, Clone, Copy)]
pub struct BlockRenderData {
    pub textures: BlockFaceTextures,
    pub is_transparent: bool,
}

/// Cold "heavy" block metadata.
#[derive(Debug, Clone)]
pub struct BlockDescription {
    pub display_name: String,
}

// INFO: ------------------
//         subtypes
// ------------------------

/// The textures associated with each face of a particular block type.
#[derive(Debug, Clone, Copy)]
pub struct BlockFaceTextures<T = TextureId> {
    pub top: T,
    pub bottom: T,
    pub front: T,
    pub back: T,
    pub right: T,
    pub left: T,
}

impl<T: Copy> BlockFaceTextures<T> {
    pub fn map<U, F>(self, mut f: F) -> BlockFaceTextures<U>
    where
        F: FnMut(T) -> U,
    {
        BlockFaceTextures {
            top: f(self.top),
            bottom: f(self.bottom),
            front: f(self.front),
            back: f(self.back),
            right: f(self.right),
            left: f(self.left),
        }
    }

    #[inline(always)]
    pub fn get(&self, face_index: usize) -> T {
        match face_index {
            0 => self.top,
            1 => self.bottom,
            2 => self.left,
            3 => self.right,
            4 => self.front,
            _ => self.back,
        }
    }
}

// INFO: -------------------------
//         deserialization
// -------------------------------

mod raw {
    use super::*;

    /// A struct that matches the structure of the block RON files
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub(super) struct BlockProperties {
        pub(super) display_name: String,
        pub(super) textures: TextureConfig,
        pub(super) is_transparent: bool,
    }

    impl BlockProperties {
        /// Consumes the raw struct and returns hot/cold separated components.
        pub fn split_into_components(self) -> (super::BlockRenderData, super::BlockDescription) {
            let render_data = super::BlockRenderData {
                textures: self.textures.resolve(),
                is_transparent: self.is_transparent,
            };

            let description = super::BlockDescription {
                display_name: self.display_name,
            };

            (render_data, description)
        }
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub(super) struct TextureConfig {
        pub(super) fallback: TextureId,

        #[serde(default)]
        pub top: Option<TextureId>,
        #[serde(default)]
        pub bottom: Option<TextureId>,
        #[serde(default)]
        pub front: Option<TextureId>,
        #[serde(default)]
        pub back: Option<TextureId>,
        #[serde(default)]
        pub right: Option<TextureId>,
        #[serde(default)]
        pub left: Option<TextureId>,
    }

    impl TextureConfig {
        pub(super) fn resolve(self) -> BlockFaceTextures {
            let fallback = self.fallback;

            BlockFaceTextures {
                top: self.top.unwrap_or(fallback),
                bottom: self.bottom.unwrap_or(fallback),
                front: self.front.unwrap_or(fallback),
                back: self.back.unwrap_or(fallback),
                right: self.right.unwrap_or(fallback),
                left: self.left.unwrap_or(fallback),
            }
        }
    }
}
