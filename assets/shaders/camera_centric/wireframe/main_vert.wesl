import super::common::{camera, objects, VertexInput, VertexOutput};

@vertex
fn vs_main(
    model_in: VertexInput,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let model = objects[instance_index].model_matrix;
    out.clip_position = camera.view_proj * model * vec4(model_in.position, 1.0);

    return out;
}
