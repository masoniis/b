use crate::core::input::InputSystem;
use crate::ecs::components::{Mesh, Transform};
use crate::ecs::resources::{Camera, InputResource, TimeResource, WindowResource};
use crate::ecs::systems::camera_control_system::camera_control_system;
use crate::ecs::systems::render_system::render_system;
use crate::ecs::systems::time_system::time_system;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use bevy_ecs::schedule::{Schedule, ScheduleLabel, SystemSet};
use bevy_ecs::world::World;
use glam::Vec2;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Schedules {
    Startup,
    Main,
    Render,
}

pub struct App {
    // OS Interactions
    window: Option<Window>,

    // Display logic
    renderer: Option<Renderer>,
    shader_program: Option<ShaderProgram>,
    render_schedule: Schedule,

    // Game Logic
    input_system: InputSystem,
    world: World,
    schedule: Schedule,
}

impl App {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(InputResource::new());
        world.insert_resource(TimeResource::default());
        world.insert_resource(Camera::default());
        world.insert_resource(WindowResource::default());

        // Spawn a basic entity
        world.spawn((
            Mesh {
                vertices: vec![
                    0.5, 0.9, 0.0, // top right
                    0.5, -0.5, 0.0, // bottom right
                    -0.5, -0.5, 0.0, // bottom left
                ],
                indices: vec![0, 1, 2],
            },
            Transform::default(),
        ));

        let mut schedule = Schedule::default();
        schedule.add_systems((time_system, camera_control_system));

        let mut render_schedule = Schedule::new(Schedules::Render); // Use a label for clarity
        render_schedule.add_systems(render_system);

        Self {
            window: None,
            world: world,
            input_system: InputSystem,
            schedule: schedule,
            renderer: None,
            shader_program: None,
            render_schedule: render_schedule,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            info!("App resumed, creating window!");
            let (window, gl_surface, gl_context) =
                crate::core::window::create_gl_window(event_loop, Vec2::new(800.0, 500.0));

            self.window = Some(window);

            if let Some(window_ref) = self.window.as_ref() {
                window_ref.set_cursor_visible(false);
                if let Err(err) =
                    window_ref.set_cursor_grab(winit::window::CursorGrabMode::Confined)
                {
                    info!("Failed to grab cursor: {:?}", err);
                }
            }

            let shader_program = ShaderProgram::new(
                "src/assets/shaders/simple.vert",
                "src/assets/shaders/simple.frag",
            )
            .unwrap();
            let renderer = Renderer::new(gl_surface, gl_context);

            self.shader_program = Some(shader_program);
            self.renderer = Some(renderer);
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _: StartCause) {
        self.input_system.new_events_hook(&mut self.world);

        self.schedule.run(&mut self.world);

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        self.input_system.device_event_hook(&mut self.world, &event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
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
                if let (Some(window), Some(renderer), Some(shader_program)) = (
                    self.window.as_ref(),
                    // Take ownership from from App for renderer and shader_program
                    self.renderer.take(),
                    self.shader_program.take(),
                ) {
                    // 1. Temporarily insert the main-thread data as NonSend resources
                    self.world.insert_non_send_resource(renderer);
                    self.world.insert_non_send_resource(shader_program);

                    // 2. Run the render schedule. Bevy will pass the resources to the system.
                    self.render_schedule.run(&mut self.world);

                    // 3. Remove the resources and give them back to the App.
                    // This is crucial for the next frame.
                    self.renderer = self.world.remove_non_send_resource::<Renderer>();
                    self.shader_program = self.world.remove_non_send_resource::<ShaderProgram>();

                    // You would swap buffers here, then request the next redraw
                    window.request_redraw();
                }
            }
            // WindowEvent::RedrawRequested => {
            //     // Run all data-driven systems in parallel
            //     self.schedule.run(&mut self.world);
            //
            //     if let (Some(window), Some(renderer), Some(shader_program)) = (
            //         self.window.as_ref(),
            //         &mut self.renderer,
            //         &mut self.shader_program,
            //     ) {
            //         render_system(&mut self.world, renderer, shader_program);
            //         window.request_redraw();
            //     }
            // }
            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(renderer), Some(shader_program)) = (&self.renderer, &self.shader_program) {
            renderer.cleanup();
            shader_program.delete();
        }
    }
}
