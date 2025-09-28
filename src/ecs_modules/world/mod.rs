use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;

pub mod startup_system;
pub use startup_system as world_startup_system;
pub use world_startup_system::*;

pub mod main_system;
pub use main_system as world_main_system;
pub use world_main_system::*;

pub struct WorldModule;

impl WorldModule {
    pub fn build(
        _startup_schedule: &mut Schedule,
        _main_schedule: &mut Schedule,
        _world: &mut World,
    ) {
        _startup_schedule.add_systems((startup_system::chunk_generation_system,));
        _main_schedule.add_systems((main_system::time_system,));
    }
}
