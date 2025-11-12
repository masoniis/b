use crate::simulation_world::{
    camera::{ActiveCamera, CameraComponent},
    user_interface::{components::UiText, screens::debug::debug_screen::CameraXYZCoordTextMarker},
};
use bevy_ecs::prelude::*;

pub fn update_camera_xyz_coord_screen_text(
    active_camera: Res<ActiveCamera>,
    camera_query: Query<&CameraComponent>,
    mut query: Query<&mut UiText, With<CameraXYZCoordTextMarker>>,
) {
    if let Ok(cam) = camera_query.get(active_camera.0) {
        if let Ok(mut ui_text) = query.single_mut() {
            ui_text.content = format!(
                "{:.1}, {:.1}, {:.1}",
                cam.position.x, cam.position.y, cam.position.z
            );
        }
    }
}
