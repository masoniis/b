use glutin::context::PossiblyCurrentContext;
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, WindowSurface};

pub struct Renderer {
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

impl Renderer {
    pub fn new(gl_surface: Surface<WindowSurface>, gl_context: PossiblyCurrentContext) -> Self {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            // These are for blending the text/scene
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Renderer {
            gl_surface,
            gl_context,
        }
    }

    pub fn clear_frame(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn swap_buffers(&self) {
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }
}
