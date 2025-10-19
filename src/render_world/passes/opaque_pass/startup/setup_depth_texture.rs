use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use crate::render_world::passes::opaque_pass::startup::DEPTH_FORMAT;
use bevy_ecs::prelude::Resource;
use bevy_ecs::prelude::*;
use bevy_ecs::system::Commands;
use wgpu::{Texture, TextureView};

/// A resource to hold the depth texture and its view
#[derive(Resource)]
pub struct DepthTextureResource {
    pub texture: Texture,
    pub view: TextureView,
}

/// A system that sets up the depth texture used in the opaque render pass.
///
/// Since the depth texture depends on the surface configuration (width, height, format),
/// this system must run again if the surface is resized.
#[instrument(skip_all)]
pub fn setup_depth_texture_system(
    // Input
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,

    // Output (spawned resource)
    mut commands: Commands,
) {
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
