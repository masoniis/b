use bevy_ecs::prelude::*;
use glam::IVec3;
use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct ChunkLoadManager {
    /// Map of coordinates for chunks that are fully loaded and have an entity spawned.
    pub loaded_chunks: HashMap<IVec3, Entity>,

    /// Map of coordinates for chunks currently being generated.
    /// Key: IVec3 chunk coordinate
    /// Value: Entity ID holding the generation Task component.
    pub generating_chunks: HashMap<IVec3, Entity>,

    /// Map of coordinates for chunks currently being meshed.
    /// Key: IVec3 chunk coordinate
    /// Value: Entity ID holding the meshing Task component.
    pub meshing_chunks: HashMap<IVec3, Entity>,
}

impl ChunkLoadManager {
    /// Checks if a chunk is loaded, being generated, or being meshed.
    pub fn is_chunk_present_or_loading(&self, coord: IVec3) -> bool {
        self.loaded_chunks.contains_key(&coord)
            || self.generating_chunks.contains_key(&coord)
            || self.meshing_chunks.contains_key(&coord)
    }

    // Mark a chunk that has an async generation task running.
    pub fn mark_as_generating(&mut self, coord: IVec3, generation_task_entity: Entity) {
        self.generating_chunks.insert(coord, generation_task_entity);
    }

    // Mark a chunk that has an async meshing task running.
    pub fn mark_as_meshing(&mut self, coord: IVec3, meshing_task_entity: Entity) {
        self.generating_chunks.remove(&coord); // no longer generating
        self.meshing_chunks.insert(coord, meshing_task_entity); // now meshing
    }

    // Mark a chunk as fully loaded with a mesh and transform.
    pub fn mark_as_loaded(&mut self, coord: IVec3, final_chunk_entity: Entity) {
        self.meshing_chunks.remove(&coord); // no longer meshing
        self.loaded_chunks.insert(coord, final_chunk_entity); // now loaded
    }

    /// Mark a loaded chunk entity as despawned.
    pub fn mark_as_unloaded(&mut self, coord: IVec3) {
        self.loaded_chunks.remove(&coord);
    }

    /// Mark a chunk loading task for cancellation (either generation or meshing).
    pub fn mark_as_cancelled(&mut self, coord: IVec3) {
        self.generating_chunks.remove(&coord);
        self.meshing_chunks.remove(&coord);
    }
}
