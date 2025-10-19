import package::scene::common::{camera, myTexture, mySampler, objects, VertexInput, VertexOutput};

@vertex
fn vs_main(
    model_in: VertexInput,
    @builtin(instance_index) index: u32
) -> VertexOutput {
    var out: VertexOutput;

    // Look up this instance's model matrix from the storage buffer
    let model = objects[index].model_matrix;

    // Standard vertex transformation
    out.clip_position = camera.view_proj * model * vec4<f32>(model_in.position, 1.0);

    // Pass other attributes to the fragment shader
    out.color = model_in.color;
    out.tex_coords = model_in.tex_coords;
    out.texture_index = model_in.texture_index;

    return out;
}
