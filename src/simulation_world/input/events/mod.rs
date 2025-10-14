pub mod external;
pub mod internal;

pub use external::{RawDeviceEvent, RawWindowEvent};
pub use internal::{KeyboardInputEvent, MouseButtonInputEvent, MouseMoveEvent, MouseScrollEvent};
