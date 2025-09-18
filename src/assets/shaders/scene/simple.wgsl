@vertex fn vs_main(@location(0) in_position : vec3<f32>,
                   @location(1) in_color : vec3<f32>) ->
    @builtin(position) vec4<f32> {
  return vec4<f32>(in_position, 1.0);
}

@fragment fn fs_main(@builtin(position) in_position : vec4<f32>) ->
    @location(0) vec4<f32> {
  return vec4<f32>(in_position.x, in_position.y, in_position.z, 1.0);
}
