use crate::core::input::InputSystem;
use crate::ecs::resources::{Camera, InputResource, TimeResource, WindowResource};
use crate::ecs::systems::camera_control_system::camera_control_system;
use crate::ecs::systems::time_system::time_system;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;
use glam::Vec2;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    // OS Interactions
    window: Option<Window>,

    // Display logic
    renderer: Option<Renderer>,
    shader_program: Option<ShaderProgram>,

    // ECS and Game Logic
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

        let mut schedule = Schedule::default();
        schedule.add_systems((time_system, camera_control_system));

        Self {
            window: None,
            world: world,
            input_system: InputSystem,
            schedule: schedule,
            renderer: None,
            shader_program: None,
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

            self.shader_program = Some(
                ShaderProgram::new(
                    "src/assets/shaders/simple.vert",
                    "src/assets/shaders/simple.frag",
                )
                .unwrap(),
            );
            self.renderer = Some(Renderer::new(gl_surface, gl_context));
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _: StartCause) {
        self.input_system.new_events_hook(&mut self.world);
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
                // Run all data-driven systems in parallel
                self.schedule.run(&mut self.world);

                // Render the scene
                let camera = self.world.resource::<Camera>();

                if let (Some(window), Some(renderer), Some(shader_program)) = (
                    self.window.as_ref(),
                    &mut self.renderer,
                    &mut self.shader_program,
                ) {
                    renderer.set_frame(shader_program, &camera);
                    shader_program.set_mat4("modelView", &camera.get_view_matrix());
                    shader_program.set_mat4("projection", &camera.get_projection_matrix());
                    window.request_redraw();
                }
            }
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
