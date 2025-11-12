use crate::prelude::*;
use crate::simulation_world::block::TargetedBlock;
use crate::simulation_world::{
    camera::{ActiveCamera, CameraComponent},
    chunk::BreakVoxelEvent,
};
use bevy_ecs::prelude::{MessageWriter, Query, Res};
use bevy_ecs::system::ResMut;

/// Breaks a voxel based on current camera raycast.
///
/// Should be set to run on a certain input action.
pub fn raycast_break_voxel_event_system(
    active_camera: Res<ActiveCamera>,
    camera_query: Query<&CameraComponent>,
    mut break_voxel_writer: MessageWriter<BreakVoxelEvent>,
    mut targeted_block: ResMut<TargetedBlock>,
) {
    let Ok(cam) = camera_query.get(active_camera.0) else {
        warn!(
            "camera_control_system: ActiveCamera entity {:?} not found or has no CameraComponent.",
            active_camera.0
        );
        return;
    };

    if let Some(voxel_pos) = raycast_voxel(cam.position, cam.front) {
        info!("Breaking voxel at position: {:?}", voxel_pos);

        targeted_block.position = Some(voxel_pos);

        break_voxel_writer.write(BreakVoxelEvent {
            world_pos: voxel_pos,
        });
    }
}

/// Raycasts from the camera to find if a voxel is within range.
///
/// Returns the voxel position if found.
fn raycast_voxel(origin: Vec3, direction: Vec3) -> Option<IVec3> {
    let target_dist = 4.0;
    let target = origin + direction * target_dist;
    Some(target.round().as_ivec3())
}
