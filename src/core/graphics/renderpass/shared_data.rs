use wgpu::util::DeviceExt;

use crate::core::graphics::rendercore::camera_uniform::CameraUniform;

pub struct SharedRenderData {
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
}

impl SharedRenderData {
    pub fn new(device: &wgpu::Device) -> Self {
        // Create the buffer
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shared Camera Buffer"),
            contents: bytemuck::cast_slice(&[CameraUniform::new()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create the bind group layout that ALL passes will use for the camera
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Shared Camera Bind Group Layout"),
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

        // Create the actual bind group
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Shared Camera Bind Group"),
        });

        Self {
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
        }
    }

    pub fn update_camera(&self, queue: &wgpu::Queue, view_proj_matrix: glam::Mat4) {
        let mut camera_uniform_gpu = CameraUniform::new();
        camera_uniform_gpu.update_view_proj(view_proj_matrix);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform_gpu]),
        );
    }
}
