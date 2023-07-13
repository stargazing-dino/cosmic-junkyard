use bevy::prelude::*;

use crate::{app::app_state_machine::AppState, PreviousState};

pub struct GameStateMachinePlugin;

impl Plugin for GameStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameTransitionEvent>()
            .init_resource::<PreviousState<GameState>>()
            .add_systems(Update, in_game_transition)
            .add_systems(
                Update,
                apply_transition.run_if(in_state(AppState::InGameLevel)),
            );
    }
}

fn in_game_transition(
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if current_state.is_changed() {
        if *current_state == AppState::InGameLevel {
            next_state.set(GameState::AssetLoading);
        } else {
            next_state.set(GameState::None);
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    // We need this silliness because otherwise we'd immediately be in the AssetLoading
    // state which, after loading, would move us to the preparing state.
    // This would all be running in the background.
    #[default]
    None,

    // TODO: I'm curious if this should instead go on the AppState? So I can
    // keep this game state game specific naw mean?
    AssetLoading,

    Preparing,

    Playing,

    Paused,

    /// The player beat the level
    Completed,

    /// The player failed the level
    Failed,
}

#[derive(Event)]
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
        let next_queued = match (current_state.clone(), transition_event) {
            (GameState::Preparing, GameTransitionEvent::Play) => GameState::Playing,
            (GameState::Playing, GameTransitionEvent::Pause) => GameState::Paused,
            (GameState::Paused, GameTransitionEvent::Unpause) => GameState::Playing,
            (GameState::Playing, GameTransitionEvent::Complete) => GameState::Completed,
            (GameState::Playing, GameTransitionEvent::Fail) => GameState::Failed,
            _ => panic!("Invalid state transition"),
        };

        previous_state.0 = Some(current_state.clone());
        next_state.0 = Some(next_queued);
    }
}
