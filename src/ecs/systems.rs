use crate::ecs::resources::Input;
use winit::event::{Event, WindowEvent};
use winit::keyboard::PhysicalKey;

pub fn input_system(input: &mut Input, event: &Event<()>) {
    if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::KeyboardInput {
            event: key_event, ..
        } = event
        {
            if let PhysicalKey::Code(key_code) = key_event.physical_key {
                match key_event.state {
                    winit::event::ElementState::Pressed => {
                        input.pressed_keys.insert(key_code);
                    }
                    winit::event::ElementState::Released => {
                        input.pressed_keys.remove(&key_code);
                    }
                }
            }
        }
    }
}

