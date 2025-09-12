use crate::core::input::InputSystem;
use crate::ecs::resources::{Camera, InputResource, TimeResource, WindowResource};
use crate::ecs::systems::camera_control_system::CameraControlSystem;
use crate::ecs::systems::time_system::TimeSystem;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use glam::Vec2;
use shred::World;
use shred::{Dispatcher, DispatcherBuilder};
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App<'a, 'b> {
    // OS Interactions
    window: Option<Window>,

    // Display logic
    renderer: Option<Renderer>,
    shader_program: Option<ShaderProgram>,

    // ECS and Game Logic
    input_system: InputSystem,
    world: World,
    game_logic_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::empty();
        world.insert(InputResource::new());
        world.insert(TimeResource::default());
        world.insert(Camera::default());
        world.insert(WindowResource::default());

        // All data-driven systems are registered here.
        let dispatcher = DispatcherBuilder::new()
            .with(TimeSystem, "time_system", &[])
            .with(
                CameraControlSystem,
                "camera_control_system",
                &["time_system"],
            )
            .build();

        Self {
            window: None,
            world: world,
            input_system: InputSystem,
            game_logic_dispatcher: dispatcher,
            renderer: None,
            shader_program: None,
        }
    }
}

impl<'a, 'b> ApplicationHandler for App<'a, 'b> {
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

            // The Renderer and ShaderProgram are now stored on the App struct,
            // NOT in the ECS World. This avoids the Send + Sync issue.
            self.shader_program = Some(
                ShaderProgram::new(
                    "src/assets/shaders/simple.vert",
                    "src/assets/shaders/triangle.frag",
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
                let mut window_size = self.world.fetch_mut::<WindowResource>();
                window_size.width = physical_size.width;
                window_size.height = physical_size.height;
            }
            WindowEvent::RedrawRequested => {
                // Run all data-driven systems in parallel
                self.game_logic_dispatcher.dispatch(&mut self.world);

                // Render the scene
                let camera = self.world.fetch::<Camera>();

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
