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

impl FromWorld for ShadowDepthTextureResource {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        debug!(
            target : "wgpu_setup",
            "Creating shadow depth texture resource with fixed size {}x{}",
            SHADOW_MAP_RESOLUTION,
            SHADOW_MAP_RESOLUTION
        );

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Shadow Depth Texture"),
            size: wgpu::Extent3d {
                width: SHADOW_MAP_RESOLUTION,
                height: SHADOW_MAP_RESOLUTION,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: SHADOW_DEPTH_FORMAT,
            // texture binding for sampling in shaders (shadow map)
            // render attachment for rendering to it (shadow pass)
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[SHADOW_DEPTH_FORMAT],
        });

        let view = texture.create_view(&Default::default());

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

        Self {
            texture,
            view,
            sampler,
        }
    }
}
