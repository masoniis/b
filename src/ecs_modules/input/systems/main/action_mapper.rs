use crate::ecs_modules::input::{
    resources::{action::ActionStateResource, input::InputResource, input_action_map::Input},
    InputActionMapResource,
};
use bevy_ecs::prelude::{Res, ResMut};

/// A system to convert inputs to actions (should be the only system to do this)
pub fn update_action_state_system(
    input: Res<InputResource>,
    input_map: Res<InputActionMapResource>,
    mut action_state: ResMut<ActionStateResource>,
) {
    action_state.clear();

    for (input_type, action) in input_map.0.iter() {
        let (was_pressed, is_down) = match input_type {
            Input::Key(key_code) => (
                input.was_key_pressed(*key_code),
                input.is_key_down(*key_code),
            ),
            Input::MouseButton(_button) => {
                // TODO: Mouse button support
                (false, false)
            }
        };

        if was_pressed {
            action_state.press(*action);
        }

        if is_down {
            action_state.hold(*action);
        } else {
            action_state.release(*action);
        }
    }
}
