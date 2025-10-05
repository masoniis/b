use crate::{
    game_world::{
        app_lifecycle::AppState,
        global_resources::{texture_map::TextureMapResource, window::WindowResource},
        input::events::{RawDeviceEvent, RawWindowEvent},
        schedules::GameSchedule,
    },
    prelude::*,
    render_world::{
        context::GraphicsContext,
        extract::utils::run_extract_schedule::initialize_main_world_for_extract,
    },
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

use crate::game_world;
use crate::game_world::GameWorldInterface;
use crate::render_world::extract::utils::run_extract_schedule;
use crate::render_world::{
    build_render_world, configure_render_world, RenderSchedule, RenderWorldInterface,
};

/// The main application struct, responsible for orchestrating window events
/// as well as the scheduling of the main ECS systems.
/// ECS, and graphics context.
pub struct App {
    // OS and Winit State
    window: Option<Arc<Window>>,

    // Core Engine Modules
    game_world: Option<GameWorldInterface>,
    render_world: Option<RenderWorldInterface>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            game_world: None,
            render_world: None,
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

            let mut builder = game_world::configure_game_world();
            builder
                .add_resource(WindowResource::new(window.inner_size()))
                .add_resource(TextureMapResource {
                    registry: texture_map,
                });
            let mut game_world = game_world::build_game_world(builder);
            initialize_main_world_for_extract(game_world.borrow());

            info!("Running startup systems...\n\n\n");
            game_world.run_schedule(GameSchedule::Startup);

            let render_builder = configure_render_world();
            let render_world = build_render_world(render_builder, graphics_context);

            self.window = Some(window.clone());
            self.game_world = Some(game_world);
            self.render_world = Some(render_world);
        }
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let Some(game_world) = &mut self.game_world {
            game_world.send_event(RawDeviceEvent(event.clone()));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(game_world) = &mut self.game_world {
            game_world.send_event(RawWindowEvent(event.clone()));

            // NOTE: The events handled here should only be events that rely on the event loop
            // itself. Any other event should be fine to handle within the ECS world itself.
            match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested, exiting app event loop.");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    if let (Some(game_world), Some(render_world)) =
                        (self.game_world.as_mut(), self.render_world.as_mut())
                    {
                        let current_app_state = game_world.get_app_state();

                        match current_app_state {
                            AppState::Loading => {
                                game_world.run_schedule(GameSchedule::Loading);
                            }
                            AppState::Running => {
                                game_world.run_schedule(GameSchedule::Main);
                            }
                            AppState::Closing => {}
                        };

                        run_extract_schedule(
                            game_world.borrow(),
                            render_world.borrow(),
                            RenderSchedule::Extract,
                        );

                        // TODO: These schedules can run in parallel with the next frame of the game
                        render_world.run_schedule(RenderSchedule::Prepare);
                        render_world.run_schedule(RenderSchedule::Queue);
                        render_world.run_schedule(RenderSchedule::Render);

                        // Request the next frame
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    } else {
                        warn!("Redraw requested but game or render world is not initialized.");
                    }
                }
                _ => {}
            }
        }
    }
}
