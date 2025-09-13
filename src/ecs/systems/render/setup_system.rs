use crate::graphics::renderer::Renderer;
use bevy_ecs::prelude::NonSendMut;

pub fn setup_render_system(renderer: NonSendMut<Renderer>, // main-thread only (NonSend)
) {
    renderer.clear_frame();
}
