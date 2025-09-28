use crate::{
    core::graphics::context::GraphicsContext,
    ecs_bridge::{EcsState, InputSystem},
    ecs_resources::window::WindowResource,
    guard,
    prelude::*,
};
use std::error::Error;
use std::sync::Arc;
use winit::event_loop::EventLoop;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

/// The main application struct, responsible for orchestrating the event loop,
/// ECS, and graphics context.
pub struct App {
    // OS and Winit State
    window: Option<Arc<Window>>,
    input_system: InputSystem,

    // Core Engine Modules
    graphics_context: Option<GraphicsContext>,
    ecs_state: EcsState,

    // State Flags
    startup_done: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            input_system: InputSystem,
            graphics_context: None,
            ecs_state: EcsState::new(),
            startup_done: false,
        }
    }

    pub fn run_app() -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoop::new()?;

        let mut app = App::new();

        event_loop.run_app(&mut app)?;
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            info!("App started/resumed, creating window and renderer...");

            let window_attributes = Window::default_attributes()
                .with_title("🅱️")
                .with_inner_size(LogicalSize::new(1280, 720));
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.window = Some(window.clone());

            self.ecs_state
                .world
                .insert_resource(WindowResource::new(window.inner_size()));

            window.set_cursor_visible(false);
            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                error!("Failed to grab cursor: {:?}", err);
            }

            self.graphics_context = Some(pollster::block_on(GraphicsContext::new(window)));

            info!("Running startup systems...");
            self.ecs_state.run_startup();
            self.startup_done = true;
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _: StartCause) {
        guard!(self.startup_done);

        self.ecs_state.run_main();

        if let Some(window) = &self.window {
            window.request_redraw();
        }

        // We run this AFTER the main systems. It collected all the inputs from the
        // previous frame, and as such clearing it first would nullify all inputs.
        self.input_system.new_events_hook(&mut self.ecs_state.world);
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        guard!(self.startup_done);
        self.input_system
            .device_event_hook(&mut self.ecs_state.world, &event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        guard!(self.startup_done);

        self.input_system
            .window_event_hook(&mut self.ecs_state.world, &event);

        match event {
            WindowEvent::CloseRequested => {
                info!("Window close requested, exiting app event loop.");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                let mut window_resource = self.ecs_state.world.resource_mut::<WindowResource>();
                window_resource.width = physical_size.width;
                window_resource.height = physical_size.height;

                if let Some(gfx) = self.graphics_context.as_mut() {
                    gfx.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                let gfx = self.graphics_context.as_mut().unwrap();
                let (render_queue, mesh_assets, camera_uniform) = self
                    .ecs_state
                    .render_state
                    .get_mut(&mut self.ecs_state.world);

                match gfx.render(&render_queue, &mesh_assets, &camera_uniform) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        warn!("WGPU SurfaceError::Lost, resizing surface.");
                        let size = self.window.as_ref().unwrap().inner_size();
                        gfx.resize(size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("WGPU SurfaceError::OutOfMemory, exiting event loop.");
                        event_loop.exit();
                    }
                    Err(e) => eprintln!("Error during render: {:?}", e),
                }

                self.ecs_state.render_state.apply(&mut self.ecs_state.world);

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
