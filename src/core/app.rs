use crate::{
    core::graphics::context::GraphicsContext,
    ecs_bridge::{EcsState, EcsStateBuilder},
    ecs_modules::input::events::{RawDeviceEvent, RawWindowEvent},
    ecs_resources::{texture_map::TextureMapResource, window::WindowResource},
    guard,
    prelude::*,
};
use std::error::Error;
use std::sync::Arc;
use winit::event_loop::EventLoop;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

/// The main application struct, responsible for orchestrating the event loop,
/// ECS, and graphics context.
pub struct App {
    // OS and Winit State
    window: Option<Arc<Window>>,

    // Core Engine Modules
    graphics_context: Option<GraphicsContext>,
    ecs_state: Option<EcsState>,

    // State Flags
    startup_done: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            graphics_context: None,
            ecs_state: None,
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

            let mut builder = EcsStateBuilder::default();
            builder.add_resource(WindowResource::new(window.inner_size()));
            let mut ecs_state = builder.build();

            window.set_cursor_visible(false);
            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                error!("Failed to grab cursor: {:?}", err);
            }

            let (graphics_context, texture_map) = pollster::block_on(GraphicsContext::new(window));
            self.graphics_context = Some(graphics_context);
            ecs_state.world.insert_resource(TextureMapResource {
                registry: texture_map,
            });

            info!("Running startup systems...");
            ecs_state.schedules.startup.run(&mut ecs_state.world);

            self.startup_done = true;

            self.ecs_state = Some(ecs_state);
        }
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        guard!(self.startup_done);

        if let Some(ecs_state) = &mut self.ecs_state {
            ecs_state.world.send_event(RawDeviceEvent(event.clone()));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        guard!(self.startup_done);

        if let Some(ecs_state) = &mut self.ecs_state {
            ecs_state.world.send_event(RawWindowEvent(event.clone()));

            match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested, exiting app event loop.");
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    if let Some(gfx) = self.graphics_context.as_mut() {
                        gfx.resize(physical_size);
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Update the world just before rendering. This ensures that
                    // all the input events are processed by the world since
                    // redraw request is the final event to come through.
                    ecs_state.schedules.main.run(&mut ecs_state.world);

                    let gfx = self.graphics_context.as_mut().unwrap();
                    let (render_queue, mesh_assets, camera_uniform) =
                        ecs_state.render_state.get_mut(&mut ecs_state.world);

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

                    ecs_state.render_state.apply(&mut ecs_state.world);

                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                _ => {}
            }
        }
    }
}
