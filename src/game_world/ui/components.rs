use crate::prelude::*;
use bevy_ecs::{entity::Entity, prelude::Component};

// INFO: ----------------------
//         UI Hierarchy
// ----------------------------

#[derive(Component)]
pub struct UiRoot; // marker for the root of the layout

#[derive(Component)]
pub struct Node; // marker for any entit in the ui tree

#[derive(Component)]
pub struct Parent(pub Entity);

#[derive(Component)]
pub struct Children(pub Vec<Entity>);

// INFO: -----------------
//         Styling
// -----------------------

pub enum Size {
    Px(f32),
    Percent(f32),
    Auto,
}

#[derive(Component)]
pub struct Style {
    pub width: Size,
    pub height: Size,
}

// INFO: -------------------------
//         Visual elements
// -------------------------------

#[derive(Component)]
pub enum UiMaterial {
    SolidColor { color: [f32; 4] },
}

// INFO: Output

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct CalculatedLayout {
    /// The absolute screen-space position (X, Y) of the node's top-left corner.
    pub position: Vec2,
    /// The absolute screen-space size (Width, Height) of the node.
    pub size: Vec2,
}
