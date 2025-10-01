use crate::ecs_modules::input::{
    resources::{action::ActionStateResource, input::InputResource, input_action_map::Input},
    InputActionMapResource,
};
use bevy_ecs::prelude::{Res, ResMut};

/// A system that translates the raw state from `InputResource` into abstract,
/// stateful actions in `ActionStateResource`, using the bindings from `InputActionMapResource`.
pub fn update_action_state_system(
    input: Res<InputResource>,
    input_map: Res<InputActionMapResource>,
    mut action_state: ResMut<ActionStateResource>,
) {
    action_state.clear(); // clear previous frame stale state

    // INFO: -----------------------------------
    //        Handling key-based actions
    // -----------------------------------------

    // Process held down keys to determine `pressed` and `ongoing` states
    for key_code in input.iter_current_keys() {
        if let Some(action) = input_map.get_action(&Input::Key(*key_code)) {
            if input.was_key_pressed(*key_code) {
                action_state.press(*action);
            }
            action_state.hold(*action);
        }
    }

    // Process keys that were released to update `ended` and `ongoing`.
    for key_code in input.iter_previous_keys() {
        if !input.get_current_keys().contains(key_code) {
            if let Some(action) = input_map.get_action(&Input::Key(*key_code)) {
                action_state.release(*action);
            }
        }
    }

    // INFO: -----------------------------------
    //        Handling mouse-based actions
    // -----------------------------------------

    // Process held down keys to determine `pressed` and `ongoing` states
    for button in input.iter_current_mouse_buttons() {
        if let Some(action) = input_map.get_action(&Input::MouseButton(*button)) {
            if !input.get_previous_mouse_buttons().contains(button) {
                action_state.press(*action);
            }
            action_state.hold(*action);
        }
    }

    // Process keys that were released to update `ended` and `ongoing`.
    for button in input.iter_previous_mouse_buttons() {
        if !input.get_current_mouse_buttons().contains(button) {
            if let Some(action) = input_map.get_action(&Input::MouseButton(*button)) {
                action_state.release(*action);
            }
        }
    }
}
