// While multiple external preludes can obscure the origin of code, a single internal
// prelude does the opposite. This file will serve as the one clear, predictable source
// for the project's most common imports. It will be frequently used across many files.

// I plan to not use any  external preludes, and rely solely on this prelude for common imports.
pub use crate::{game_world::CoreSet, utils::*};
pub use tracing::{debug, error, info, warn};
pub use winit::{
    dpi::{LogicalSize, PhysicalSize},
    keyboard::KeyCode,
};
