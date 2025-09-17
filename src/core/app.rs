use crate::guard;
use crate::{
    ecs::{
        resources::{
            input::InputResource, time::TimeResource, window::WindowResource, CameraResource,
            ShaderManagerResource, TextureManagerResource,
        },
        systems::{
            begin_frame_system, camera_control_system, chunk_init_system, finish_frame_system,
            font_loader_system, render_scene_system, render_text_system, screen_diagnostics_system,
            time_system, update_text_mesh_system, InputSystem,
        },
    },
    graphics::renderer::Renderer,
};

use bevy_ecs::prelude::*;
use bevy_ecs::{
    schedule::{Schedule, ScheduleLabel},
    world::World,
};
use glam::Vec2;
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

pub struct App {
    // OS Interactions
    window: Option<Window>,
    input_system: InputSystem,

    // Display logic
    renderer: Option<Renderer>,
    shader_manager: Option<ShaderManagerResource>,

    // Game Logic
    world: World,
    startup_scheduler: Schedule,
    render_scheduler: Schedule,
    main_scheduler: Schedule,

    startup_done: bool,
    main_done: bool, // just for the first main run
}

impl App {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(InputResource::new());
        world.insert_resource(TimeResource::default());
        world.insert_resource(CameraResource::default());
        world.insert_resource(WindowResource::default());
        world.insert_non_send_resource(TextureManagerResource::default());

        let mut startup_scheduler = Schedule::new(Schedules::Startup);
        startup_scheduler.add_systems((chunk_init_system, font_loader_system).chain());

        let mut render_scheduler = Schedule::new(Schedules::Render);
        render_scheduler.add_systems(
            (
                begin_frame_system,
                render_scene_system,
                render_text_system,
                finish_frame_system,
            )
                .chain(),
        );

        let mut main_scheduler = Schedule::new(Schedules::Main);
        main_scheduler.add_systems((
            time_system.before(screen_diagnostics_system),
            update_text_mesh_system.before(screen_diagnostics_system),
            screen_diagnostics_system,
            camera_control_system,
        ));

        Self {
            window: None,
            input_system: InputSystem,

            renderer: None,
            shader_manager: None,

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

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            info!("App resumed, creating window!");
            let (window, gl_surface, gl_context) =
                crate::core::window::create_gl_window(event_loop, Vec2::new(1800.0, 1500.0));

            self.window = Some(window);

            if let Some(window_ref) = self.window.as_ref() {
                window_ref.set_cursor_visible(false);
                if let Err(err) =
                    window_ref.set_cursor_grab(winit::window::CursorGrabMode::Confined)
                {
                    error!("Failed to grab cursor: {:?}", err);
                }
            }

            let shader_manager =
                ShaderManagerResource::new().expect("Failed to create ShaderManager!!!");

            let renderer = Renderer::new(gl_surface, gl_context);

            self.shader_manager = Some(shader_manager);
            self.renderer = Some(renderer);

            info!("Running startup systems...");
            self.startup_scheduler.run(&mut self.world);
            self.startup_done = true;
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _: StartCause) {
        guard!(self.startup_done);

        self.main_scheduler.run(&mut self.world);

        self.main_done = true;

        if let Some(window) = self.window.as_ref() {
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
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(renderer), Some(shader_manager)) = (
                    self.window.as_ref(),
                    self.renderer.take(),
                    self.shader_manager.take(),
                ) {
                    // 1. Temporarily insert the main-thread data as NonSend resources
                    self.world.insert_non_send_resource(renderer);
                    self.world.insert_non_send_resource(shader_manager);

                    // 2. Run the render schedule. Bevy will pass the resources to the system.
                    self.render_scheduler.run(&mut self.world);

                    // 3. Remove the resources and give them back to the App.
                    self.renderer = self.world.remove_non_send_resource::<Renderer>();
                    self.shader_manager = self
                        .world
                        .remove_non_send_resource::<ShaderManagerResource>();

                    // (request redraw to keep loop running)
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(shader_manager) = &self.shader_manager {
            shader_manager.delete();
        }
    }
}
