#[derive(Actionlike, PartialEq, Clone, Copy, Debug)]
pub enum PlayerAction {
    Pause,

    Continue,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(KeyCode::Space, Continue);
        input_map.insert(KeyCode::Space, Pause);

        input_map
    }
}

fn resume_play(
    mut query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut transition_writer: EventWriter<AppTransitionEvent>,
) {
    for (action_state, _player) in query.iter_mut() {
        if action_state.just_pressed(PlayerAction::Continue) {
            transition_writer.send(AppTransitionEvent::UnpauseGame);
        }
    }
}

fn pause_play(
    mut query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut transition_writer: EventWriter<AppTransitionEvent>,
) {
    for (action_state, _player) in query.iter_mut() {
        if action_state.just_pressed(PlayerAction::Pause) {
            transition_writer.send(AppTransitionEvent::PauseGame);
        }
    }
}
