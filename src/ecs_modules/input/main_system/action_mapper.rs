// TODO: Create a single source that maps input to action

// #[derive(Resource, Default)]
// pub struct ActionState {
//     pub pressed: HashSet<GameAction>,
//     pub just_pressed: HashSet<GameAction>,
//     pub just_released: HashSet<GameAction>,
// }
//
// /// This ONE system updates the ActionState resource every frame.
// pub fn update_action_state_system(
//     input: Res<InputResource>,
//     input_map: Res<InputMap>,
//     mut action_state: ResMut<ActionState>,
// ) {
//     // Logic to clear just_pressed/just_released and update the sets...
//     // This would be similar to how `end_of_frame_input_maintenance_system` works
// }
