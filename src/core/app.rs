use crate::{
    core::graphics::context::GraphicsContext,
    core::{ExternalEcsInterface, ExternalEcsInterfaceBuilder},
    ecs_resources::{
        graphics_context::GraphicsContextResource, texture_map::TextureMapResource,
        window::WindowResource,
    },
    game_world::{
        input::events::{RawDeviceEvent, RawWindowEvent},
        schedules::ScheduleLables,
        state_machine::resources::AppState,
    },
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
    ecs_interface: Option<ExternalEcsInterface>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            ecs_interface: None,
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

            window.set_cursor_visible(false);
            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                error!("Failed to grab cursor: {:?}", err);
            }

            let (graphics_context, texture_map) =
                pollster::block_on(GraphicsContext::new(window.clone()));

            let mut builder = ExternalEcsInterfaceBuilder::default();
            builder
                .add_resource(WindowResource::new(window.inner_size()))
                .add_resource(GraphicsContextResource {
                    context: graphics_context,
                })
                .add_resource(TextureMapResource {
                    registry: texture_map,
                });
            let mut ecs_interface = builder.build();

            info!("Running startup systems...\n\n\n");
            ecs_interface.run_schedule(ScheduleLables::Startup);

            self.window = Some(window.clone());
            self.ecs_interface = Some(ecs_interface);
        }
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let Some(ecs_interface) = &mut self.ecs_interface {
            ecs_interface.send_event(RawDeviceEvent(event.clone()));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(ecs_interface) = &mut self.ecs_interface {
            ecs_interface.send_event(RawWindowEvent(event.clone()));

            // NOTE: The events handled here should only be events that rely on the event loop
            // itself. Any other event should be fine to handle within the ECS world itself.
            match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested, exiting app event loop.");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    let current_app_state = ecs_interface.get_app_state();

                    match current_app_state {
                        AppState::Loading => {
                            ecs_interface.run_schedule(ScheduleLables::Loading);
                        }
                        AppState::Running => {
                            ecs_interface.run_schedule(ScheduleLables::Main);
                        }
                        AppState::Closing => {}
                    };

                    // Request the next frame
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                _ => {}
            }
        }
    }
}
