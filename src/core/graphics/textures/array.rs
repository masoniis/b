pub use super::{super::types::TextureId, TextureRegistry};
use image::RgbaImage;
use std::{collections::HashMap, path::Path};
use wgpu::{
    Device, Extent3d, Queue, Sampler, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureView,
};

pub struct TextureArray {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

/// Loads a texture array onto the device based on the compile-time `TextureId` enum manifest.
///
/// Any png added to the textures folder will be loaded here. Note that all files
/// in the texture folder msut be identical in size, or this function will error.
pub fn load_texture_array(
    device: &Device,
    queue: &Queue,
) -> Result<(TextureArray, TextureRegistry), String> {
    let path = Path::new("assets/textures");

    let mut images = Vec::with_capacity(TextureId::ALL.len()); // for temp image processing
    let mut texture_map = HashMap::with_capacity(TextureId::ALL.len()); // for texture registry

    let mut width = 0;
    let mut height = 0;

    // INFO: -----------------------
    //         Loading files
    // -----------------------------

    for (i, &texture_id) in TextureId::ALL.iter().enumerate() {
        let image = if texture_id == TextureId::Missing {
            // If this is the special 'Missing' texture, generate it procedurally.
            // We'll determine the size from the first *real* texture later.
            // For now, push a placeholder.
            RgbaImage::new(0, 0)
        } else {
            // For all other textures, load them from the file system.
            let texture_name = texture_id.name();
            let file_path = path.join(format!("{}.png", texture_name));
            image::open(&file_path)
                .map_err(|e| format!("Failed to open image {:?}: {}", file_path, e))?
                .to_rgba8()
        };

        if i == 1 {
            // After loading the first *real* texture (index 1 if Missing is at 0)
            (width, height) = image.dimensions();
        }

        images.push(image);
        texture_map.insert(texture_id, i as u32);
    }

    // Now that we know the dimensions, generate the actual missing texture
    if let Some(missing_index) = texture_map.get(&TextureId::Missing) {
        images[*missing_index as usize] = generate_missing_texture(width, height);
    } else {
        return Err("TextureId::Missing was not found in the manifest.".to_string());
    }

    if images.len() < 2 {
        // Need at least missing + 1 real texture
        return Err(format!("No textures found in directory: {:?}", path));
    }

    // 2. Verify all images have the same dimensions.
    for (i, img) in images.iter().enumerate() {
        if img.dimensions() != (width, height) {
            let texture_id = TextureId::ALL[i];
            return Err(format!(
                "Texture dimension mismatch for '{:?}': expected {}x{}, but got {}x{}",
                texture_id,
                width,
                height,
                img.width(),
                img.height()
            ));
        }
    }

    // INFO: ---------------------------
    //         Set up WGPU array
    // ---------------------------------

    let array_layers = images.len() as u32;

    let texture_size = Extent3d {
        width,
        height,
        depth_or_array_layers: array_layers,
    };

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

    // Copy each loaded image into the correct texture array layer.
    for (i, img) in images.iter().enumerate() {
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: i as u32,
                },
                aspect: wgpu::TextureAspect::All,
            },
            img.as_raw(),
            TexelCopyBufferLayout {
                offset: 0,
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

    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("texture_array_sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    // INFO: ---------------------------------
    //         Create texture registry
    // ---------------------------------------

    let missing_texture_index = texture_map[&TextureId::Missing];
    let registry = TextureRegistry::new(texture_map, missing_texture_index);

    Ok((
        TextureArray {
            texture,
            view,
            sampler,
        },
        registry,
    ))
}

/// Generates the missing texture programmatically as a purple and black checkerboard pattern.
///
/// This is necessary because our texture folder supports textures of any scale, and the texture
/// array must have all textures be the same size. Thus, we generate this texture to match size.
fn generate_missing_texture(width: u32, height: u32) -> RgbaImage {
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
