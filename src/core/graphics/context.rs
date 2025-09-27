use crate::{
    core::graphics::rendercore::Renderer,
    ecs_resources::{
        asset_storage::MeshAsset, AssetStorageResource, CameraUniformResource, RenderQueueResource,
    },
    prelude::*,
};
use std::sync::Arc;
use wgpu::{
    Adapter, Device, DeviceDescriptor, Instance, InstanceDescriptor, PowerPreference, PresentMode,
    Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError,
    TextureViewDescriptor,
};
use winit::window::Window;

/// A container for the core WGPU state and the renderer.
pub struct GraphicsContext {
    // renderer
    pub renderer: Renderer,

    // properties
    pub config: SurfaceConfiguration,
    pub surface: Surface<'static>,
    pub device: Arc<Device>,
    pub instance: Instance,
    pub queue: Arc<Queue>,
    pub adapter: Adapter,
}

impl GraphicsContext {
    /// Creates a new `GraphicsContext` with the given window.
    pub async fn new(window: Arc<Window>) -> Self {
        let wgpu_instance = Instance::new(&InstanceDescriptor::default());

        let surface = wgpu_instance
            .create_surface(window.clone())
            .expect("Failed to create surface from window");

        let adapter = wgpu_instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .unwrap();

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

        let renderer = Renderer::new(device.clone(), queue.clone(), &config);

        Self {
            // renderer
            renderer,

            // properties
            instance: wgpu_instance,
            surface,
            config,
            adapter,
            device,
            queue,
        }
    }

    /// Let the graphics context know that the window associated with the graphics
    /// context been resized. Relays information to the necessary config elements.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.renderer.resize(new_size);
        } else {
            warn!("Attempted to resize graphics context to zero dimensions.");
        }
    }

    /// A render method that handles the bridge between the ECS world and the
    /// graphics context. Takes in ECS global resources related to rendering.
    pub fn render(
        &mut self,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    ) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        if let Err(e) = self
            .renderer
            .render(&view, render_queue, mesh_assets, camera_uniform)
        {
            eprintln!("Renderer error: {:?}", e);
        }

        output.present();
        Ok(())
    }
}
