use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct MyApp {
    window: Option<Window>,
}

impl ApplicationHandler for MyApp {
    /// Method called when the app starts of or resumes.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            println!("App resumed, creating window!");
            let window_attributes = Window::default_attributes().with_title("My App");
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
                println!("Close button was pressed, exiting.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // println!("Redrawing the window!");

                // We must request another redraw to keep the animation loop going
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (), // Handle other events if you need to
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut my_app = MyApp::default();
    event_loop.run_app(&mut my_app)?;

    Ok(())
}
