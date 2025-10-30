import package::opaque::common::{camera, myTexture, mySampler, objects, VertexInput, VertexOutput};

@vertex
fn vs_main(
    model_in: VertexInput,
    @builtin(instance_index) index: u32
) -> VertexOutput {
    var out: VertexOutput;

		// get model matrix from buffer and apply it
    let model = objects[index].model_matrix;
    out.clip_position = camera.view_proj * model * vec4<f32>(model_in.position, 1.0);

		// pass other attributes to the fragment shader
    out.normal = model_in.normal;
    out.color = model_in.color;
    out.tex_coords = model_in.tex_coords;
    out.texture_index = model_in.texture_index;

    return out;
}
