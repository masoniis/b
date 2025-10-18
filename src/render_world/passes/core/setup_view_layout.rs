use crate::prelude::*;
use crate::render_world::resources::GraphicsContextResource;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct ViewBindGroupLayout(pub wgpu::BindGroupLayout);

/// A system to setup the view bind group (should run once at startup)
///
/// This system belongs in core because ALL pipelines share this view bind
/// group layout, where a single uniform buffer is expected for all the per-
/// frame (aka view) data.
#[instrument(skip_all)]
pub fn setup_view_bind_group_layout_system(
    mut commands: Commands,
    gfx: Res<GraphicsContextResource>,
) {
    let device = &gfx.context.device;
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("View Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    commands.insert_resource(ViewBindGroupLayout(layout));
}
