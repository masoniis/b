/// Vertex representation that the meshes from the main code use.
struct WorldVertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
    @location(4) texture_index: u32,
};

/// Vertex representation that the wireframes from the main code use.
struct WireframeVertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};
