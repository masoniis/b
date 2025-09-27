use bevy_ecs::prelude::Entity;
use bevy_ecs::prelude::Resource;
use std::collections::hash_map::{Entry, HashMap};

use crate::{
    core::graphics::{rendercore::QueuedDraw, renderpass::text_renderpass::QueuedText},
    ecs_resources::asset_storage::{Handle, MeshAsset},
};

#[derive(Resource, Default)]
pub struct RenderQueueResource {
    /// Elements queued to render in the world
    scene_object_queue: Vec<QueuedDraw>,

    /// ECS Entities queued to the screen as text UI elements
    screen_texts: HashMap<Entity, QueuedText>,
}

impl RenderQueueResource {
    /// Clear all queued elements
    pub fn clear_object_queue(&mut self) {
        self.scene_object_queue.clear();
    }

    // INFO: ----------------------------
    //         Adding render data
    // ----------------------------------

    pub fn add_screen_text(&mut self, entity: Entity, text: QueuedText) {
        self.screen_texts.insert(entity, text);
    }

    pub fn add_scene_object(&mut self, object: QueuedDraw) {
        self.scene_object_queue.push(object);
    }

    // INFO: ----------------------------------------
    //         Retrieving individual elements
    // ----------------------------------------------

    pub fn get_screen_text(&self, entity: &Entity) -> Option<&QueuedText> {
        self.screen_texts.get(entity)
    }

    pub fn get_or_insert_screen_text_mut(&mut self, entity: Entity) -> &mut QueuedText {
        return self.screen_texts.entry(entity).or_default();
    }

    pub fn get_screen_text_mut(&mut self, entity: &Entity) -> Option<&mut QueuedText> {
        self.screen_texts.get_mut(entity)
    }

    pub fn get_screen_text_entry(&mut self, entity: Entity) -> Entry<'_, Entity, QueuedText> {
        self.screen_texts.entry(entity)
    }

    // INFO: ---------------------------
    //         Retrieving queues
    // ---------------------------------

    pub fn get_scene_objects(&self) -> &Vec<QueuedDraw> {
        &self.scene_object_queue
    }

    pub fn get_screen_texts(&self) -> impl Iterator<Item = &QueuedText> {
        self.screen_texts.values()
    }

    pub fn iter_by_mesh(&self) -> HashMap<Handle<MeshAsset>, Vec<&QueuedDraw>> {
        let mut map: HashMap<Handle<MeshAsset>, Vec<&QueuedDraw>> = HashMap::new();
        for draw in &self.scene_object_queue {
            map.entry(draw.mesh_handle).or_default().push(draw);
        }
        map
    }

    // INFO: ---------------------------
    //         Removing elements
    // ---------------------------------

    pub fn remove_screen_text(&mut self, entity: &Entity) -> Option<QueuedText> {
        self.screen_texts.remove(entity)
    }
}
