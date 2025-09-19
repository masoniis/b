use crate::core::app::App;
use std::error::Error;
use winit::event_loop::EventLoop;

pub async fn run_app() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let mut app = App::new();

    event_loop.run_app(&mut app)?;
    Ok(())
}
