use bevy_ecs::prelude::*;
use glam::IVec3;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    NeedsGenerating(Entity), // Entity that can be acquired for generation
    Generating(Entity),      // Entity holds the generation Task component
    NeedsMeshing(Entity),    // Data is generated, waiting for meshing slot
    Meshing(Entity),         // Entity holds the meshing Task component
    Loaded(Option<Entity>),  // Entity is the final, rendered chunk
}

impl ChunkState {
    /// Returns the Entity associated with this chunk state.
    pub fn entity(&self) -> Option<Entity> {
        match *self {
            ChunkState::NeedsGenerating(e) => Some(e),
            ChunkState::Generating(e) => Some(e),
            ChunkState::NeedsMeshing(e) => Some(e),
            ChunkState::Meshing(e) => Some(e),
            ChunkState::Loaded(e) => e,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct ChunkLoadManager {
    /// Map tracking the state of all non-unloaded chunks.
    pub chunk_states: HashMap<IVec3, ChunkState>,
}

impl ChunkLoadManager {
    /// Gets the current state of a chunk, if tracked.
    pub fn get_state(&self, coord: IVec3) -> Option<ChunkState> {
        self.chunk_states.get(&coord).copied()
    }

    /// Gets the Entity for a chunk, if that chunk is tracked.
    pub fn get_entity(&self, coord: IVec3) -> Option<Entity> {
        self.chunk_states.get(&coord).map(|state| state.entity())?
    }

    /// Checks if a chunk exists in any loading or loaded state.
    pub fn is_chunk_present_or_loading(&self, coord: IVec3) -> bool {
        self.chunk_states.contains_key(&coord)
    }

    /// Marks that a chunk is requested to be loaded.
    pub fn mark_as_needs_generating(&mut self, coord: IVec3, needs_generation_task_entity: Entity) {
        self.chunk_states.insert(
            coord,
            ChunkState::NeedsGenerating(needs_generation_task_entity),
        );
    }

    /// Marks that a chunk is currently undergoing generation.
    pub fn mark_as_generating(&mut self, coord: IVec3, generation_task_entity: Entity) {
        self.chunk_states
            .insert(coord, ChunkState::Generating(generation_task_entity));
    }

    /// Called once a chunk's data is generated but needs to be meshed.
    pub fn mark_as_needs_meshing(&mut self, coord: IVec3, needs_meshing_entity: Entity) {
        self.chunk_states
            .insert(coord, ChunkState::NeedsMeshing(needs_meshing_entity));
    }

    /// Called once a chunk starts meshing.
    pub fn mark_as_meshing(&mut self, coord: IVec3, meshing_task_entity: Entity) {
        self.chunk_states
            .insert(coord, ChunkState::Meshing(meshing_task_entity));
    }

    /// Called once a chunk has finished meshing and is fully loaded.
    pub fn mark_as_loaded(&mut self, coord: IVec3, final_chunk_entity: Entity) {
        self.chunk_states
            .insert(coord, ChunkState::Loaded(Some(final_chunk_entity)));
    }

    /// Called once a chunk has finished meshing and is fully loaded.
    ///
    /// Passing no entity means the chunk is empty and does not need to be rendered.
    pub fn mark_as_loaded_but_empty(&mut self, coord: IVec3) {
        self.chunk_states.insert(coord, ChunkState::Loaded(None));
    }

    /// Called when a chunk is unloaded, removing it from tracking.
    pub fn mark_as_unloaded(&mut self, coord: IVec3) {
        self.chunk_states.remove(&coord);
    }

    /// A help to iterate over all chunks needing meshing.
    ///
    /// Necessary to prevent throttling by only meshing a few
    /// chunks per frame/tick.
    pub fn iter_needs_meshing(&self) -> impl Iterator<Item = &IVec3> {
        self.chunk_states.iter().filter_map(|(coord, state)| {
            if matches!(state, ChunkState::NeedsMeshing(_)) {
                Some(coord)
            } else {
                None
            }
        })
    }
}
