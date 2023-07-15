use bevy::prelude::*;

use crate::app::app_state_machine::AppState;

pub struct GameStateMachinePlugin;

impl Plugin for GameStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameTransitionEvent>()
            .add_systems(
                Update,
                from_none_state, // .run_if(not(in_state(AppState::InGameLevel))),
            )
            .add_systems(
                Update,
                game_transition.run_if(in_state(AppState::InGameLevel)),
            );
    }
}

#[derive(Resource, Default, Debug)]
pub struct PreviousState<S: States>(pub Option<S>);

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

// I don't want
fn from_none_state(
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

fn game_transition(
    mut previous_state: Local<PreviousState<GameState>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut transition_event_reader: EventReader<GameTransitionEvent>,
) {
    for transition_event in transition_event_reader.iter() {
        let next_queued = match (current_state.clone(), transition_event) {
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
