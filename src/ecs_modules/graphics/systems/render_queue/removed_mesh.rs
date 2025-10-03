use crate::{
    ecs_modules::graphics::{MeshComponent, RenderQueueResource},
    prelude::*,
};
use bevy_ecs::prelude::{RemovedComponents, ResMut};

/// removed despawned meshes from the render queue.
pub fn removed_mesh_system(
    mut render_queue: ResMut<RenderQueueResource>,
    mut removed: RemovedComponents<MeshComponent>,
) {
    for entity in removed.read() {
        debug!(target: "mesh_sync", "Removing mesh for entity {:?}", entity);
        render_queue.remove_scene_object(&entity);
    }
}
