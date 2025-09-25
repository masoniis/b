use bevy_ecs::prelude::Resource;

#[derive(Resource, Default)]
pub struct CameraUniformResource {
    pub view_proj_matrix: glam::Mat4,
}
