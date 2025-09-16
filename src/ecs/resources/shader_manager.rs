use crate::graphics::shader_program::ShaderProgram;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Scene,
    Text,
}

pub struct ShaderManagerResource {
    shaders: HashMap<ShaderType, ShaderProgram>,
}

impl ShaderManagerResource {
    pub fn new() -> Result<Self, String> {
        let mut shaders = HashMap::new();

        let scene_shader = ShaderProgram::new(
            "src/assets/shaders/scene/simple.vert",
            "src/assets/shaders/scene/simple.frag",
        )?;
        shaders.insert(ShaderType::Scene, scene_shader);

        let text_shader = ShaderProgram::new(
            "src/assets/shaders/text/text.vert",
            "src/assets/shaders/text/text.frag",
        )?;
        shaders.insert(ShaderType::Text, text_shader);

        Ok(ShaderManagerResource { shaders })
    }

    pub fn get(&self, shader_type: ShaderType) -> Option<&ShaderProgram> {
        self.shaders.get(&shader_type)
    }

    pub fn get_mut(&mut self, shader_type: ShaderType) -> Option<&mut ShaderProgram> {
        self.shaders.get_mut(&shader_type)
    }

    pub fn delete(&self) {
        for (_, shader) in &self.shaders {
            shader.delete();
        }
    }
}
