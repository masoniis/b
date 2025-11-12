use crate::render_world::global_extract::run_extract_schedule::SimulationWorld;
use crate::simulation_world::player::CameraComponent;
use crate::{prelude::*, simulation_world::player::ActiveCamera};
use bevy_ecs::prelude::{Res, ResMut, Resource};

/// A resource in the render world holding the extracted camera matrices.
#[derive(Resource, Debug, Default)]
pub struct RenderCameraResource {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub world_position: Vec3,
}

/// A standalone system to extract the active camera's data from sim world.
#[instrument(skip_all)]
pub fn extract_active_camera_system(
    // Input
    simulation_world: Res<SimulationWorld>,

    // Output
    mut render_camera: ResMut<RenderCameraResource>,
) {
    let sim_world = &simulation_world.val;

    // get the ActiveCamera resource from the simulation world
    let active_camera_res = match sim_world.get_resource::<ActiveCamera>() {
        Some(res) => res,
        None => {
            warn!("extract_active_camera_system: SimulationWorld has no ActiveCamera resource.");
            return;
        }
    };
    let active_entity = active_camera_res.0;

    // get the CameraComponent for that entity
    let source_component = match sim_world.get::<CameraComponent>(active_entity) {
        Some(comp) => comp,
        None => {
            warn!(
                "extract_active_camera_system: ActiveCamera entity {:?} has no CameraComponent.",
                active_entity
            );
            return; // entity exists but component is missing
        }
    };

    // update the render world camera resource
    render_camera.view_matrix = source_component.view_matrix;
    render_camera.projection_matrix = source_component.projection_matrix;
    render_camera.world_position = source_component.position;
}
