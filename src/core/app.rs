use crate::ecs::resources::Input as InputResource;
use crate::ecs::systems::input_system;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    renderer: Option<Renderer>,
    shader_program: Option<ShaderProgram>,
    window: Option<Window>,
    input_resource: InputResource,
}

impl Default for App {
    fn default() -> Self {
        Self {
            renderer: None,
            shader_program: None,
            window: None,
            input_resource: InputResource::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            info!("App resumed, creating window!");
            let (window, gl_surface, gl_context) =
                crate::core::window::create_gl_window(event_loop);

            self.window = Some(window);

            self.shader_program = Some(
                ShaderProgram::new(
                    "src/assets/shaders/triangle.vert",
                    "src/assets/shaders/triangle.frag",
                )
                .unwrap(),
            );
            self.renderer = Some(Renderer::new(gl_surface, gl_context));
        }
    }

    /// new_events is called before any events are dispatched, so it is ideal for 1-per-frame
    /// events like updating time or anything else that should run at the BEGINNING of a cycle
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Inform input system of the event
        input_system(
            &mut self.input_resource,
            &Event::WindowEvent {
                window_id: _window_id,
                event: event.clone(),
            },
        );

        match event {
            WindowEvent::CloseRequested => {
                info!("Close button was pressed, exiting.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(renderer), Some(shader_program)) =
                    (&self.renderer, &self.shader_program)
                {
                    renderer.begin_frame();
                    shader_program.activate();
                    renderer.end_frame();

                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(renderer), Some(shader_program)) = (&self.renderer, &self.shader_program) {
            renderer.cleanup();
            shader_program.delete();
        }
    }
}
