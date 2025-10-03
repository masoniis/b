use bevy_ecs::prelude::Component;

#[derive(Component, PartialEq)]
pub enum VisibilityComponent {
    Visible,
    Hidden,
}

impl VisibilityComponent {
    /// Toggles the visibility state in place.
    /// Visible -> Hidden
    /// Hidden -> Visible
    pub fn toggle(&mut self) {
        *self = match *self {
            VisibilityComponent::Visible => VisibilityComponent::Hidden,
            VisibilityComponent::Hidden => VisibilityComponent::Visible,
        };
    }
}
