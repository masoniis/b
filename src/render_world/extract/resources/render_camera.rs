use crate::ecs_resources::CameraResource;
use crate::render_world::extract::utils::ExtractResource; // Assuming this path exists
use bevy_ecs::prelude::Resource;
use glam::Mat4;

/// A resource in the render world holding the extracted camera matrices.
/// Pre-calculating all matrices here is efficient, as render systems
/// can access exactly what they need without re-computing.
#[derive(Resource, Debug, Default)]
pub struct RenderCameraResource {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}

impl ExtractResource for RenderCameraResource {
    type Source = CameraResource;
    type Output = Self;

    fn extract_resource(source: &Self::Source) -> Self::Output {
        return RenderCameraResource {
            view_matrix: source.view_matrix,
            projection_matrix: source.projection_matrix,
        };
    }
}
