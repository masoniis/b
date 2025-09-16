use crate::graphics::renderer::Renderer;
use bevy_ecs::prelude::NonSendMut;

pub fn begin_frame_system(renderer: NonSendMut<Renderer>, // main-thread only (NonSend)
) {
    renderer.clear_frame();
}
