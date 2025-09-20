use crate::{
    ecs::{
        resources::{
            CameraResource,
            // TextureManagerResource,
            input::InputResource,
            time::TimeResource,
            window::WindowResource,
        },
        systems::{
            InputSystem, camera_control_system, chunk_generation_system,
            clear_previous_frame_system, init_screen_diagnostics_system, mesh_render_system,
            screen_diagnostics_system, time_system, triangle_render_system,
        },
    },
    graphics::webgpu_renderer::WebGpuRenderer,
    guard,
};
use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
    world::World,
};
use std::sync::Arc;
use tracing::{error, info};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Schedules {
    Startup,
    Main,
}

pub struct App {
    // OS Interactions
    window: Option<Arc<Window>>,
    input_system: InputSystem,

    // Display logic
    instance: Option<Instance>,
    surface: Option<Surface<'static>>, // lifetime managed by window Arc
    config: Option<SurfaceConfiguration>,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,

    // Game Logic
    world: World,
    startup_scheduler: Schedule,
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
        // world.insert_non_send_resource(TextureManagerResource::default());

        let mut startup_scheduler = Schedule::new(Schedules::Startup);
        startup_scheduler.add_systems((
            chunk_generation_system,
            // font_loader_system,
            init_screen_diagnostics_system,
            // triangle_render_system,
            mesh_render_system.after(chunk_generation_system),
        ));

        let mut main_scheduler = Schedule::new(Schedules::Main);
        main_scheduler.add_systems((
            time_system.before(screen_diagnostics_system),
            // update_text_mesh_system.before(screen_diagnostics_system),
            screen_diagnostics_system,
            camera_control_system,
            // clear_previous_frame_system,
            // triangle_render_system.after(clear_previous_frame_system),
            // mesh_render_system.after(clear_previous_frame_system),
        ));

        Self {
            // Device and window
            window: None,
            input_system: InputSystem,

            // Webgpu state and renderer
            instance: None,
            surface: None,
            config: None,
            adapter: None,
            device: None,
            queue: None,

            // ECS state
            world: world,
            startup_scheduler,
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
            info!("App resumed, creating window and renderer...");

            // --- 1. Window Creation ---
            let window_attributes = Window::default_attributes()
                .with_title("🅱️")
                .with_inner_size(PhysicalSize::new(1800, 1500));
            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("Failed to create window"),
            );

            window.set_cursor_visible(false);
            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                error!("Failed to grab cursor: {:?}", err);
            }

            // --- 2. WGPU Initialization ---
            // We do all the async setup in this block and wait for it to finish.
            let (instance, surface, adapter, device, queue, config) = pollster::block_on(async {
                // The instance is the entry point to WGPU
                let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

                // The surface is the part of the window we draw to.
                // By using an Arc<Window>, we can create the surface safely.
                let surface = instance.create_surface(window.clone()).unwrap();

                // The adapter is a handle to a physical graphics card.
                let adapter = instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::default(),
                        compatible_surface: Some(&surface),
                        force_fallback_adapter: false,
                    })
                    .await
                    .unwrap();

                // The device and queue are our primary interface to the GPU.
                let (device, queue) = adapter
                    .request_device(&wgpu::DeviceDescriptor::default())
                    .await
                    .unwrap();

                // The surface configuration defines how the surface creates its textures.
                let surface_caps = surface.get_capabilities(&adapter);
                let surface_format = surface_caps
                    .formats
                    .iter()
                    .copied()
                    .find(|f| f.is_srgb())
                    .unwrap_or(surface_caps.formats[0]);
                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: surface_format,
                    width: window.inner_size().width,
                    height: window.inner_size().height,
                    present_mode: wgpu::PresentMode::Fifo,
                    alpha_mode: surface_caps.alpha_modes[0],
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                };
                surface.configure(&device, &config);

                (instance, surface, adapter, device, queue, config)
            });

            // --- 3. Create the Decoupled Renderer ---
            let webgpu_renderer = WebGpuRenderer::new(device.clone(), queue.clone(), &config);
            self.world.insert_resource(webgpu_renderer);

            // --- 4. Store Everything in self ---
            self.window = Some(window);
            self.instance = Some(instance);
            self.surface = Some(surface);
            self.adapter = Some(adapter);
            self.config = Some(config);
            self.device = Some(device);
            self.queue = Some(queue);

            info!("Running startup systems...");
            self.startup_scheduler.run(&mut self.world);
            self.startup_done = true;
        }
    }
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _: StartCause) {
        guard!(self.startup_done);

        self.main_scheduler.run(&mut self.world);

        self.main_done = true;

        if let Some(window) = &self.window {
            window.request_redraw(); // begin the drawing loop
        }

        // We run this AFTER the schedule as this is responsible for cleaning up
        // the input system deltas (if we run it first, input would always be empty)
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
                info!("Window close requested, exiting app event loop.");
                event_loop.exit();
            }

            WindowEvent::Resized(physical_size) => {
                if physical_size.width > 0 && physical_size.height > 0 {
                    // Update the ECS resource for window size
                    let mut window_size = self.world.resource_mut::<WindowResource>();
                    window_size.width = physical_size.width;
                    window_size.height = physical_size.height;

                    // Update the surface configuration for webgpu
                    if let (Some(config), Some(surface), Some(device)) = (
                        self.config.as_mut(),
                        self.surface.as_ref(),
                        self.device.as_ref(),
                    ) {
                        config.width = physical_size.width;
                        config.height = physical_size.height;
                        surface.configure(device, config);
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                let surface = self.surface.as_ref().unwrap();
                let renderer = self.world.get_resource_mut::<WebGpuRenderer>().unwrap();

                match surface.get_current_texture() {
                    Ok(output) => {
                        let view = output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        // Call the renderer's updated render method, passing the texture view
                        if let Err(e) = renderer.render(&view) {
                            eprintln!("Renderer error: {:?}", e);
                        }

                        // Present the frame to the screen
                        output.present();
                    }
                    Err(wgpu::SurfaceError::Lost) => {
                        // This means the surface is outdated and needs to be reconfigured.
                        let size = self.window.as_ref().unwrap().inner_size();
                        if let (Some(config), Some(surface), Some(device)) = (
                            self.config.as_mut(),
                            self.surface.as_ref(),
                            self.device.as_ref(),
                        ) {
                            config.width = size.width;
                            config.height = size.height;
                            surface.configure(device, config);
                        }
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("WGPU SurfaceError::OutOfMemory, exiting.");
                        event_loop.exit();
                    }
                    Err(e) => {
                        eprintln!("Error acquiring next texture: {:?}", e);
                    }
                }
                // After drawing, request another redraw to keep the animation loop going.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
