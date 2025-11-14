import super::common::{myTexture, mySampler, objects, VertexOutput};
import super::super::lib::camera_uniform::{camera};
import package::world::lib::vertex::{WorldVertex};

@vertex
fn vs_main(
    vertex: WorldVertex,
    @builtin(instance_index) index: u32
) -> VertexOutput {
    var out: VertexOutput;

    // get model matrix from buffer and apply it
    let model = objects[index].model_matrix;
    let world_position = model * vec4<f32>(vertex.position, 1.0);
    out.clip_position = camera.view_proj * world_position;

    // pass other attributes to the fragment shader
    out.normal = vertex.normal;
    out.color = vertex.color;
    out.tex_coords = vertex.tex_coords;
    out.texture_index = vertex.texture_index;
    out.distance_from_camera = length(world_position.xyz - camera.world_position);
    out.world_pos = world_position.xyz;

    return out;
}
