use crate::simulation_world::asset_management::MeshDeletionRequest;
use bevy_ecs::prelude::Resource;
use crossbeam::channel::{Receiver, Sender};

/// Resource for the Simulation World to send mesh deletions to render world
#[derive(Resource)]
pub struct SimToRenderSender(pub Sender<MeshDeletionRequest>);

/// Resource for the Render World to receive mesh deletions from sim world
#[derive(Resource)]
pub struct SimToRenderReceiver(pub Receiver<MeshDeletionRequest>);
