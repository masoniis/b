use crate::{
    ecs_core::{async_loading::LoadingTracker, frame_sync::FrameSync},
    prelude::*,
    render_world::{
        global_extract::utils::run_extract_schedule, graphics_context::GraphicsContext,
        scheduling::RenderSchedule, textures::load_texture_array, RenderWorldInterface,
    },
    simulation_world::{
        input::events::{RawDeviceEvent, RawWindowEvent},
        SimulationSchedule, SimulationWorldInterface,
    },
};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{CursorGrabMode, Window, WindowId},
};

/// The main application container, responsible for orchestrating OS
/// events as well as the creation and scheduling of the ECS worlds.
pub struct App {
    // Window is an Arc because the surface created by wgpu needs to hold
    // a window reference with a static lifetime (like Arc) for safety.
    window: Option<Arc<Window>>,

    // The worlds
    simulation_world: Option<Arc<Mutex<SimulationWorldInterface>>>,
    render_world: Option<Arc<Mutex<RenderWorldInterface>>>,
    // + a loading tracker to orchestrate async tasks between the two worlds
    loading_tracker: LoadingTracker,

    // World parallelization
    frame_sync: FrameSync,
    render_thread: Option<JoinHandle<()>>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            simulation_world: None,
            render_world: None,
            loading_tracker: LoadingTracker::default(),
            frame_sync: FrameSync::new(),
            render_thread: None,
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

            // INFO: --------------------------
            //         Setup the window
            // --------------------------------

            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("🅱️")
                            .with_inner_size(LogicalSize::new(1280, 720)),
                    )
                    .unwrap(),
            );

            window.set_cursor_visible(false);
            if let Err(err) = window.set_cursor_grab(CursorGrabMode::Confined) {
                error!("Failed to grab cursor: {:?}", err);
            }

            // INFO: ----------------------------------------
            //         Create and initiate the worlds
            // ----------------------------------------------

            // world dependencies that the app must create (due to window)
            let graphics_context = pollster::block_on(GraphicsContext::new(window.clone()));
            let (texture_array, texture_registry) =
                load_texture_array(&graphics_context.device, &graphics_context.queue).unwrap();

            let mut simulation_world = SimulationWorldInterface::new(&window, texture_registry);
            let mut render_world = RenderWorldInterface::new(graphics_context, texture_array);

            simulation_world.add_resource(self.loading_tracker.clone());
            render_world.add_resource(self.loading_tracker.clone());

            info!("Running startup systems...\n\n\n");
            simulation_world.run_schedule(SimulationSchedule::Startup);
            render_world.run_schedule(RenderSchedule::Startup);

            // INFO: ----------------------------------------------
            //         Spawn and setup the rendering thread
            // ----------------------------------------------------

            let render_world = Arc::new(Mutex::new(render_world));
            let simulation_world = Arc::new(Mutex::new(simulation_world));

            let render_sync = self.frame_sync.clone();
            let render_world_for_render = render_world.clone();
            let sim_world_for_render = simulation_world.clone();

            let render_thread = thread::spawn(move || {
                let _span = info_span!("Render thread").entered();
                loop {
                    // wait until we are signaled to extract form the sim world

                    render_sync.wait_for_extraction();
                    {
                        let mut sim_guard = sim_world_for_render.lock().unwrap();
                        let mut render_guard = render_world_for_render.lock().unwrap();

                        let _extract_phase_span = tracing::info_span!("extract_schedule").entered();
                        // the special extract schedule needs mutable access to the simulation world
                        run_extract_schedule(
                            &mut sim_guard.borrow(),
                            &mut render_guard.borrow(),
                            RenderSchedule::Extract,
                        );

                        sim_guard.clear_trackers();
                    }
                    render_sync.finish_extraction();

                    // perform rendering now that sim is active again

                    let mut render_world = render_world_for_render.lock().unwrap();
                    {
                        let _render_phase_span =
                            tracing::info_span!("main_render_schedule").entered();
                        render_world.run_schedule(RenderSchedule::Main);
                        render_world.clear_trackers();
                    }
                }
            });

            // INFO: ------------------------------
            //         Update the App state
            // ------------------------------------

            self.window = Some(window.clone());
            self.simulation_world = Some(simulation_world);
            self.render_world = Some(render_world);
            self.render_thread = Some(render_thread);
        }
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let Some(simulation_world) = &mut self.simulation_world {
            simulation_world
                .lock()
                .unwrap()
                .send_event(RawDeviceEvent(event.clone()));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(simulation_world) = &mut self.simulation_world {
            simulation_world
                .lock()
                .unwrap()
                .send_event(RawWindowEvent(event.clone()));

            // NOTE: The events handled here should only be events that rely on the event loop
            // or window. Any other event should be fine to handle within the ECS world itself.
            match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested, exiting app event loop.");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    if let Some(simulation_world) = self.simulation_world.as_mut() {
                        // wait for the sim signal and run
                        self.frame_sync.wait_for_simulation();
                        {
                            let _main_loop_span =
                                tracing::info_span!("main_simulation_schedule").entered();
                            simulation_world
                                .lock()
                                .unwrap()
                                .run_schedule(SimulationSchedule::Main);
                        }
                        self.frame_sync.finish_simulation();

                        // request the next frame
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
