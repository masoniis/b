use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use bevy_ecs::prelude::*;
use wgpu::{Texture, TextureView};

pub const MAIN_DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// A resource to hold the depth texture and its view
#[derive(Resource)]
pub struct MainDepthTextureResource {
    pub texture: Texture,
    pub view: TextureView,
}

/// Utility function to create the depth texture and its view
fn create_depth_texture(
    device: &RenderDevice,
    width: u32,
    height: u32,
) -> MainDepthTextureResource {
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Main Depth Texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: MAIN_DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[MAIN_DEPTH_FORMAT],
    });

    let depth_view = depth_texture.create_view(&Default::default());

    MainDepthTextureResource {
        texture: depth_texture,
        view: depth_view,
    }
}

/// A system that sets up the depth texture used in the opaque and transparent passes.
///
/// This system should run once at startup to create the initial depth texture.
#[instrument(skip_all)]
pub fn setup_main_depth_texture_system(
    // Input
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,

    // Output (spawned resource)
    mut commands: Commands,
) {
    debug!(
        target : "wgpu_setup",
        "Inserting main depth texture resource with size {}x{}",
        config.width,
        config.height
    );
    let depth_resource = create_depth_texture(&device, config.width, config.height);
    commands.insert_resource(depth_resource);
}

/// A system that resizes the depth texture if the surface configuration changes.
///
/// Since the depth texture depends on the surface configuration (width, height, format),
/// this system must run again if the surface is resized. It relies on the setup system
/// having already inserted the resource.
#[instrument(skip_all)]
pub fn resize_main_depth_texture_system(
    // Input
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,

    // Output (updated resource)
    mut depth_texture_res: ResMut<MainDepthTextureResource>,
) {
    if cfg!(debug_assertions) && !config.is_changed() {
        error!(
            "resize_main_depth_texture_system was called but RenderSurfaceConfig has not changed! This indicates a scheduling error."
        );
    }

    debug!(
        target : "wgpu_resize",
        "Updating main depth texture resource to use width {}x{}",
        config.width,
        config.height
    );

    let new_depth_resource = create_depth_texture(&device, config.width, config.height);
    *depth_texture_res = new_depth_resource;
}
