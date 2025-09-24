use crate::ecs::components::ScreenTextComponent;
use crate::graphics::webgpu_renderer::WebGpuRenderer;
use bevy_ecs::prelude::{Query, ResMut};

pub fn screen_text_render_system(
    mut _renderer: ResMut<WebGpuRenderer>,
    _query: Query<&ScreenTextComponent>,
) {
    return;
}
