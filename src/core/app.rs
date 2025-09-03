use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::GlSurface;
use glutin::surface::Surface;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    renderer: Option<Renderer>,
    shader_program: Option<ShaderProgram>,
    window: Option<Window>,
    gl_surface: Option<Surface<glutin::surface::WindowSurface>>,
    gl_context: Option<PossiblyCurrentContext>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            renderer: None,
            shader_program: None,
            window: None,
            gl_surface: None,
            gl_context: None,
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
            self.gl_surface = Some(gl_surface);
            self.gl_context = Some(gl_context);

            self.shader_program = Some(
                ShaderProgram::new(
                    "src/assets/shaders/triangle.vert",
                    "src/assets/shaders/triangle.frag",
                )
                .unwrap(),
            );
            self.renderer = Some(Renderer::new());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Close button was pressed, exiting.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(renderer), Some(shader_program), Some(gl_surface), Some(gl_context)) = (
                    &self.renderer,
                    &self.shader_program,
                    &self.gl_surface,
                    &self.gl_context,
                ) {
                    unsafe {
                        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }

                    shader_program.use_program();
                    renderer.draw_triangle();

                    gl_surface.swap_buffers(gl_context).unwrap();
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
