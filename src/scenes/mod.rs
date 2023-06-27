#![allow(dead_code)]
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use self::{
    game::GamePlugin, level_selection::LevelSelectionPlugin, main_menu::MainMenuPlugin,
    player_input::PlayerInputPlugin, settings_dialog::SettingsDialogPlugin,
};

mod game;
mod level_selection;
mod main_menu;
mod player_input;
mod settings_dialog;

pub struct GameStateMachinePlugin;

impl Plugin for GameStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::LevelSelection),
            )
            .add_event::<TransitionEvent>()
            .init_resource::<PreviousState<GameState>>()
            .add_plugin(PlayerInputPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(LevelSelectionPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(SettingsDialogPlugin)
            .add_system(apply_transition);
    }
}

// We should probably prefer a stack or something for this. Eh.
#[derive(Resource, Default, Debug)]
pub struct PreviousState<S: States>(pub Option<S>);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,

    /// The user is selecting a level to play
    LevelSelection,

    /// The user is configuring their level
    Preparation,

    /// The game/simulation is running
    Playing,

    /// The game is paused
    Paused,

    /// The player beat the level
    LevelComplete,

    /// The player failed the level
    LevelFailed,
}

pub enum TransitionEvent {
    NewGame,
    SelectLevel(usize),
    PrepareLevel,
    RetryLevel,
    StartPlay,
    PauseGame,
    UnpauseGame,
    LevelCompleted,
    LevelFailed,
    NextLevel(usize),
    Quit,
}

fn apply_transition(
    mut previous_state: ResMut<PreviousState<GameState>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut transition_event_reader: EventReader<TransitionEvent>,
) {
    use GameState::*;

    for transition_event in transition_event_reader.iter() {
        let next_queued = match (current_state.0.clone(), transition_event) {
            (Start, TransitionEvent::NewGame) => LevelSelection,
            (LevelSelection, TransitionEvent::SelectLevel(level)) => {
                // Prolly want to load level data and stuff
                Preparation
            }
            (LevelFailed, TransitionEvent::RetryLevel) => Preparation,
            (LevelComplete, TransitionEvent::RetryLevel) => Preparation,
            (PlanetaryConfiguration, TransitionEvent::PrepareLevel) => PlanetaryConfiguration,
            (PlanetaryConfiguration, TransitionEvent::StartPlay) => Playing,
            (Playing, TransitionEvent::LevelCompleted) => LevelComplete,
            (Playing, TransitionEvent::LevelFailed) => LevelFailed,
            (LevelComplete, TransitionEvent::NextLevel(level)) => LevelSelection,
            (Preparation, TransitionEvent::PauseGame) | (Playing, TransitionEvent::PauseGame) => {
                Paused
            }
            (Pause, TransitionEvent::UnpauseGame) => previous_state.0.as_ref().unwrap().clone(),
            _ => panic!("Invalid state transition"),
        };

        previous_state.0 = Some(current_state.0.clone());
        next_state.0 = Some(next_queued);
    }
}
