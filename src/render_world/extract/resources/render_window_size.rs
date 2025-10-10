use crate::{
    game_world::input::resources::WindowSizeResource,
    render_world::extract::extract_resource::ExtractResource,
};
use bevy_ecs::{
    prelude::Resource,
    system::{Commands, ResMut},
};

#[derive(Resource, Debug, Default)]
pub struct RenderWindowSizeResource {
    pub width: f32,
    pub height: f32,
}

// impl ExtractResource for RenderWindowSizeResource {
//     type Source = WindowSizeResource;
//     type Output = Self;
//
//     fn extract_resource(source: &Self::Source) -> Self::Output {
//         RenderWindowSizeResource {
//             width: source.width as f32,
//             height: source.height as f32,
//         }
//     }
// }

impl ExtractResource for RenderWindowSizeResource {
    type Source = WindowSizeResource;
    type Output = RenderWindowSizeResource;

    fn extract_and_update(
        commands: &mut Commands,
        source: &Self::Source,
        target: Option<ResMut<Self::Output>>,
    ) {
        let new_width = source.width as f32;
        let new_height = source.height as f32;

        if let Some(mut target_res) = target {
            // The resource exists. Unconditionally update it.
            target_res.width = new_width;
            target_res.height = new_height;
        } else {
            // The resource doesn't exist. Insert it.
            commands.insert_resource(RenderWindowSizeResource {
                width: new_width,
                height: new_height,
            });
        }
    }
}
