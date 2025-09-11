pub struct DeltaTimeResource {
    pub seconds: f32,
}

impl Default for DeltaTimeResource {
    fn default() -> Self {
        Self { seconds: 0.0 }
    }
}
