import package::world::lib::vertex::{WorldVertex};
import super::lib::sun_camera_uniform::{SunCameraUniform};

struct ObjectData {
    model_matrix: mat4x4<f32>,
};
@group(1) @binding(0)
var<storage, read> objects: array<ObjectData>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> shadow_camera: SunCameraUniform;

@vertex
fn vs_main(
    vertex: WorldVertex,
    @builtin(instance_index) index: u32
) -> VertexOutput {
    var out: VertexOutput;

    let model = objects[index].model_matrix;
    let world_position = model * vec4<f32>(vertex.position, 1.0);

    out.clip_position = shadow_camera.view_proj * world_position;

    return out;
}
