import super::common::{camera, objects, VertexInput, VertexOutput};
import package::world::lib::vertex::{WireframeVertex};

@vertex
fn vs_main(
    vertex: WireframeVertex,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let model = objects[instance_index].model_matrix;
    out.clip_position = camera.view_proj * model * vec4(vertex.position, 1.0);

    return out;
}
