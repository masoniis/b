use crate::prelude::*;
use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use wgpu::{Sampler, Texture, TextureView};

pub const SHADOW_DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const SHADOW_MAP_RESOLUTION: u32 = 2048;

/// A resource to hold the shadow depth texture and its view
#[derive(Resource)]
pub struct ShadowDepthTextureResource {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

/// Utility function to create the depth texture and its view
fn create_depth_texture(
    device: &RenderDevice,
    width: u32,
    height: u32,
) -> ShadowDepthTextureResource {
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Shadow Depth Texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: SHADOW_DEPTH_FORMAT,
        // texture binding usage allows sampling in shaders
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[SHADOW_DEPTH_FORMAT],
    });

    let depth_view = depth_texture.create_view(&Default::default());

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Shadow Map Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        compare: Some(wgpu::CompareFunction::LessEqual),
        ..Default::default()
    });

    ShadowDepthTextureResource {
        texture: depth_texture,
        view: depth_view,
        sampler,
    }
}

/// A system that sets up the depth texture used in the shadow pass.
///
/// This system should run once at startup to create the fixed-resolution shadow map.
#[instrument(skip_all)]
pub fn setup_shadow_depth_texture_system(
    // Input
    device: Res<RenderDevice>,

    // Output (spawned resource)
    mut commands: Commands,
) {
    debug!(
        target : "wgpu_setup",
        "Inserting shadow depth texture resource with fixed size {}x{}",
        SHADOW_MAP_RESOLUTION,
        SHADOW_MAP_RESOLUTION
    );
    let depth_resource =
        create_depth_texture(&device, SHADOW_MAP_RESOLUTION, SHADOW_MAP_RESOLUTION);
    commands.insert_resource(depth_resource);
}
