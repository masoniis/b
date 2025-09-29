use super::TextureRegistry;
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

/// Loads all images from a directory, creates a single `wgpu::Texture` array,
/// and returns it along with a texture registry for looking up indices by name.
///
///  NOTE: This function assumes all images in the directory have the same dimensions.
/// Texture files should be named exactly as they'll be referenced (e.g., "grass.png")
/// but the extension doesn't matter as long as the image format can be opened by rust.
pub fn load_texture_array(
    device: &Device,
    queue: &Queue,
    path: &Path,
) -> Result<(TextureArray, TextureRegistry), String> {
    // 1. Read directory and sort files to ensure a deterministic order.
    let mut paths: Vec<_> = std::fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory {:?}: {}", path, e))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    // Sort by filename to guarantee the texture order is always the same.
    paths.sort();

    // 2. Load images into memory and build the name-to-index map.
    let mut name_to_index = HashMap::new();
    let images: Vec<RgbaImage> = paths
        .iter()
        .enumerate()
        .map(|(i, path)| {
            // Get the filename without extension to use as the texture name.
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                name_to_index.insert(name.to_string(), i as u32);
            }

            // Load the image and convert to RGBA8 format.
            let img =
                image::open(path).map_err(|e| format!("Failed to open image {:?}: {}", path, e))?;
            Ok(img.to_rgba8())
        })
        .collect::<Result<Vec<_>, String>>()?;

    if images.is_empty() {
        return Err(format!("No textures found in directory: {:?}", path));
    }

    let (width, height) = images[0].dimensions();

    // Create the missing texture as the first layer (index 0)
    let missing_texture = generate_missing_texture(width, height);
    let mut all_images = vec![missing_texture];
    all_images.extend(images);

    let array_layers = all_images.len() as u32;

    // Verify all loaded images have the same dimensions
    for (i, img) in all_images.iter().enumerate() {
        let (img_width, img_height) = img.dimensions();
        if img_width != width || img_height != height {
            return Err(format!(
                "Texture dimension mismatch: expected {}x{}, but '{}' has dimensions {}x{}",
                width,
                height,
                paths[i]
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown"),
                img_width,
                img_height
            ));
        }
    }

    // Update the name-to-index map to account for missing texture at index 0
    let mut adjusted_map = HashMap::new();
    for (name, idx) in name_to_index {
        adjusted_map.insert(name, idx + 1); // Shift all indices by 1
    }

    // 3. Create the wgpu Texture Array on the GPU.
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

    // 4. Loop through all images (including missing texture) and copy each one into the correct layer.
    for (i, img) in all_images.iter().enumerate() {
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: i as u32, // The 'z' origin is the layer index.
                },
                aspect: wgpu::TextureAspect::All,
            },
            img.as_raw(), // Convert RgbaImage to &[u8]
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1, // We are writing one layer at a time.
            },
        );
    }

    // 5. Create the view and sampler.
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("texture_array_sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        // Use Nearest for the classic, sharp pixelated look.
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    Ok((
        TextureArray {
            texture,
            view,
            sampler,
        },
        TextureRegistry::new(adjusted_map, 0), // Missing texture is at index 0
    ))
}

/// Generates the missing texture programmatically as a purple and black checkerboard pattern.
fn generate_missing_texture(width: u32, height: u32) -> RgbaImage {
    let mut img = RgbaImage::new(width, height);
    let checker_size = (width / 2).max(1); // 2x2 checkerboard pattern

    for y in 0..height {
        for x in 0..width {
            // Determine which checker square we're in
            let checker_x = x / checker_size;
            let checker_y = y / checker_size;
            let is_even = (checker_x + checker_y) % 2 == 0;

            // Purple and black checkerboard
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
