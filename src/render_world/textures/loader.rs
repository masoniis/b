use crate::{
    prelude::*,
    render_world::{
        textures::{error::TextureLoadError, registry::TextureRegistryResource},
        types::TextureId,
    },
};
use bevy_ecs::prelude::Resource;
use image::RgbaImage;
use std::{collections::HashMap, path::Path};

/// A temporary staging resource holding CPU pixel data passed from the main app loop.
#[derive(Resource)]
pub struct StagingTextureImages {
    pub images: Vec<RgbaImage>,
    pub width: u32,
    pub height: u32,
}

// INFO: ----------------------------
//         public loading API
// ----------------------------------

/// Loads texture assets from disk. Returns a registry (likely for use by sim world) as
/// well as the raw "staging" images (likely for use by render world to upload to GPU).
pub fn load_voxel_texture_assets(
) -> Result<(StagingTextureImages, TextureRegistryResource), TextureLoadError> {
    info!("Loading texture assets from disk...");

    // load
    let (mut images, texture_map) = load_images_from_manifest()?;

    // validate
    let (width, height) = determine_texture_dimensions(&images)?;
    generate_and_insert_missing_texture(&mut images, &texture_map, width, height)?;
    validate_image_dimensions(&images, width, height)?;

    // create
    let registry = TextureRegistryResource::new(texture_map)?;
    let staging = StagingTextureImages {
        images,
        width,
        height,
    };

    Ok((staging, registry))
}

// INFO: ----------------------------------
//         private helper functions
// ----------------------------------------

/// Iterates the `TextureId` manifest and loads the corresponding PNG files.
fn load_images_from_manifest() -> Result<(Vec<RgbaImage>, HashMap<TextureId, u32>), TextureLoadError>
{
    let path = Path::new("assets/textures");
    let mut images = Vec::with_capacity(TextureId::ALL.len());
    let mut texture_map = HashMap::with_capacity(TextureId::ALL.len());

    for (i, &texture_id) in TextureId::ALL.iter().enumerate() {
        let image = if texture_id == TextureId::Missing {
            RgbaImage::new(0, 0) // placeholder till a real missing image is generated
        } else {
            let texture_name = texture_id.name();
            let file_path = path.join(format!("{}.png", texture_name));
            let path_str = file_path.display().to_string();
            image::open(&file_path)
                .map_err(|e| TextureLoadError::ImageError(path_str.clone(), e))?
                .to_rgba8()
        };
        images.push(image);
        texture_map.insert(texture_id, i as u32);
    }
    Ok((images, texture_map))
}

/// Finds the first valid, non-placeholder image to determine the reference dimensions.
fn determine_texture_dimensions(images: &[RgbaImage]) -> Result<(u32, u32), TextureLoadError> {
    images
        .iter()
        .find(|img| img.width() > 0 && img.height() > 0)
        .map(|img| img.dimensions())
        .ok_or(TextureLoadError::NoTexturesFound)
}

/// Generates the "missing" texture and inserts it into the correct slot in the image vector.
fn generate_and_insert_missing_texture(
    images: &mut [RgbaImage],
    texture_map: &HashMap<TextureId, u32>,
    width: u32,
    height: u32,
) -> Result<(), TextureLoadError> {
    let missing_index = *texture_map
        .get(&TextureId::Missing)
        .ok_or(TextureLoadError::MissingTextureNotInManifest)?;
    images[missing_index as usize] = generate_missing_texture_image(width, height);
    Ok(())
}

/// Validates that all images in the vector match the reference dimensions.
fn validate_image_dimensions(
    images: &[RgbaImage],
    width: u32,
    height: u32,
) -> Result<(), TextureLoadError> {
    for (i, img) in images.iter().enumerate() {
        if img.dimensions() != (width, height) {
            let texture_id_name = TextureId::ALL[i].name().to_string();
            return Err(TextureLoadError::DimensionMismatch(
                texture_id_name,
                img.width(),
                img.height(),
                width,
                height,
            ));
        }
    }
    Ok(())
}

/// Generates the missing texture programmatically as a purple and black checkerboard pattern.
fn generate_missing_texture_image(width: u32, height: u32) -> RgbaImage {
    let mut img = RgbaImage::new(width, height);
    let checker_size = (width / 2).max(1); // 2x2 checkerboard pattern

    for y in 0..height {
        for x in 0..width {
            let checker_x = x / checker_size;
            let checker_y = y / checker_size;
            let is_even = (checker_x + checker_y) % 2 == 0;

            let color = if is_even {
                [255, 0, 255, 255] // magenta/purple
            } else {
                [0, 0, 0, 255] // black
            };

            img.put_pixel(x, y, image::Rgba(color));
        }
    }

    img
}
