use crate::render_world::types::TextureId;
use serde::Deserialize;

/// Loads a `BlockProperties` struct from a RON string.
///
/// Handles the entire raw ron -> type `BlockProperties` conversion process.
pub fn load_block_from_str(ron_string: &str) -> Result<BlockProperties, ron::Error> {
    let raw_properties: raw::BlockProperties = ron::from_str(ron_string)?;
    Ok(raw_properties.into())
}

#[derive(Debug, Clone)]
pub struct BlockProperties {
    pub display_name: String,
    pub textures: BlockFaceTextures,
    pub is_transparent: bool,
}

#[derive(Debug, Clone)]
pub struct BlockFaceTextures {
    pub top: TextureId,
    pub bottom: TextureId,
    pub front: TextureId,
    pub back: TextureId,
    pub right: TextureId,
    pub left: TextureId,
}

mod raw {
    use super::*;

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)] // error if RON has unknown fields
    pub(super) struct BlockProperties {
        pub(super) display_name: String,
        pub(super) textures: TextureConfig,
        pub(super) is_transparent: bool,
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub(super) struct TextureConfig {
        pub(super) fallback: TextureId, // default if sides not specified

        // individual faces are optional
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

    // convert "raw" ron into BlockProperties
    impl From<raw::BlockProperties> for super::BlockProperties {
        fn from(raw_props: raw::BlockProperties) -> Self {
            Self {
                display_name: raw_props.display_name,
                is_transparent: raw_props.is_transparent,
                textures: raw_props.textures.resolve(),
            }
        }
    }

    impl TextureConfig {
        /// Resolves the optional texture fields into a final, non-optional struct.
        pub(super) fn resolve(self) -> BlockFaceTextures {
            let fallback = self.fallback;

            BlockFaceTextures {
                top: self.top.unwrap_or_else(|| fallback.clone()),
                bottom: self.bottom.unwrap_or_else(|| fallback.clone()),
                front: self.front.unwrap_or_else(|| fallback.clone()),
                back: self.back.unwrap_or_else(|| fallback.clone()),
                right: self.right.unwrap_or_else(|| fallback.clone()),
                left: self.left.unwrap_or_else(|| fallback.clone()),
            }
        }
    }
}
