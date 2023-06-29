use bevy::prelude::*;

use crate::PreviousState;

pub struct GameStateMachinePlugin;

impl Plugin for GameStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameTransitionEvent>()
            .init_resource::<PreviousState<GameState>>()
            .add_system(apply_transition);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Preparing,

    Playing,

    Paused,

    /// The player beat the level
    Completed,

    /// The player failed the level
    Failed,
}

pub enum GameTransitionEvent {
    Play,
    Prepare,
    Pause,
    Unpause,
    Complete,
    Fail,
}

fn apply_transition(
    mut previous_state: ResMut<PreviousState<GameState>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut transition_event_reader: EventReader<GameTransitionEvent>,
) {
    for transition_event in transition_event_reader.iter() {
        let next_queued = match (current_state.0.clone(), transition_event) {
            (GameState::Playing, GameTransitionEvent::Pause) => GameState::Paused,
            (GameState::Paused, GameTransitionEvent::Unpause) => GameState::Playing,
            (GameState::Playing, GameTransitionEvent::Complete) => GameState::Completed,
            (GameState::Playing, GameTransitionEvent::Fail) => GameState::Failed,
            _ => panic!("Invalid state transition"),
        };

        previous_state.0 = Some(current_state.0.clone());
        next_state.0 = Some(next_queued);
    }
}
