use crate::prelude::*;
use std::sync::Arc;
use wgpu::{
    Adapter, Device, DeviceDescriptor, Instance, InstanceDescriptor, PowerPreference, PresentMode,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration,
};
use winit::window::Window;

/// A container for the core WGPU state and the renderer that the app holds.
pub struct GraphicsContext {
    pub config: SurfaceConfiguration,
    pub surface: Arc<Surface<'static>>,
    pub device: Arc<Device>,
    pub instance: Arc<Instance>,
    pub queue: Arc<Queue>,
    pub adapter: Arc<Adapter>,
}

impl GraphicsContext {
    /// Creates a new `GraphicsContext` with the given window.
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = Arc::new(Instance::new(&InstanceDescriptor::default()));

        let surface = Arc::new(
            instance
                .create_surface(window.clone())
                .expect("Failed to create surface from window"),
        );

        let adapter = Arc::new(
            instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .expect("Failed to find an appropriate adapter to use."),
        );

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Main WGPU device"),
                required_features: wgpu::Features::empty() | wgpu::Features::POLYGON_MODE_LINE,
                ..Default::default()
            })
            .await
            .expect("Failed to create a WGPU device with necessary features.");
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let present_mode = if surface_caps.present_modes.contains(&PresentMode::Immediate) {
            PresentMode::Immediate
        } else {
            PresentMode::AutoNoVsync
        };

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            width: window.inner_size().width,
            height: window.inner_size().height,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            format: surface_format,
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
        };
        surface.configure(&device, &config);

        debug!(
            target: "wgpu_init",
            "\nAdapter: '{}'\nBackend: {:?}\nSurface Format: {:?}\nPresent Mode: {:?}",
            adapter.get_info().name,
            adapter.get_info().backend,
            config.format,
            config.present_mode,
        );

        Self {
            instance,
            surface,
            config,
            adapter,
            device,
            queue,
        }
    }

    /// Let the graphics context know that the window associated with the graphics
    /// context been resized. Relays information to the necessary config elements.
    pub fn inform_resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        } else {
            warn!("Attempted to resize graphics context to zero dimensions.");
        }
    }
}
