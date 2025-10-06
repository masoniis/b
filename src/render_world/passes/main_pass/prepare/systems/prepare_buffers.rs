use crate::render_world::passes::main_pass::prepare::DepthTextureResource;
use crate::render_world::resources::GraphicsContextResource;
use bevy_ecs::prelude::*;
use bevy_ecs::system::Commands;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

// This system runs once to create long-lived GPU resources.
pub fn prepare_render_buffers_system(
    mut commands: Commands,
    gfx_context: Res<GraphicsContextResource>,
) {
    let device = &gfx_context.context.device;
    let config = &gfx_context.context.config;

    // --- Create Depth Texture ---

    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[DEPTH_FORMAT],
    });
    let depth_view = depth_texture.create_view(&Default::default());
    commands.insert_resource(DepthTextureResource {
        view: depth_view,
        texture: depth_texture,
    });
}
