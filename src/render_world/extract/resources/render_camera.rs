use crate::game_world::global_resources::camera::CameraResource;
use crate::prelude::*;
use crate::render_world::extract::utils::ExtractResource;
use bevy_ecs::prelude::Resource;

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

    fn extract_resource(source: &Self::Source) -> Self::Output {
        return RenderCameraResource {
            view_matrix: source.view_matrix,
            projection_matrix: source.projection_matrix,
            world_position: source.view_matrix.inverse().w_axis.truncate(),
        };
    }
}
