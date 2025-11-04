use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
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

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// A system that sets up the depth texture used in the opaque render pass.
///
/// Since the depth texture depends on the surface configuration (width, height, format),
/// this system must run again if the surface is resized.
#[instrument(skip_all)]
pub fn setup_or_resize_opaque_depth_texture_system(
    // Input
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,

    // Output (spawned/updated resource)
    mut commands: Commands,
    depth_texture_res: Option<ResMut<DepthTextureResource>>,
) {
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Opaque Depth Texture"),
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

    if let Some(mut existing_depth_res) = depth_texture_res {
        // resize existing
        debug!(
            target : "wgpu_resize",
            "Updating opauqe depth texture resource to use width {}x{}",
            config.width,
            config.height
        );

        *existing_depth_res = DepthTextureResource {
            view: depth_view,
            texture: depth_texture,
        };
    } else {
        // insert for first time
        commands.insert_resource(DepthTextureResource {
            view: depth_view,
            texture: depth_texture,
        });
    }
}
