pub mod external;
pub mod internal;

pub use external::{RawDeviceMessage, RawWindowMessage};
pub use internal::{
    KeyboardInputMessage, MouseButtonInputMessage, MouseMoveMessage, MouseScrollMessage,
};
