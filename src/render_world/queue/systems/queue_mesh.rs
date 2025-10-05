use crate::render_world::{
    extract::{
        extract_meshes::{RenderMeshComponent, RenderTransformComponent},
        RenderCameraResource,
    },
    queue::{Opaque3dRenderPhase, PhaseItem, RenderQueueResource},
};
use bevy_ecs::prelude::*;

/// The system responsible for populating the `RenderQueueResource`.
///
/// Performs a query for all entities that have been extracted into the render
/// world and adds them to a list of draw calls for the renderer to consume.
pub fn queue_mesh_system(
    // Input
    camera_info: Res<RenderCameraResource>,
    meshes_query: Query<(Entity, &RenderMeshComponent, &RenderTransformComponent)>,

    // Output
    mut render_queue: ResMut<RenderQueueResource>,
    mut opaque_phase: ResMut<Opaque3dRenderPhase>,
) {
    opaque_phase.items.clear();

    // It's often a good idea to clear the queue at the start of the system
    // to ensure no data from the previous frame persists.
    render_queue.clear_object_queue();
    //
    // for (entity, mesh, transform) in meshes_query.iter() {
    //     // info!("Queueing mesh for entity: {:?}", entity);
    //     // info!(" - Mesh Handle: {:?}", mesh.mesh_handle);
    //     // info!(" - Transform: {:?}", transform.transform);
    //     let queued_draw = QueuedDraw {
    //         entity,
    //         mesh_handle: mesh.mesh_handle,
    //         instance_count: 1, // Assuming 1 instance per entity for now
    //         transform: transform.transform,
    //     };
    //     render_queue.add_scene_object(entity, queued_draw);
    // }

    let camera_position = camera_info.world_position;

    for (entity, _mesh, transform) in meshes_query.iter() {
        // TODO: Implement frustum culling. Check if the mesh's bounding box
        // is visible from the camera's perspective.
        // let is_visible = frustum_cull(transform, &camera_info.frustum);
        // if !is_visible {
        //     continue;
        // }

        let object_position = transform.transform.w_axis.truncate();
        let distance_from_camera = (object_position - camera_position).length_squared();

        opaque_phase.items.push(PhaseItem {
            entity,
            distance: distance_from_camera,
        });
    }

    // --- 3. Sorting ---
    // For opaque objects, sort front-to-back (descending distance) to maximize early-Z culling.
    opaque_phase
        .items
        .sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
}
