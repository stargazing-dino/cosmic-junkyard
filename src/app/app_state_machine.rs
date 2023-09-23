use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

pub struct AppStateMachinePlugin;

impl Plugin for AppStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::MainMenu),
            )
            .add_event::<AppTransitionEvent>()
            .add_systems(Update, app_transition);
    }
}

use std::collections::VecDeque;

#[derive(Debug)]
pub struct StateStack<S: States> {
    queue: VecDeque<S>,
    max_size: usize,
}

impl Default for StateStack<AppState> {
    fn default() -> Self {
        Self::new(10)
    }
}

impl<S: States> StateStack<S> {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, state: S) {
        if self.queue.len() == self.max_size {
            self.queue.pop_front();
        }
        self.queue.push_back(state);
    }

    pub fn pop(&mut self) -> Option<S> {
        self.queue.pop_back()
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

fn app_transition(
    mut previous_state: Local<StateStack<AppState>>,
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
            (_, AppTransitionEvent::GoBack) => previous_state.pop().unwrap_or(AppState::MainMenu),
            _ => panic!("Invalid state transition"),
        };

        previous_state.push(current_state.clone());
        next_state.0 = Some(next_queued);
    }
}
