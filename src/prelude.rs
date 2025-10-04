/// Prelude is to be reserved for imports that get used across many
/// files. For this project, that mostly includes things that are used
/// at a system ecs module level (eg plugin) as there will be many modules.
pub use crate::{
    core::world::{CommonEcsInterface, EcsBuilder, Plugin, PluginGroup, ScheduleBuilder},
    game_world::system_sets::CoreSet,
    utils::*,
};

pub use tracing::{debug, error, info, warn};

pub use winit::{
    dpi::{LogicalSize, PhysicalSize},
    keyboard::KeyCode,
};
