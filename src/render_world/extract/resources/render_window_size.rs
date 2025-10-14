use crate::{
    render_world::extract::extract_resource::ExtractResource,
    simulation_world::input::resources::WindowSizeResource,
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    prelude::Resource,
    system::{Commands, ResMut},
};

#[derive(Resource, Debug, Default, PartialEq)]
pub struct RenderWindowSizeResource {
    pub width: f32,
    pub height: f32,
}

impl ExtractResource for RenderWindowSizeResource {
    type Source = WindowSizeResource;
    type Output = RenderWindowSizeResource;

    fn extract_and_update(
        commands: &mut Commands,
        source: &Self::Source,
        target: Option<ResMut<Self::Output>>,
    ) {
        let new_size = RenderWindowSizeResource {
            width: source.width as f32,
            height: source.height as f32,
        };

        if let Some(mut target_res) = target {
            target_res.set_if_neq(new_size);
        } else {
            commands.insert_resource(new_size);
        }
    }
}
