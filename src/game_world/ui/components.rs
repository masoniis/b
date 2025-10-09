use crate::prelude::*;
use bevy_ecs::{entity::Entity, prelude::Component};

// INFO: ----------------------
//         UI Hierarchy
// ----------------------------

/// A marker for the entity representing the ui root node.
///
/// !! Only a single root node should exist.
#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct Node; // marker for any entit in the ui tree

#[derive(Component)]
pub struct Parent(pub Entity);

#[derive(Component, Default)]
pub struct Children(pub Vec<Entity>);

// INFO: ------------------
//         Styling
// ------------------------

#[derive(Clone, Copy, Debug)]
pub enum Size {
    Px(f32),
    Percent(f32),
    Auto,
}

#[derive(Component)]
pub struct Style {
    pub width: Size,
    pub height: Size,
    pub justify_content: Option<taffy::style::JustifyContent>,
    pub align_items: Option<taffy::style::AlignItems>,
}

// Now, implement Default for your main Style component
impl Default for Style {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
            justify_content: None,
            align_items: None,
        }
    }
}

// INFO: -------------------------
//         Visual elements
// -------------------------------

#[derive(Component, Clone, Debug)]
pub enum UiBackground {
    SolidColor { color: [f32; 4] },
    // TODO: image support
    Image { color: [f32; 4] },
    // Image {
    //     texture: Handle<Image>,
    //     /// A color to tint the texture. Use white `[1.0, 1.0, 1.0, 1.0]` for no tint.
    //     tint: [f32; 4],
    // },
}

#[derive(Component, Clone)]
pub struct UiText {
    pub content: String,
    pub font_size: f32,
    pub color: [f32; 4],
}

// INFO: Output

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct CalculatedLayout {
    /// The absolute screen-space position (X, Y) of the node's top-left corner.
    pub position: Vec2,
    /// The absolute screen-space size (Width, Height) of the node.
    pub size: Vec2,
}
