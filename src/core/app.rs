use crate::guard;
use crate::{
    ecs::{
        resources::{
            CameraResource, TextureManagerResource, input::InputResource, time::TimeResource,
            window::WindowResource,
        },
        systems::{
            InputSystem, camera_control_system, init_screen_diagnostics_system,
            screen_diagnostics_system, time_system, webgpu_render_system,
        },
    },
    graphics::webgpu_renderer::WebGpuRenderer,
};

use bevy_ecs::prelude::*;
use bevy_ecs::{
    schedule::{Schedule, ScheduleLabel},
    world::World,
};
use tracing::{error, info};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Schedules {
    Startup,
    Main,
    Render,
}

pub struct App<'a> {
    // OS Interactions
    window: Option<&'a Window>,
    input_system: InputSystem,

    // Display logic
    webgpu_renderer: Option<WebGpuRenderer<'a>>,

    // Game Logic
    world: World,
    startup_scheduler: Schedule,
    render_scheduler: Schedule,
    main_scheduler: Schedule,

    startup_done: bool,
    main_done: bool, // just for the first main run
}

impl<'a> App<'a> {
    pub fn new(window: &'a Window) -> Self {
        let mut world = World::new();
        world.insert_resource(InputResource::new());
        world.insert_resource(TimeResource::default());
        world.insert_resource(CameraResource::default());
        world.insert_resource(WindowResource::default());
        world.insert_non_send_resource(TextureManagerResource::default());

        let mut startup_scheduler = Schedule::new(Schedules::Startup);
        startup_scheduler.add_systems((
            // chunk_generation_system,
            // font_loader_system,
            init_screen_diagnostics_system,
        ));

        let mut render_scheduler = Schedule::new(Schedules::Render);
        render_scheduler.add_systems(webgpu_render_system);

        let mut main_scheduler = Schedule::new(Schedules::Main);
        main_scheduler.add_systems((
            time_system.before(screen_diagnostics_system),
            // update_text_mesh_system.before(screen_diagnostics_system),
            screen_diagnostics_system,
            camera_control_system,
        ));

        Self {
            window: Some(window),
            input_system: InputSystem,

            webgpu_renderer: None,

            world: world,
            startup_scheduler,
            render_scheduler,
            main_scheduler,

            // State for schedulers
            startup_done: false,
            main_done: false,
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            info!("App resumed, window should be set by main!");
            // The window is now created in main.rs and passed to App::new.
            // So, self.window should already be Some here.
            // This block should ideally not be reached if the window is always passed.
            // For now, we'll panic if it's None, as it indicates an unexpected state.
            panic!("Window not set in App::resumed!");
        }

        // Initialize WebGpuRenderer
        let window_ref = self.window.unwrap();
        window_ref.set_cursor_visible(false);
        if let Err(err) = window_ref.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
            error!("Failed to grab cursor: {:?}", err);
        }

        // We need to block here because resumed is not async, but WebGpuRenderer::new is.
        // In a real application, you might want to defer this initialization or use a different pattern.
        let webgpu_renderer = pollster::block_on(WebGpuRenderer::new(window_ref));
        self.webgpu_renderer = Some(webgpu_renderer);

        info!("Running startup systems...");
        self.startup_scheduler.run(&mut self.world);
        self.startup_done = true;
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _: StartCause) {
        guard!(self.startup_done);

        self.main_scheduler.run(&mut self.world);

        self.main_done = true;

        if let Some(window) = self.window {
            window.request_redraw(); // begin the drawing loop
        }

        // We run this AFTER the schedule as this is responsible for cleaning up
        // the input system deltas so it makes sense to run it last.
        self.input_system.new_events_hook(&mut self.world);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        guard!(self.startup_done);

        self.input_system.device_event_hook(&mut self.world, &event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        guard!(self.startup_done && self.main_done);

        self.input_system.window_event_hook(&mut self.world, &event);

        match event {
            WindowEvent::CloseRequested => {
                info!("Close button was pressed, exiting.");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                let mut window_size = self.world.resource_mut::<WindowResource>();
                window_size.width = physical_size.width;
                window_size.height = physical_size.height;
                if let Some(renderer) = self.webgpu_renderer.as_mut() {
                    renderer.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.webgpu_renderer.as_mut() {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            renderer.resize(self.window.unwrap().inner_size())
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    self.window.unwrap().request_redraw();
                }
            }
            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // No shader_manager to delete for WebGPU
    }
}
