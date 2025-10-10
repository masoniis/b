use crate::game_world::global_resources::camera::CameraResource;
use crate::prelude::*;
use crate::render_world::extract::extract_resource::ExtractResource;
use bevy_ecs::prelude::Resource;
use bevy_ecs::system::{Commands, ResMut};

/// A resource in the render world holding the extracted camera matrices.
/// Pre-calculating all matrices here is efficient, as render systems
/// can access exactly what they need without re-computing.
#[derive(Resource, Debug, Default)]
pub struct RenderCameraResource {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub world_position: Vec3,
}

impl ExtractResource for RenderCameraResource {
    type Source = CameraResource;
    type Output = Self;

    fn extract_and_update(
        commands: &mut Commands,
        source: &Self::Source,
        target: Option<ResMut<Self::Output>>,
    ) {
        let view_matrix = source.view_matrix;
        let projection_matrix = source.projection_matrix;
        let world_position = source.view_matrix.inverse().w_axis.truncate();

        // Handle the insert vs. update logic
        if let Some(mut target_res) = target {
            target_res.view_matrix = view_matrix;
            target_res.projection_matrix = projection_matrix;
            target_res.world_position = world_position;
        } else {
            commands.insert_resource(RenderCameraResource {
                view_matrix,
                projection_matrix,
                world_position,
            });
        }
    }
}
