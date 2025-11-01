use crate::simulation_world::input::types::simulation_action::SimulationAction;
use bevy_ecs::prelude::Resource;
use std::collections::hash_map::{HashMap, Iter};
use winit::{
    event::MouseButton,
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Input {
    Key(PhysicalKey),
    MouseButton(MouseButton),
}

/// A map from input keys to an action. Set as a resource
/// which means it can be configured by systems at runtime.
#[derive(Debug, Resource)]
pub struct InputActionMapResource {
    bindings: HashMap<Input, SimulationAction>,
}

impl InputActionMapResource {
    /// Gets the game action associated with a given input, if one exists.
    pub fn get_action(&self, input: &Input) -> Option<&SimulationAction> {
        self.bindings.get(input)
    }

    /// Provides an iterator over all the currently configured input bindings.
    pub fn iter<'a>(&'a self) -> Iter<'a, Input, SimulationAction> {
        self.bindings.iter()
    }
}

impl Default for InputActionMapResource {
    fn default() -> Self {
        Self {
            bindings: HashMap::from([
                // Core player movement
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyW)),
                    SimulationAction::MoveForward,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyS)),
                    SimulationAction::MoveBackward,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyA)),
                    SimulationAction::MoveLeft,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::KeyD)),
                    SimulationAction::MoveRight,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::ShiftLeft)),
                    SimulationAction::MoveFaster,
                ),
                // Misc
                (
                    Input::Key(PhysicalKey::Code(KeyCode::Escape)),
                    SimulationAction::TogglePause,
                ),
                // Debug/analysis tools
                (
                    Input::Key(PhysicalKey::Code(KeyCode::F1)),
                    SimulationAction::ToggleDiagnostics,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::Digit1)),
                    SimulationAction::ToggleDiagnostics,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::F2)),
                    SimulationAction::ToggleOpaqueWireframeMode,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::Digit2)),
                    SimulationAction::ToggleOpaqueWireframeMode,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::F3)),
                    SimulationAction::ToggleChunkBorders,
                ),
                (
                    Input::Key(PhysicalKey::Code(KeyCode::Digit3)),
                    SimulationAction::ToggleChunkBorders,
                ),
            ]),
        }
    }
}
