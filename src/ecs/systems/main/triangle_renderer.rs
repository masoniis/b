use crate::graphics::webgpu_renderer::{QueuedDraw, Vertex, WebGpuRenderer};
use bevy_ecs::prelude::ResMut;

pub fn triangle_render_system(mut renderer: ResMut<WebGpuRenderer>) {
    renderer.clear_queue();

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
    };

    renderer.queue_draw(triangle_draw);
}
