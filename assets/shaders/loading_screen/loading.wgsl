struct TimeUniform {
    total_elapse: f32,
};

// TODO: Need to pass resolution as a uniform, should we useful for scene shader as well.

@group(0) @binding(0)
var<uniform> time: TimeUniform;

/// A quad of positions to cover the whole screen
var<private> POSITIONS: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
    vec2<f32>(-1.0, -1.0), // Bottom-left
    vec2<f32>(1.0, -1.0),  // Bottom-right
    vec2<f32>(-1.0, 1.0),  // Top-left
    vec2<f32>(1.0, 1.0)    // Top-right
);

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(POSITIONS[in_vertex_index], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / vec2<f32>(800.0, 600.0); // Assuming a resolution for now
    let color = vec3<f32>(sin(uv.x + time.total_elapse), uv.y, 0.5); // Simple animation
    return vec4<f32>(color, 1.0);
}
