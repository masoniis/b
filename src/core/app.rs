use crate::ecs::systems::{CameraSystem, InputSystem, RenderSystem, System};
use crate::ecs::world::World;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use std::time::Instant;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    // OS Interactions
    window: Option<Window>,
    last_frame_time: Instant,

    // Game logic
    world: World,
    systems: Vec<Box<dyn System>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            world: World::default(),
            systems: vec![
                Box::new(InputSystem),
                Box::new(CameraSystem),
                Box::new(RenderSystem),
            ],
            last_frame_time: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            info!("App resumed, creating window!");
            let (window, gl_surface, gl_context) =
                crate::core::window::create_gl_window(event_loop, self.world.window_size);

            self.window = Some(window);

            // Hide and grab the cursor
            if let Some(window_ref) = self.window.as_ref() {
                window_ref.set_cursor_visible(false);
                if let Err(err) =
                    window_ref.set_cursor_grab(winit::window::CursorGrabMode::Confined)
                {
                    info!("Failed to grab cursor: {:?}", err);
                }
            }

            self.world.shader_program = Some(
                ShaderProgram::new(
                    "src/assets/shaders/simple.vert",
                    "src/assets/shaders/triangle.frag",
                )
                .unwrap(),
            );
            self.world.renderer = Some(Renderer::new(gl_surface, gl_context));
        }
    }

    /// new_events is called before any events are dispatched, so it is ideal for 1-per-frame
    /// events like updating time or anything else that should run at the BEGINNING of a cycle
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        let current_time = Instant::now();
        let delta_time = current_time
            .duration_since(self.last_frame_time)
            .as_secs_f32();
        self.last_frame_time = current_time;
        self.world.delta_time.0 = delta_time;

        for system in &mut self.systems {
            system.new_events_hook(&mut self.world);
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        for system in &mut self.systems {
            system.device_event_hook(&mut self.world, &event);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = self.window.as_ref() {
            for system in &mut self.systems {
                system.window_event_hook(&mut self.world, &event, window);
            }
        }

        // Handle other overarching events that don't directly effect game state
        // (redraw requests, resizing, closing, etc)
        match event {
            WindowEvent::CloseRequested => {
                info!("Close button was pressed, exiting.");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.world.window_size = (physical_size.width, physical_size.height);
                self.world.camera.projection_dirty = true;
            }
            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(renderer), Some(shader_program)) =
            (&self.world.renderer, &self.world.shader_program)
        {
            renderer.cleanup();
            shader_program.delete();
        }
    }
}
