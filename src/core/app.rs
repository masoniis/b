use crate::{
    core::graphics::context::GraphicsContext,
    ecs_resources::window::WindowResource,
    ecs_systems::{EcsState, InputSystem},
    prelude::*,
};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

// An enum to represent the state of our app, much cleaner than a boolean.
pub enum AppState {
    Running,
    Paused,
}

/// The main application struct, responsible for orchestrating the event loop,
/// ECS, and graphics context.
pub struct App {
    // --- FIELDS ARE NO LONGER OPTIONAL ---
    // All initialization is done in `new_async` before this struct is created.
    window: Arc<Window>,
    input_system: InputSystem,
    graphics_context: GraphicsContext,
    ecs_state: EcsState,
    state: AppState,
}

impl App {
    // --- NEW ASYNC CONSTRUCTOR ---
    // This is where all setup, including async setup, now happens.
    pub async fn new_async(window: Arc<Window>) -> Self {
        info!("App starting, creating renderer...");

        let mut ecs_state = EcsState::new();
        ecs_state
            .world
            .insert_resource(WindowResource::new(window.inner_size()));

        window.set_cursor_visible(false);
        if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
            error!("Failed to grab cursor: {:?}", err);
        }

        // --- ASYNC CALL ---
        // We now `.await` the async function directly, no more `pollster::block_on`.
        let graphics_context = GraphicsContext::new(window.clone()).await;

        info!("Running startup systems...");
        ecs_state.run_startup();

        Self {
            window,
            input_system: InputSystem,
            graphics_context,
            ecs_state,
            state: AppState::Running,
        }
    }
}

// --- SIMPLIFIED ApplicationHandler ---
// Its only job now is to handle events for an already-initialized app.
impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // This is intentionally left empty.
        // All initialization is now handled in `App::new_async` before the event loop starts.
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _: StartCause) {
        // We can now use our state machine to control logic flow.
        match self.state {
            AppState::Running => {
                self.ecs_state.run_main();

                // Clearing inputs happens after they've been processed by the main systems.
                self.input_system.new_events_hook(&mut self.ecs_state.world);

                self.window.request_redraw();
            }
            AppState::Paused => {
                // When paused, we might still want to clear inputs for a pause menu.
                self.input_system.new_events_hook(&mut self.ecs_state.world);
            }
        }
    }

    fn device_event(
        &mut self,
        _el: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        self.input_system
            .device_event_hook(&mut self.ecs_state.world, &event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Let the input system process the event first. It won't consume it.
        self.input_system
            .window_event_hook(&mut self.ecs_state.world, &event);

        match event {
            WindowEvent::CloseRequested => {
                info!("Window close requested, exiting app event loop.");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                let mut window_resource = self.ecs_state.world.resource_mut::<WindowResource>();
                window_resource.width = physical_size.width;
                window_resource.height = physical_size.height;
                self.graphics_context.resize(physical_size);
            }
            // Example of using the state machine
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => match self.state {
                AppState::Running => {
                    info!("Pausing game.");
                    self.state = AppState::Paused;
                }
                AppState::Paused => {
                    info!("Resuming game.");
                    self.state = AppState::Running;
                }
            },
            WindowEvent::RedrawRequested => {
                let (render_queue, mesh_assets, camera_uniform) = self
                    .ecs_state
                    .render_state
                    .get_mut(&mut self.ecs_state.world);

                match self
                    .graphics_context
                    .render(&render_queue, &mesh_assets, &camera_uniform)
                {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        warn!("WGPU SurfaceError::Lost, resizing surface.");
                        let size = self.window.inner_size();
                        self.graphics_context.resize(size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("WGPU SurfaceError::OutOfMemory, exiting event loop.");
                        event_loop.exit();
                    }
                    Err(e) => error!("Error during render: {:?}", e),
                }

                self.ecs_state.render_state.apply(&mut self.ecs_state.world);
                self.window.request_redraw();
            }
            _ => {}
        }
    }
}

// --- NEW TOP-LEVEL RUNNER FUNCTION ---
// This is the shared entry point for both native and web.
pub async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let mut window_attributes = Window::default_attributes()
        .with_title("🅱️")
        .with_inner_size(LogicalSize::new(1280, 720));

    // --- WEB-SPECIFIC CANVAS ATTACHMENT ---
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::wasm_bindgen::{prelude::*, JsCast};
        let web_window = web_sys::window().unwrap();
        let document = web_window.document().unwrap();
        // Use a CSS selector to find your canvas element
        let canvas = document
            .get_element_by_id("wgpu-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        window_attributes = window_attributes.with_canvas(Some(canvas));
    }

    let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

    let mut app = App::new_async(window).await;

    // `run_app` is a winit feature that handles the event loop differences
    // between native and web automatically. It blocks on native and returns on web.
    event_loop.run_app(&mut app).unwrap();
}
