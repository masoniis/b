/// Prelude is to be reserved for imports that get used across many
/// files. For this project, that mostly includes things that are used
/// at a system ecs module level (eg plugin) as there will be many modules.
pub use crate::{
    ecs_core::{CommonEcsInterface, EcsBuilder, Plugin, PluginGroup, ScheduleBuilder},
    simulation_world::SimulationSet,
    utils::*,
};

pub use derive_more::{Deref, DerefMut};

pub use glam::{Mat3, Mat4, Vec2, Vec3, Vec4};

pub use tracing::{debug, error, info, info_span, instrument, warn};

pub use winit::{
    dpi::{LogicalSize, PhysicalSize},
    keyboard::KeyCode,
};
