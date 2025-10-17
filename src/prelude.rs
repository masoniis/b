/// Prelude is to be reserved for imports that get used across many
/// files. For this project, that mostly includes things that are used
/// at a system ecs module level (eg plugin) as there will be many modules.
pub use crate::simulation_world::SimulationSet;
pub use crate::{
    ecs_core::{CommonEcsInterface, EcsBuilder, Plugin, PluginGroup, ScheduleBuilder},
    utils::*,
};

pub use tracing::{debug, error, info, info_span, instrument, warn};

pub use glam::{Mat3, Mat4, Vec2, Vec3, Vec4};

pub use winit::{
    dpi::{LogicalSize, PhysicalSize},
    keyboard::KeyCode,
};
