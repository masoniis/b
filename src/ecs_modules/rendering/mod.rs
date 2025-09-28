use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;

mod components;
pub use components::*;

mod main_system;
pub use main_system::*;

pub struct RenderingModule;

impl RenderingModule {
    pub fn build(
        _startup_schedule: &mut Schedule,
        main_schedule: &mut Schedule,
        _world: &mut World,
    ) {
        main_schedule.add_systems((
            main_system::changed_mesh_system,
            main_system::removed_mesh_system,
        ));
    }
}
