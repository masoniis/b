use crate::core::app::App;
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::GlDisplay;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::GlWindow;
use std::error::Error;
use std::ffi::CString;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;

pub fn run_app() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}

pub fn create_gl_window(
    event_loop: &ActiveEventLoop,
) -> (
    Window,
    Surface<WindowSurface>,
    glutin::context::PossiblyCurrentContext,
) {
    let window_attributes = Window::default_attributes().with_title("🅱️");
    let template = glutin::config::ConfigTemplateBuilder::new();
    let (window, gl_config) = glutin_winit::DisplayBuilder::new()
        .with_window_attributes(Some(window_attributes.clone()))
        .build(event_loop, template, |mut configs| configs.next().unwrap())
        .unwrap();

    let window = window.unwrap();

    let raw_window_handle = window.window_handle().unwrap().as_raw();

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

    let not_current_gl_context =
        unsafe { gl_display.create_context(&gl_config, &context_attributes) }.unwrap();

    let attrs = window.build_surface_attributes(Default::default()).unwrap();
    let gl_surface = unsafe { gl_display.create_window_surface(&gl_config, &attrs) }.unwrap();

    let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

    gl::load_with(|s| {
        let s = CString::new(s).unwrap();
        gl_display.get_proc_address(&s)
    });

    (window, gl_surface, gl_context)
}
