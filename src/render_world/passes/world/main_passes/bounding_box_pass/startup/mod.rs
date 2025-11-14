pub mod setup_bb_mesh;
pub mod setup_bb_pipeline;

pub use setup_bb_mesh::{setup_unit_bounding_box_mesh_system, DebugWireframeMesh};
pub use setup_bb_pipeline::{
    setup_bb_pipeline_and_buffers, WireframeObjectBuffer, WireframeObjectData, WireframePipeline,
};
