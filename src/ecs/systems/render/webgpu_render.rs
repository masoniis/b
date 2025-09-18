use crate::graphics::webgpu_renderer::WebGpuRenderer;
use bevy_ecs::prelude::NonSendMut;
use tracing::error;

pub fn webgpu_render_system(mut renderer: NonSendMut<WebGpuRenderer>) {
    match renderer.render() {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost) => {
            // Reconfigure the surface if lost
            // This requires access to the window size, which is not directly available here.
            // For now, we'll just log an error.
            error!("Surface lost, reconfiguring not yet implemented.");
        }
        Err(wgpu::SurfaceError::OutOfMemory) => {
            // Handle out of memory by exiting
            panic!("Out of memory!");
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
