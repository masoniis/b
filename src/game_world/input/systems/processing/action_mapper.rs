use crate::game_world::input::{
    resources::{action::ActionStateResource, buttons::Buttons, input_action_map::Input},
    InputActionMapResource,
};
use bevy_ecs::prelude::{Res, ResMut};
use winit::{event::MouseButton, keyboard::PhysicalKey};

/// A system that translates the raw state from `Buttons` resources into abstract,
/// stateful actions in `ActionStateResource`, using the bindings from `InputActionMapResource`.
pub fn update_action_state_system(
    // Input state
    keyboard_input: Res<Buttons<PhysicalKey>>,
    mouse_input: Res<Buttons<MouseButton>>,
    input_map: Res<InputActionMapResource>,

    // Output state
    mut action_state: ResMut<ActionStateResource>,
) {
    action_state.clear(); // clear previous frame stale state

    // INFO: ---------------------------------
    //         Handle keyboard buttons
    // ---------------------------------------

    for key_code in keyboard_input.iter_current() {
        if let Some(action) = input_map.get_action(&Input::Key(*key_code)) {
            if keyboard_input.was_pressed(*key_code) {
                action_state.press(*action);
            }
            action_state.hold(*action);
        }
    }

    for key_code in keyboard_input.iter_previous() {
        if !keyboard_input.is_down(*key_code) {
            if let Some(action) = input_map.get_action(&Input::Key(*key_code)) {
                action_state.release(*action);
            }
        }
    }

    // INFO: ------------------------------
    //         Handle mouse buttons
    // ------------------------------------

    for button in mouse_input.iter_current() {
        if let Some(action) = input_map.get_action(&Input::MouseButton(*button)) {
            if mouse_input.was_pressed(*button) {
                action_state.press(*action);
            }
            action_state.hold(*action);
        }
    }

    for button in mouse_input.iter_previous() {
        if !mouse_input.is_down(*button) {
            if let Some(action) = input_map.get_action(&Input::MouseButton(*button)) {
                action_state.release(*action);
            }
        }
    }
}
