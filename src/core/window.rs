use tracing::info;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct MyApp {
    window: Option<Window>,
}

impl ApplicationHandler for MyApp {
    /// Method called when the app starts of or resumes.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            info!("App resumed, creating window!");
            let window_attributes = Window::default_attributes().with_title("🅱️");
            let window = event_loop.create_window(window_attributes).unwrap();
            self.window = Some(window);
        }
    }

    /// Method called on window events.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Close button was pressed, exiting.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    // request another redraw to keep the animation loop going
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_app = MyApp::default();

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut my_app)?;

    Ok(())
}
