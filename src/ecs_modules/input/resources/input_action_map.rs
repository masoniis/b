use bevy_ecs::prelude::Resource;
use std::collections::hash_map::{HashMap, Iter};
use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};

use super::super::types::game_action::GameAction;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Input {
    Key(PhysicalKey),
    MouseButton(MouseButton),
}

/// A map from input keys to an action. Set as a resource
/// so that it can be configured by systems at runtime.
#[derive(Debug, Resource)]
pub struct InputActionMapResource {
    bindings: HashMap<Input, GameAction>,
}

impl InputActionMapResource {
    /// Gets the game action associated with a given input, if one exists.
    pub fn get_action(&self, input: &Input) -> Option<&GameAction> {
        self.bindings.get(input)
    }

    /// Provides an iterator over all the currently configured input bindings.
    pub fn iter<'a>(&'a self) -> Iter<'a, Input, GameAction> {
        self.bindings.iter()
    }
}

impl Default for InputActionMapResource {
    fn default() -> Self {
        Self {
            bindings: HashMap::from([
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyW)),
                    GameAction::MoveForward,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyS)),
                    GameAction::MoveBackward,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyA)),
                    GameAction::MoveLeft,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyD)),
                    GameAction::MoveRight,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::F3)),
                    GameAction::ToggleDiagnostics,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::ShiftLeft)),
                    GameAction::Shift,
                ),
            ]),
        }
    }
}
