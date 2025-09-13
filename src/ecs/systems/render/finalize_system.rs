use crate::graphics::renderer::Renderer;
use bevy_ecs::prelude::NonSendMut;

pub fn finalize_render_system(renderer: NonSendMut<Renderer>, // main-thread only (NonSend)
) {
    renderer.swap_buffers();
}
