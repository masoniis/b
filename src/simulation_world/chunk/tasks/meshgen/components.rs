use crate::simulation_world::chunk::{OpaqueMeshComponent, TransparentMeshComponent};
use bevy_ecs::prelude::Component;
use crossbeam::channel::Receiver;

/// Marks a chunk meshing task in the simulation world that returns a MeshComponent.
#[derive(Component)]
pub struct ChunkMeshingTaskComponent {
    pub receiver: Receiver<(
        Option<OpaqueMeshComponent>,
        Option<TransparentMeshComponent>,
    )>,
}

/// A signal marking that chunks should be checked for meshing. This check is a necessary
/// optimization as chunks require all neighbors to be generated before they mesh.
#[derive(Component)]
pub struct CheckForMeshing;

/// A signal marking that chunks wants to be meshed. In this phase, the chunk is
/// waiting to be assigned to the thread pool, and can't be assigned until all
/// of its relevant neighbors have block data generated.
#[derive(Component)]
pub struct WantsMeshing;
