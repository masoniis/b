use crate::{
    ecs::{
        changed_screen_text_system, init_screen_diagnostics_system, removed_screen_text_system,
        resources::{
            asset_storage::MeshAsset, input::InputResource, time::TimeResource,
            window::WindowResource, AssetStorageResource, CameraResource, CameraUniformResource,
            RenderQueueResource,
        },
        screen_diagnostics_system,
        systems::{
            camera_control_system, chunk_generation_system, mesh_render_system, time_system,
            InputSystem,
        },
    },
    graphics::WebGpuRenderer,
    guard,
};
use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
    system::SystemState,
    world::World,
};
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
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
    renderer: Option<WebGpuRenderer>,
    instance: Option<Instance>,
    surface: Option<Surface<'static>>, // lifetime managed by window Arc
    config: Option<SurfaceConfiguration>,
    adapter: Option<Adapter>,
    device: Option<Arc<Device>>,
    queue: Option<Arc<Queue>>,

    // ECS state required by our main system
    render_state: SystemState<(
        ResMut<'static, RenderQueueResource>,
        Res<'static, AssetStorageResource<MeshAsset>>,
        Res<'static, CameraUniformResource>,
    )>,

    // Game Logic
    world: World,
    startup_scheduler: Schedule,
    main_scheduler: Schedule,

    startup_done: bool, // boolean toggled AFTER the very first (and only) startup schedule run
    main_done: bool,    // boolean toggled AFTER the very first main schedule run
}

impl App {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(InputResource::new());
        world.insert_resource(TimeResource::default());
        world.insert_resource(CameraResource::default());
        world.insert_resource(RenderQueueResource::default());
        world.insert_resource(CameraUniformResource::default());

        world.insert_resource(AssetStorageResource::<MeshAsset>::default());

        let render_state = SystemState::new(&mut world);

        let mut startup_scheduler = Schedule::new(Schedules::Startup);
        startup_scheduler.add_systems((
            chunk_generation_system,
            init_screen_diagnostics_system,
            mesh_render_system.after(chunk_generation_system),
        ));

        let mut main_scheduler = Schedule::new(Schedules::Main);
        main_scheduler.add_systems((
            time_system.before(screen_diagnostics_system),
            screen_diagnostics_system,
            camera_control_system,
            changed_screen_text_system.after(screen_diagnostics_system),
            removed_screen_text_system.after(screen_diagnostics_system),
        ));

        Self {
            // Device and window
            window: None,
            input_system: InputSystem,

            // Webgpu state and renderer
            renderer: None,
            instance: None,
            surface: None,
            config: None,
            adapter: None,
            device: None,
            queue: None,

            // ECS state
            world: world,
            render_state,
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

            // Window creation
            let window_attributes = Window::default_attributes()
                .with_title("🅱️")
                .with_inner_size(LogicalSize::new(1280, 720));

            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("Failed to create window"),
            );

            self.world
                .insert_resource(WindowResource::new(window.inner_size()));

            window.set_cursor_visible(false);
            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                error!("Failed to grab cursor: {:?}", err);
            }

            // Set up wgpu, app holds all of the state
            let (instance, surface, adapter, device, queue, config) = pollster::block_on(async {
                let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

                // I was a bit confused by Arc at first so leaving this here. By cloning
                // the Arc<Window>, we incrememnt a reference counter. When this reference
                // counter is greater than zero, Window will stay alive, but if it hits 0
                // it will be garbage collected. This works asynchronously too, and ensures
                // that our surface's reference to window will remain valid for it's life
                let surface = instance
                    .create_surface(window.clone())
                    .expect("Failed to create surface from window (is the window still alive?)");

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

                let present_mode = if surface_caps
                    .present_modes
                    .contains(&wgpu::PresentMode::Immediate)
                {
                    wgpu::PresentMode::Immediate // uncapped fps
                } else {
                    wgpu::PresentMode::AutoNoVsync // tries to find a no vsync mode but willing to default to whatever is available
                };

                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    width: window.inner_size().width,
                    height: window.inner_size().height,
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                    // Adaptive device-specific config
                    format: surface_format,
                    present_mode,
                    alpha_mode: surface_caps.alpha_modes[0],
                };
                surface.configure(&device, &config);

                debug!(
                    target: "wgpu_init",
                    "\nAdapter: '{}'\n\
                    Backend: {:?}\n\
                    Surface Format: {:?}\n\
                    Present Mode: {:?}\n\
                    Alpha Mode: {:?}\n\
                    Enabled Device Features: {:?}\n\
                    Device Limits: {:#?}",
                    adapter.get_info().name,
                    adapter.get_info().backend,
                    config.format,
                    config.present_mode,
                    config.alpha_mode,
                    device.features(),
                    device.limits()
                );

                (
                    instance,
                    surface,
                    adapter,
                    Arc::new(device),
                    Arc::new(queue),
                    config,
                )
            });

            // --- 3. Create the Decoupled Renderer ---
            let webgpu_renderer =
                crate::graphics::WebGpuRenderer::new(device.clone(), queue.clone(), &config);
            self.renderer = Some(webgpu_renderer);

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

                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(physical_size);
                }
            }

            WindowEvent::RedrawRequested => {
                let App {
                    surface,
                    device,
                    config,
                    window,
                    ..
                } = self;

                let surface = surface.as_ref().unwrap();

                // Handle potential surface errors
                let output = match surface.get_current_texture() {
                    Ok(output) => output,
                    Err(wgpu::SurfaceError::Lost) => {
                        warn!("WGPU SurfaceError::Lost, reconfiguring surface.");
                        let size = window.as_ref().unwrap().inner_size();
                        if let (Some(config), Some(device)) = (config.as_mut(), device.as_ref()) {
                            config.width = size.width;
                            config.height = size.height;
                            surface.configure(device, config);
                        }
                        window.as_ref().unwrap().request_redraw(); // try again with a new frame
                        return;
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("WGPU SurfaceError::OutOfMemory, exiting event loop.");
                        event_loop.exit();
                        return;
                    }
                    Err(e) => {
                        eprintln!("Error acquiring next texture: {:?}", e);
                        return;
                    }
                };

                // Get all the resources needed for rendering from the ecs world
                let (render_queue, mesh_assets, camera_uniform) =
                    self.render_state.get_mut(&mut self.world);

                // Get the view and render
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                if let Err(e) = self
                    .renderer
                    .as_mut()
                    .expect("WebGPU Renderer missing")
                    .render(&view, &render_queue, &mesh_assets, &camera_uniform)
                {
                    eprintln!("Renderer error: {:?}", e);
                }

                // Apply any deferred changes.
                self.render_state.apply(&mut self.world);

                output.present();
                window.as_ref().unwrap().request_redraw();
            }
            _ => { /* pass rest of events */ }
        }
    }
}
