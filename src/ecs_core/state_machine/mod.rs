// The state machine module provides a flexible way to manage any transitionary state.
//
// This is important for both ECS worlds and thus it is in ecs_core. It provides utilities
// for run conditions based on the current state, for example, and is very generic overall.

pub mod plugin;
pub mod resources;
pub mod systems;
pub mod utils;

pub use plugin::*;
pub use resources::*;
pub use systems::*;
pub use utils::*;
