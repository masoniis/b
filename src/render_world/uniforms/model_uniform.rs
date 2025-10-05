use glam::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    pub model_mat: [[f32; 4]; 4],
    _padding: [u8; 192],
}

impl ModelUniform {
    pub fn default() -> Self {
        Self {
            model_mat: Mat4::IDENTITY.to_cols_array_2d(),
            _padding: [0; 192],
        }
    }

    pub fn new(model_matrix: [[f32; 4]; 4]) -> Self {
        Self {
            model_mat: model_matrix,
            _padding: [0; 192],
        }
    }

    pub fn update_model_mat(&mut self, proj: glam::Mat4) {
        self.model_mat = proj.to_cols_array_2d();
    }
}
