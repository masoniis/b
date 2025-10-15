use bevy_ecs::schedule::{ScheduleLabel, SystemSet};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderSchedule {
    /// A schedule that runs once when the render world is first created.
    /// Needed for the one-time wgpu initialization.
    Startup,

    /// A schedule that runs every frame, before the main schedule. It is
    /// unique in that it has exclusive access to the simulation world.
    Extract,

    /// The main, per-frame schedule for the render world.
    /// This will contain our ordered system sets.
    Main,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RenderSet {
    /// This phase runs **first**. Its job is to prepare data for the GPU.
    /// This includes writing to uniform buffers (like camera matrices or lighting data)
    /// and creating bind groups.
    Prepare,

    /// This phase runs **after `Prepare`**. Its job is to record rendering
    /// commands into a command buffer. Systems here will create render passes
    /// and tell the GPU which pipelines and bind groups to use.
    Queue,

    /// This phase runs **last**. It has one job: take the command buffer(s)
    /// created in the `Queue` phase and submit them to the GPU for execution.
    Render,
}
