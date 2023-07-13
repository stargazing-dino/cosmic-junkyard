use bevy::prelude::*;

use crate::PreviousState;

pub struct AppStateMachinePlugin;

impl Plugin for AppStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_event::<AppTransitionEvent>()
            .init_resource::<PreviousState<AppState>>()
            .add_systems(Update, apply_transition);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    AssetLoading,

    /// The user is selecting a level to play
    MainMenu,

    /// The game is paused
    Settings,

    /// The user wants to navigate to level selection
    LevelSelection,

    /// The user is in the game
    InGameLevel,
}

#[derive(Event)]
pub enum AppTransitionEvent {
    Continue,

    SelectLevel(Option<usize>),

    Retry,

    NextLevel(usize),

    Settings,

    GoBack,
}

fn apply_transition(
    mut previous_state: ResMut<PreviousState<AppState>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut transition_event_reader: EventReader<AppTransitionEvent>,
) {
    for transition_event in transition_event_reader.iter() {
        let next_queued = match (current_state.clone(), transition_event) {
            // Main Menu Transitions
            (AppState::MainMenu, AppTransitionEvent::Continue) => {
                // TODO: Get the last level played and pass it through
                AppState::InGameLevel
            }
            (AppState::MainMenu, AppTransitionEvent::SelectLevel(None)) => AppState::LevelSelection,
            (AppState::MainMenu, AppTransitionEvent::Settings) => AppState::Settings,

            // Settings Transitions

            // Level Selection Transitions
            (AppState::LevelSelection, AppTransitionEvent::SelectLevel(_level)) => {
                // TODO: Pass through the level selected
                AppState::InGameLevel
            }

            // In Game Transitions
            (AppState::InGameLevel, AppTransitionEvent::NextLevel(_level)) => {
                // TODO: Pass through the level selected
                AppState::InGameLevel
            }
            (AppState::InGameLevel, AppTransitionEvent::Retry) => AppState::InGameLevel,

            // Default Transitions
            // TODO: Curious if this will cause a loop where previous is what
            // previous was previously, lol
            (_, AppTransitionEvent::GoBack) => {
                previous_state.0.clone().unwrap_or(AppState::MainMenu)
            }
            _ => panic!("Invalid state transition"),
        };

        previous_state.0 = Some(current_state.clone());
        next_state.0 = Some(next_queued);
    }
}
