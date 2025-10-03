use crate::{
    core::graphics::context::GraphicsContext,
    ecs_bridge::{EcsState, EcsStateBuilder},
    ecs_modules::{
        input::events::{RawDeviceEvent, RawWindowEvent},
        state_machine::resources::{AppState, CurrentState},
    },
    ecs_resources::{
        graphics_context::GraphicsContextResource, texture_map::TextureMapResource,
        window::WindowResource,
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
    ecs_state: Option<EcsState>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            ecs_state: None,
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

            let mut builder = EcsStateBuilder::default();
            builder
                .add_resource(WindowResource::new(window.inner_size()))
                .add_resource(GraphicsContextResource {
                    context: graphics_context,
                })
                .add_resource(TextureMapResource {
                    registry: texture_map,
                });
            let mut ecs_state = builder.build();

            info!("Running startup systems...");
            ecs_state.schedules.startup.run(&mut ecs_state.world);

            self.window = Some(window.clone());
            self.ecs_state = Some(ecs_state);
        }
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let Some(ecs_state) = &mut self.ecs_state {
            ecs_state.world.send_event(RawDeviceEvent(event.clone()));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(ecs_state) = &mut self.ecs_state {
            ecs_state.world.send_event(RawWindowEvent(event.clone()));

            match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested, exiting app event loop.");
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    if let Some(mut gfx_res) = ecs_state
                        .world
                        .get_resource_mut::<GraphicsContextResource>()
                    {
                        gfx_res.context.resize(physical_size);
                    }
                }
                WindowEvent::RedrawRequested => {
                    let current_app_state = ecs_state.world.resource::<CurrentState<AppState>>();

                    match current_app_state.val {
                        AppState::Loading => {
                            ecs_state.schedules.loading.run(&mut ecs_state.world);
                        }
                        AppState::Running => {
                            ecs_state.schedules.main.run(&mut ecs_state.world);
                            ecs_state.render_state.apply(&mut ecs_state.world);
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
