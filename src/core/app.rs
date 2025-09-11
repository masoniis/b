use crate::core::input::InputSystem;
use crate::ecs::resources::{Camera, DeltaTimeResource, InputResource, WindowResource};
use crate::ecs::systems::camera_system::CameraSystem;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use glam::Vec2;
use shred::World;
use shred::{Dispatcher, DispatcherBuilder};
use std::time::Instant;
use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

// The App struct is now generic over the lifetimes 'a and 'b required by the Dispatcher.
pub struct App<'a, 'b> {
    // OS Interactions
    window: Option<Window>,
    last_frame_time: Instant,

    // Display logic
    renderer: Option<Renderer>,
    shader_program: Option<ShaderProgram>,

    // ECS and Game Logic
    world: World,
    // The `systems` vector is replaced with specific handlers and a dispatcher.
    input_system: InputSystem,
    game_logic_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::empty();
        world.insert(InputResource::new());
        world.insert(DeltaTimeResource::default());
        world.insert(Camera::default());
        world.insert(WindowResource::default());

        // The dispatcher is built once, when the app is created.
        // All data-driven systems are registered here.
        let dispatcher = DispatcherBuilder::new()
            .with(CameraSystem, "camera_system", &[])
            .build();

        Self {
            window: None,
            world: world,
            input_system: InputSystem,
            game_logic_dispatcher: dispatcher,
            renderer: None,
            shader_program: None,
            last_frame_time: Instant::now(),
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
                // 2. Update DeltaTime resource
                let current_time = Instant::now();
                let delta_time = current_time
                    .duration_since(self.last_frame_time)
                    .as_secs_f32();
                self.last_frame_time = current_time;
                self.world.fetch_mut::<DeltaTimeResource>().seconds = delta_time;

                // 3. Run all data-driven systems in parallel (where possible)
                self.game_logic_dispatcher.dispatch(&mut self.world);

                // 4. Render the scene
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
