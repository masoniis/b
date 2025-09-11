use crate::ecs::systems::System;
use crate::ecs::world::World;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct RenderSystem;
impl System for RenderSystem {
    fn window_event_hook(&mut self, world: &mut World, event: &WindowEvent, window: &Window) {
        if let WindowEvent::RedrawRequested = event {
            if let (Some(renderer), Some(shader_program)) = (&world.renderer, &world.shader_program)
            {
                renderer.set_frame(shader_program, &world.camera);
                shader_program.set_mat4("modelView", &world.camera.get_view_matrix());
                shader_program.set_mat4("projection", &world.camera.get_projection_matrix());
                window.request_redraw();
            }
        }
    }
}
