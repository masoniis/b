use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use bevy_ecs::prelude::Resource;
use bevy_ecs::prelude::*;
use bevy_ecs::system::Commands;
use wgpu::{Texture, TextureView};

/// A resource to hold the depth texture and its view, used for depth testing.
#[derive(Resource)]
pub struct DepthTextureResource {
    /// The depth texture itself. We must hold this to prevent it from being dropped.
    pub texture: Texture,
    /// The view of the depth texture, which is what render passes use.
    pub view: TextureView,
}

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

// This system runs once to create long-lived GPU resources.
#[instrument(skip_all)]
pub fn prepare_render_buffers_system(
    mut commands: Commands,
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,
) {
    let device = &device.0;
    let config = &config.0;

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
