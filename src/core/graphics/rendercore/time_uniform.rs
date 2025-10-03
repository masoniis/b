use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct TimeUniform {
    pub total_time: f32,
    pub _padding: f32, // Pad to 8 bytes
}

impl TimeUniform {
    pub fn new() -> Self {
        Self {
            total_time: 0.0,
            _padding: 0.0,
        }
    }

    pub fn update_total_time(&mut self, total_time: f32) {
        self.total_time = total_time;
    }
}
