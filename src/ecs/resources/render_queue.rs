use bevy_ecs::prelude::Resource;

use crate::graphics::{main_renderer::QueuedDraw, text_renderer::QueuedText};

#[derive(Resource, Default)]
pub struct RenderQueueResource {
    /// Elements queued to render in the world
    scene_object_queue: Vec<QueuedDraw>,

    /// Elements queued to the screen as text UI elements
    screen_text_queue: Vec<QueuedText>,
}

impl RenderQueueResource {
    /// Clear all queued elements
    pub fn clear_object_queue(&mut self) {
        self.scene_object_queue.clear();
    }

    pub fn clear_text_queue(&mut self) {
        self.screen_text_queue.clear();
    }

    // INFO: -------------------------
    //         Adding to queue
    // -------------------------------

    pub fn add_screen_text(&mut self, text: QueuedText) {
        self.screen_text_queue.push(text);
    }

    pub fn add_scene_object(&mut self, object: QueuedDraw) {
        self.scene_object_queue.push(object);
    }

    // INFO: ---------------------------
    //         Retrieving queues
    // ---------------------------------

    pub fn get_scene_objects(&self) -> &Vec<QueuedDraw> {
        &self.scene_object_queue
    }

    pub fn get_screen_texts(&self) -> &Vec<QueuedText> {
        &self.screen_text_queue
    }
}
