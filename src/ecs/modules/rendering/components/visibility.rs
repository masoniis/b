use bevy_ecs::prelude::Component;

#[derive(Component, PartialEq)]
pub enum VisibilityComponent {
    Visible,
    Hidden,
}
