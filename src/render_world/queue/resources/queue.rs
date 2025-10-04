use bevy_ecs::prelude::Entity;
use bevy_ecs::prelude::Resource;
use std::collections::hash_map::{Entry, HashMap};
use tracing::debug; // For debugging the remove logic

use crate::{
    core::graphics::types::gpu_queues::{QueuedDraw, QueuedText},
    game_world::global_resources::asset_storage::{Handle, MeshAsset},
};

#[derive(Resource, Default)]
pub struct RenderQueueResource {
    /// The actual draw data, stored contiguously for fast iteration by the renderer.
    scene_object_queue: Vec<QueuedDraw>,
    /// An index mapping an Entity to its location in the `scene_object_queue` Vec.
    /// Used to enable O(1) removals and lookups at the cost of extra memory.
    entity_to_queue_index: HashMap<Entity, usize>,

    // --- Screen Text Queue (unchanged) ---
    /// ECS Entities queued to the screen as text UI elements
    screen_texts: HashMap<Entity, QueuedText>,
}

impl RenderQueueResource {
    // INFO: ----------------------------
    //        Adding render data
    // ----------------------------------

    /// Add a screen text to the render queue
    pub fn add_screen_text(&mut self, entity: Entity, text: QueuedText) {
        self.screen_texts.insert(entity, text);
    }

    /// Add a scene object to the render queue, updating both the Vec and the index map.
    pub fn add_scene_object(&mut self, entity: Entity, object: QueuedDraw) {
        // Sanity check in debug builds to ensure data integrity.
        debug_assert_eq!(
            entity, object.entity,
            "Mismatched entity in add_scene_object"
        );

        let index = self.scene_object_queue.len();
        self.scene_object_queue.push(object);
        self.entity_to_queue_index.insert(entity, index);
    }

    // INFO: ----------------------------------------
    //        Retrieving individual elements
    // ----------------------------------------------

    // --- Scene Objects (New/Modified) ---

    /// Gets a mutable reference to a specific QueuedDraw by its Entity.
    pub fn get_scene_object_mut(&mut self, entity: &Entity) -> Option<&mut QueuedDraw> {
        // Look up the index in the map (O(1) average).
        let index = *self.entity_to_queue_index.get(entity)?;
        // Use the index to get the mutable reference from the vector (O(1)).
        self.scene_object_queue.get_mut(index)
    }

    // --- Screen Texts (Unchanged) ---

    pub fn get_screen_text(&self, entity: &Entity) -> Option<&QueuedText> {
        self.screen_texts.get(entity)
    }

    pub fn get_or_insert_screen_text_mut(&mut self, entity: Entity) -> &mut QueuedText {
        self.screen_texts.entry(entity).or_default()
    }

    pub fn get_screen_text_mut(&mut self, entity: &Entity) -> Option<&mut QueuedText> {
        self.screen_texts.get_mut(entity)
    }

    pub fn get_screen_text_entry(&mut self, entity: Entity) -> Entry<'_, Entity, QueuedText> {
        self.screen_texts.entry(entity)
    }

    // INFO: ---------------------------
    //        Retrieving queues
    // ---------------------------------

    /// This remains unchanged and is just as fast as before.
    pub fn get_scene_objects(&self) -> &Vec<QueuedDraw> {
        &self.scene_object_queue
    }

    /// This remains unchanged.
    pub fn iter_by_mesh(&self) -> HashMap<Handle<MeshAsset>, Vec<&QueuedDraw>> {
        let mut map: HashMap<Handle<MeshAsset>, Vec<&QueuedDraw>> = HashMap::new();
        for draw in &self.scene_object_queue {
            map.entry(draw.mesh_handle).or_default().push(draw);
        }
        map
    }

    pub fn get_screen_texts(&self) -> impl Iterator<Item = &QueuedText> {
        self.screen_texts.values()
    }

    // INFO: ---------------------------
    //        Removing elements
    // ---------------------------------

    /// Removes a scene object using the fast swap_remove pattern.
    pub fn remove_scene_object(&mut self, entity: &Entity) -> Option<QueuedDraw> {
        // 1. Get the index of the entity to remove from the map, and remove the map entry.
        let index_to_remove = self.entity_to_queue_index.remove(entity)?;

        // 2. Use `swap_remove` on the vector. This is an O(1) operation that moves the
        //    *last* element into the hole left by the removed element.
        let removed_object = self.scene_object_queue.swap_remove(index_to_remove);

        // 3. IMPORTANT: If an element was moved (i.e., the removed element was not the last one),
        //    we must update the map to point to the moved element's new index.
        if let Some(moved_object) = self.scene_object_queue.get(index_to_remove) {
            let moved_entity = moved_object.entity;
            debug!(
                "Entity {:?} was moved from index {} to {}",
                moved_entity,
                self.scene_object_queue.len(),
                index_to_remove
            );
            // Update the map entry for the moved element to point to its new home.
            self.entity_to_queue_index
                .insert(moved_entity, index_to_remove);
        }

        Some(removed_object)
    }

    /// Must clear both the vector and the index map.
    pub fn clear_object_queue(&mut self) {
        self.scene_object_queue.clear();
        self.entity_to_queue_index.clear();
    }

    pub fn remove_screen_text(&mut self, entity: &Entity) -> Option<QueuedText> {
        self.screen_texts.remove(entity)
    }
}
