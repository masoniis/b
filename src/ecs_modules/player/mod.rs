use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;

mod main_system;
pub use main_system::*;

pub struct PlayerModule;

impl PlayerModule {
    pub fn build(
        _startup_schedule: &mut Schedule,
        main_schedule: &mut Schedule,
        _world: &mut World,
    ) {
        main_schedule.add_systems((main_system::camera_control_system,));
    }
}
