use crate::core::graphics::WebGpuRenderer;
use crate::ecs_resources::{
    asset_storage::MeshAsset, AssetStorageResource, CameraUniformResource, RenderQueueResource,
};
use std::sync::Arc;
use tracing::debug;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration, SurfaceError};
use winit::window::Window;

/// A container for the core WGPU state and the renderer.
pub struct GraphicsContext {
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub adapter: Adapter,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub renderer: WebGpuRenderer,
}

impl GraphicsContext {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface from window");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
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

        let present_mode = if surface_caps
            .present_modes
            .contains(&wgpu::PresentMode::Immediate)
        {
            wgpu::PresentMode::Immediate
        } else {
            wgpu::PresentMode::AutoNoVsync
        };

        let config = wgpu::SurfaceConfiguration {
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

        let renderer = WebGpuRenderer::new(device.clone(), queue.clone(), &config);

        Self {
            instance,
            surface,
            config,
            adapter,
            device,
            queue,
            renderer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.renderer.resize(new_size);
        }
    }

    pub fn render(
        &mut self,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    ) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

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
