import package::world::shadow::lib::sun_camera_uniform::{SunCameraUniform};
import super::super::lib::environment_uniform::{EnvironmentUniform};
import super::super::lib::camera_uniform::{CameraUniform};

// INFO: -----------------------
//         per-view data
// -----------------------------

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(1)
var<uniform> shadow_camera: SunCameraUniform;

@group(0) @binding(2)
var shadow_map: texture_depth_2d;

@group(0) @binding(3)
var shadow_sampler: sampler_comparison;

// INFO: ------------------------------
//         per-environment data
// ------------------------------------

@group(1) @binding(0)
var<uniform> env_params: EnvironmentUniform;

// INFO: ---------------------------
//         per-material data
// ---------------------------------

@group(2) @binding(0)
var myTexture: texture_2d_array<f32>;
@group(2) @binding(1)
var mySampler: sampler;

// INFO: -------------------------
//         per-object data
// -------------------------------

struct ObjectData {
    model_matrix: mat4x4<f32>,
};
@group(3) @binding(0)
var<storage, read> objects: array<ObjectData>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) @interpolate(flat) texture_index: u32,
    @location(4) distance_from_camera: f32,
    @location(5) world_pos: vec3<f32>,
};
