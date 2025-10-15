use serde::Deserialize;

/// Loads a `BlockProperties` struct from a RON string.
/// This function handles the entire raw -> clean conversion process.
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
    pub top: String,
    pub bottom: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
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
        pub(super) fallback: Option<String>, // default if sides not specified

        // Individual faces are optional.
        pub(super) top: Option<String>,
        pub(super) bottom: Option<String>,
        pub(super) north: Option<String>,
        pub(super) south: Option<String>,
        pub(super) east: Option<String>,
        pub(super) west: Option<String>,
    }

    // convert "raw" ron into BlockProperties
    impl From<raw::BlockProperties> for super::BlockProperties {
        fn from(raw_props: raw::BlockProperties) -> Self {
            Self {
                display_name: raw_props.display_name,
                is_transparent: raw_props.is_transparent,
                // The magic happens here: we resolve the texture configuration.
                textures: raw_props.textures.resolve(),
            }
        }
    }

    impl TextureConfig {
        /// Resolves the optional texture fields into a final, non-optional struct.
        pub(super) fn resolve(self) -> BlockFaceTextures {
            let fallback = self
                .fallback
                .expect("Block texture definition in RON must have an 'fallback' field!");

            BlockFaceTextures {
                top: self.top.unwrap_or_else(|| fallback.clone()),
                bottom: self.bottom.unwrap_or_else(|| fallback.clone()),
                north: self.north.unwrap_or_else(|| fallback.clone()),
                south: self.south.unwrap_or_else(|| fallback.clone()),
                east: self.east.unwrap_or_else(|| fallback.clone()),
                west: self.west.unwrap_or_else(|| fallback.clone()),
            }
        }
    }
}
