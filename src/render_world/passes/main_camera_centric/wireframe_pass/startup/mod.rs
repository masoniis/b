pub mod setup_wireframe_mesh;
pub mod setup_wireframe_pipeline;

pub use setup_wireframe_mesh::{setup_unit_wireframe_cube_mesh_system, DebugWireframeMesh};
pub use setup_wireframe_pipeline::{
    setup_wireframe_pipeline_and_buffers, WireframeObjectBuffer, WireframeObjectData,
    WireframePipeline,
};
