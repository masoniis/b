# Shader .ron guide

A `.ron` file acts as the bridge between a shader's logic (`.wgsl`) and the engine's Rust code. It explicitly defines the data the shader expects, such as vertex formats and resource bindings. This allows the renderer to automatically generate the correct `wgpu::RenderPipeline` without hardcoded values, making the system robust and easy to extend.

## File Structure

A material definition has two main sections: vertex_buffers and bind_group_layouts.

```ron
(
    vertex_buffers: [ ... ],
    bind_group_layouts: { ... },
)
```

### `vertex_buffers`

This is an array that describes each vertex buffer the shader's vertex entry point expects.

- **`step_mode`**: Defines how the GPU advances through this buffer.
  - `"Vertex"`: Advance for every vertex (per-vertex data like position, normals, UVs).
  - `"Instance"`: Advance for every instance drawn (per-instance data like model matrices, colors).
- **`attributes`**: An array describing each attribute within the buffer.
  - **`name`**: A human-readable name for the attribute (e.g., "Position").
  - **`location`**: The corresponding `@location(N)` index in the shader.
  - **`format`**: The data format of the attribute (e.g., `"Float32x3"`, `"Uint32"`).

### `bind_group_layouts`

This is a map where the key is the `@group(N)` index and the value describes the layout for that bind group.

- **`bindings`**: An array describing each resource binding within the group.
  - **`binding`**: The corresponding `@binding(N)` index in the shader.
  - **`ty`**: The general category of the resource (`"Buffer"`, `"Texture"`, `"Sampler"`).
  - **`visibility`**: An array specifying which shader stages can see this resource (`"Vertex"`, `"Fragment"`, `"Compute"`).
  - **`buffer_options`**: (Required if `ty` is `"Buffer"`)
    - **`ty`**: The specific kind of buffer (`"Uniform"`, `"Storage"`).
    - **`has_dynamic_offset`**: `true` or `false`.

### Conventions for Bind Groups

To improve ergonomics and performance, the engine reserves specific bind group slots for "well-known" data.

#### `@group(0)`: Per-View Data

This group is managed entirely by the engine and provides data that is constant for a single camera's view. A shader can opt-in to this data by declaring a compatible bind group at this slot.

- **Content**: Camera projection/view matrices, viewport size, simulation time.
- **Update Frequency**: Once per view, per frame.
- **Managed By**: The engine's renderer. **Do not** define this group in a `.material.ron` file.

#### `@group(1)`: Per-Material Data

This group should be used for data that is shared by all objects using this material.

- **Content**: Textures, samplers, material properties (e.g., color tint, roughness).
- **Update Frequency**: Once per material.
- **Managed By**: The `.material.ron` file. You should define the layout for this group here.

#### `@group(2)`: Per-Object Data

This group should be used for data that is unique to each individual object being drawn.

- **Content**: The object's model matrix, object-specific color overrides.
- **Update Frequency**: For every object drawn.
- **Managed By**: Can be defined in the `.material.ron` file or handled by a more specialized system (e.g., using dynamic uniform offsets).
