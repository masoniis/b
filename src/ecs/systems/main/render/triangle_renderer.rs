use crate::graphics::Vertex;
use crate::graphics::webgpu_renderer::{QueuedDraw, WebGpuRenderer};
use bevy_ecs::prelude::ResMut;
use glam::Mat4;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub fn triangle_render_system(mut renderer: ResMut<WebGpuRenderer>) {
    #[rustfmt::skip]
    let vertices = vec![
        Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 0.0, 1.0] },

        Vertex { position: [-1.0, 1.0, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [-1.0, 0.9, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [-0.9, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
    ];

    let device = renderer.get_device();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Index Buffer"),
        contents: bytemuck::cast_slice(&[0, 1, 2, 3, 4, 5]),
        usage: wgpu::BufferUsages::INDEX,
    });

    let gpu_mesh = Arc::new(crate::graphics::GpuMesh {
        vertex_buffer,
        index_buffer,
        index_count: 6,
    });

    let triangle_draw = QueuedDraw {
        gpu_mesh,
        instance_count: 1,
        transform: Mat4::IDENTITY,
    };

    renderer.queue_draw(triangle_draw);
}
