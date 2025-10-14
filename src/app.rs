use crate::{
    prelude::*,
    render_world::{
        context::GraphicsContext, extract::run_extract_schedule, RenderSchedule,
        RenderWorldInterface,
    },
    simulation_world::{
        app_lifecycle::AppState,
        build_simulation_world, configure_simulation_world,
        input::events::{RawDeviceEvent, RawWindowEvent},
        SimulationSchedule, SimulationWorldInterface,
    },
};
use std::{error::Error, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

/// The main application container, responsible for orchestrating OS
/// events as well as the creation and scheduling of the ECS worlds.
pub struct App {
    // Window is an Arc because the surface created by wgpu needs to hold
    // a window reference with a static lifetime (like Arc) for safety.
    window: Option<Arc<Window>>,

    simulation_world: Option<SimulationWorldInterface>,
    render_world: Option<RenderWorldInterface>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            simulation_world: None,
            render_world: None,
        }
    }

    /// Simple utility method to spin up an event loop and run a default app
    pub fn create_and_run() -> Result<(), Box<dyn Error>> {
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

            let mut simulation_world =
                build_simulation_world(configure_simulation_world(texture_map, &window));
            let mut render_world = RenderWorldInterface::new(graphics_context);

            info!("Running startup systems...\n\n\n");
            simulation_world.run_schedule(SimulationSchedule::Startup);
            render_world.run_schedule(RenderSchedule::Startup); // TODO: Async handling for loading screen?

            self.window = Some(window.clone());
            self.simulation_world = Some(simulation_world);
            self.render_world = Some(render_world);
        }
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let Some(simulation_world) = &mut self.simulation_world {
            simulation_world.send_event(RawDeviceEvent(event.clone()));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(simulation_world) = &mut self.simulation_world {
            simulation_world.send_event(RawWindowEvent(event.clone()));

            // NOTE: The events handled here should only be events that rely on the event loop
            // or window. Any other event should be fine to handle within the ECS world itself.
            match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested, exiting app event loop.");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    if let (Some(simulation_world), Some(render_world)) =
                        (self.simulation_world.as_mut(), self.render_world.as_mut())
                    {
                        let current_app_state = simulation_world.get_app_state();

                        match current_app_state {
                            AppState::Loading => {
                                simulation_world.run_schedule(SimulationSchedule::Loading);
                            }
                            AppState::Running => {
                                // We run the main schedule regardless of running state
                                // Not sure yet if this case will need anything
                            }
                            AppState::Closing => {
                                // Save data or something
                            }
                        };

                        simulation_world.run_schedule(SimulationSchedule::Main);

                        run_extract_schedule(
                            simulation_world.borrow(),
                            render_world.borrow(),
                            RenderSchedule::Extract,
                        );

                        simulation_world.clear_trackers();

                        // TODO: These schedules can run in parallel with the next frame of the game (in theory)

                        render_world.run_schedule(RenderSchedule::Prepare);
                        render_world.run_schedule(RenderSchedule::Queue);
                        render_world.run_schedule(RenderSchedule::Render);

                        render_world.clear_trackers();

                        // Request the next frame
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    } else {
                        warn!(
                            "Redraw requested but simulation or render world is not initialized."
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
