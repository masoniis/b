use crate::graphics::webgpu_renderer::{QueuedDraw, Vertex, WebGpuRenderer};
use bevy_ecs::prelude::ResMut;
use glam::Mat4;

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

    let triangle_draw = QueuedDraw {
        vertices,
        indices: None,
        instance_count: 1,
        transform: Mat4::IDENTITY,
    };

    renderer.queue_draw(triangle_draw);
}
