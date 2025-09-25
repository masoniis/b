use crate::ecs::resources::RenderQueueResource;
use bevy_ecs::prelude::ResMut;

pub fn clear_previous_frame_system(mut render_queue: ResMut<RenderQueueResource>) {
    render_queue.clear_object_queue();
}
