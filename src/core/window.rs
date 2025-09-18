use crate::core::app::App;
use std::error::Error;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub async fn run_app() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let window_attributes = Window::default_attributes()
        .with_title("🅱️")
        .with_inner_size(PhysicalSize::new(1800, 1500));

    let window = event_loop.create_window(window_attributes)?;

    let mut app = App::new(&window);
    event_loop.run_app(&mut app)?;
    Ok(())
}
