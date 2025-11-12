pub mod actions;
pub mod camera;

pub use actions::*;
pub use camera::*;

// INFO: -----------------------
//         player plugin
// -----------------------------

use crate::ecs_core::{EcsBuilder, Plugin};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_plugin(CameraPlugin).add_plugin(ActionPlugin);
    }
}
