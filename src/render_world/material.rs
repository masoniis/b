use serde::Deserialize;
use std::collections::BTreeMap;

// These are definitions that mirror the definitions for ron files seen in the shader assets folder

#[derive(Debug, Deserialize)]
pub struct MaterialDefinition {
    pub vertex_buffers: Vec<VertexBufferDef>,
    pub bind_group_layouts: BTreeMap<u32, BindGroupLayoutDef>,
}

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

#[derive(Debug, Deserialize)]
pub struct BindGroupLayoutDef {
    pub bindings: Vec<BindingDef>,
}

#[derive(Debug, Deserialize)]
pub struct BindingDef {
    pub binding: u32,
    pub ty: String,
    pub visibility: Vec<String>,
    pub buffer_options: Option<BufferOptionsDef>,
}

#[derive(Debug, Deserialize)]
pub struct BufferOptionsDef {
    pub ty: String,
    pub has_dynamic_offset: bool,
}
