use serde::Deserialize;
use std::collections::BTreeMap;

// These are definitions that mirror the definitions for ron files seen in the shader assets folder

// INFO: ---------------------------------
//         Main material structure
// ---------------------------------------

#[derive(Debug, Deserialize)]
pub struct MaterialDefinition {
    pub vertex_buffers: Vec<VertexBufferDef>,
    pub bind_group_layouts: BTreeMap<u32, BindGroupLayoutDef>,
}

#[derive(Debug, Deserialize)]
pub struct BindGroupLayoutDef {
    pub bindings: Vec<BindingDef>,
}

// INFO: ------------------------------
//         Binding layout types
// ------------------------------------

#[derive(Debug, Deserialize)]
pub struct BindingDef {
    pub binding: u32,
    pub ty: String,
    pub visibility: Vec<String>,

    #[serde(default)]
    pub buffer_options: Option<BufferOptions>,

    #[serde(default)]
    pub texture_options: Option<TextureOptions>,

    #[serde(default)]
    pub sampler_options: Option<SamplerOptions>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BufferOptions {
    pub ty: String,
    pub has_dynamic_offset: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TextureOptions {
    pub sample_type: String,
    pub view_dimension: String,
    pub multisampled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SamplerOptions {
    pub ty: String,
}

// INFO: -----------------------------
//         Vertex buffer types
// -----------------------------------

#[derive(Debug, Deserialize)]
pub struct VertexBufferDef {
    pub step_mode: String,
    pub attributes: Vec<VertexAttributeDef>,
}

#[derive(Debug, Deserialize)]
pub struct VertexAttributeDef {
    pub name: String,
    pub location: u32,
    pub format: String,
}
