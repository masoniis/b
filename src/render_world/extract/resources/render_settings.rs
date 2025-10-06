use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct RenderSettingsResource {
    pub show_ui: bool,
}

impl Default for RenderSettingsResource {
    fn default() -> Self {
        Self { show_ui: true }
    }
}
