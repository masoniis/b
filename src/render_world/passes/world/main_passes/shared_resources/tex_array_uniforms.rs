use crate::{
    prelude::*,
    render_world::{
        graphics_context::resources::{RenderDevice, RenderQueue},
        textures::loader::StagingTextureImages,
    },
};
use bevy_ecs::prelude::*;
use image::RgbaImage;
use wgpu::{
    Device, Extent3d, Queue, Sampler, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureView,
};

// INFO: -------------------
//         resources
// -------------------------

/// The layout resource for the global texture map (@group(2)).
#[derive(Resource)]
pub struct TextureArrayBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for TextureArrayBindGroupLayout {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Map Bind Group Layout"),
            entries: &[
                // binding 0: texture array
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                // binding 1: sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        Self(layout)
    }
}

/// A resource owning the GPU data for the texture array for all voxel textures.
#[derive(Resource)]
pub struct TextureArrayUniforms {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub bind_group: wgpu::BindGroup,
}

impl FromWorld for TextureArrayUniforms {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        // steal the staging data
        let staging = world
            .remove_resource::<StagingTextureImages>()
            .expect("TextureArrayUniforms requires StagingTextureImages to be inserted first!");

        // get other deps
        let device = world.resource::<RenderDevice>().clone();
        let queue = world.resource::<RenderQueue>().clone();
        let layout = world.resource::<TextureArrayBindGroupLayout>();

        // create wgpu tex array
        let (texture, view, sampler) = create_wgpu_texture_array(
            &device,
            &queue,
            &staging.images,
            staging.width,
            staging.height,
        );

        // set up the bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Map Bind Group"),
            layout: &layout.0,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self {
            texture,
            view,
            sampler,
            bind_group,
        }
    }
}
/// Creates the WGPU texture array and writes the image data to it.
///
/// Returns the raw WGPU types.
fn create_wgpu_texture_array(
    device: &Device,
    queue: &Queue,
    images: &[RgbaImage],
    width: u32,
    height: u32,
) -> (Texture, TextureView, Sampler) {
    let mut effective_images = images.to_vec();

    // on WGPU -> vulkan, a heuristic is assumed about the texture array dimensions (being a Cube or a HyperCube)
    // but the heurisitc is incorrect if we are mod 6 == 0, so pad with an image to ensure textures work in this case.
    if !effective_images.is_empty() && effective_images.len() % 6 == 0 {
        if let Some(last) = effective_images.last() {
            effective_images.push(last.clone());
        }
    }

    // size of the texture array
    let texture_size = Extent3d {
        width,
        height,
        depth_or_array_layers: effective_images.len() as u32,
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
    for (i, img) in effective_images.iter().enumerate() {
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
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    (texture, view, sampler)
}
