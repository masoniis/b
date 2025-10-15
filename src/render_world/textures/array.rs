pub use super::{super::types::TextureId, TextureRegistry};
use crate::prelude::*;
use image::RgbaImage;
use std::{collections::HashMap, path::Path};
use thiserror::Error;
use wgpu::{
    Device, Extent3d, Queue, Sampler, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureView,
};

#[derive(Error, Debug)]
pub enum TextureLoadError {
    #[error("Failed to read texture directory at '{0}': {1}")]
    DirectoryRead(String, std::io::Error),
    #[error("Failed to open or decode image at '{0}': {1}")]
    ImageError(String, image::ImageError),
    #[error("No valid textures found to determine dimensions.")]
    NoTexturesFound,
    #[error("Texture '{0}' has dimensions {1}x{2}, but expected {3}x{4}.")]
    DimensionMismatch(String, u32, u32, u32, u32),
    #[error("The required TextureId::Missing was not found in the manifest.")]
    MissingTextureNotInManifest,
}

pub struct TextureArray {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

/// Loads a texture array into the wgpu device based on the compile-time `TextureId` enum.
///
/// Any png added to the textures folder will be loaded here. Note that all files
/// in the texture folder must be identical in size, or this function will error.
pub fn load_texture_array(
    device: &Device,
    queue: &Queue,
) -> Result<(TextureArray, TextureRegistry), TextureLoadError> {
    info!("Loading texture array from assets/textures/...");

    // prepping the textures from filesystem
    let (mut images, texture_map) = load_images_from_manifest()?;
    let (width, height) = determine_texture_dimensions(&images)?;
    generate_and_insert_missing_texture(&mut images, &texture_map, width, height)?;
    validate_image_dimensions(&images, width, height)?;

    // create the wgpu texture array
    let texture_array = create_wgpu_texture_array(device, queue, &images, width, height);

    // crate the public facing interface for textures
    let missing_texture_index = texture_map[&TextureId::Missing];
    let registry = TextureRegistry::new(texture_map, missing_texture_index);

    info!(
        "Successfully loaded {} textures into texture array.",
        images.len()
    );
    Ok((texture_array, registry))
}

// INFO: --------------------------
//         Helper functions
// --------------------------------

/// Iterates the `TextureId` manifest and loads the corresponding PNG files.
fn load_images_from_manifest() -> Result<(Vec<RgbaImage>, HashMap<TextureId, u32>), TextureLoadError>
{
    let path = Path::new("assets/textures");
    let mut images = Vec::with_capacity(TextureId::ALL.len());
    let mut texture_map = HashMap::with_capacity(TextureId::ALL.len());

    for (i, &texture_id) in TextureId::ALL.iter().enumerate() {
        let image = if texture_id == TextureId::Missing {
            RgbaImage::new(0, 0) // placeholder till we generate a real missing image later
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

/// Generates the missing texture and inserts it into the correct slot in the image vector.
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

/// Creates the WGPU texture array and writes the image data to it.
fn create_wgpu_texture_array(
    device: &Device,
    queue: &Queue,
    images: &[RgbaImage],
    width: u32,
    height: u32,
) -> TextureArray {
    // size of the texture array
    let texture_size = Extent3d {
        width,
        height,
        depth_or_array_layers: images.len() as u32,
    };

    // create the (empty) array on the gpu
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("texture_array"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    // load each image into its respective layer in the array
    for (i, img) in images.iter().enumerate() {
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: i as u32, // array index
                },
                aspect: wgpu::TextureAspect::All,
            },
            img.as_raw(),
            TexelCopyBufferLayout {
                offset: 0,
                // each row has `width * 4` bytes for RGBA8
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }

    // create the texture view for shaders
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("texture_array_view"),
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });

    // create the sampler
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("texture_array_sampler"),
        // ClampToEdge prevents "bleeding" from adjacent textures in the array.
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    TextureArray {
        texture,
        view,
        sampler,
    }
}

/// Generates the missing texture programmatically as a purple and black checkerboard pattern.
///
/// This is necessary because our texture folder supports textures of any scale, and the texture
/// array must have all textures be the same size. Thus, we generate this texture to match size.
fn generate_missing_texture_image(width: u32, height: u32) -> RgbaImage {
    let mut img = RgbaImage::new(width, height);
    let checker_size = (width / 2).max(1); // 2x2 checkerboard pattern

    for y in 0..height {
        for x in 0..width {
            let checker_x = x / checker_size;
            let checker_y = y / checker_size;
            let is_even = (checker_x + checker_y) % 2 == 0;

            let color = if is_even {
                [255, 0, 255, 255] // Magenta/Purple
            } else {
                [0, 0, 0, 255] // Black
            };

            img.put_pixel(x, y, image::Rgba(color));
        }
    }

    img
}
