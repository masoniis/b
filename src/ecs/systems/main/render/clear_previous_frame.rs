use crate::graphics::WebGpuRenderer;
use bevy_ecs::prelude::ResMut;

pub fn clear_previous_frame_system(mut renderer: ResMut<WebGpuRenderer>) {
    renderer.clear_queue();
}
