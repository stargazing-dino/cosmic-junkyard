#![allow(dead_code)]
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use crate::assets::{music::MusicCollection, sounds::SoundCollection};

use self::{
    app_state_machine::{AppState, AppStateMachinePlugin},
    game::GamePlugin,
    level_selection::LevelSelectionPlugin,
    main_menu::MainMenuPlugin,
    player_input::PlayerInputPlugin,
    settings_dialog::SettingsDialogPlugin,
};

mod app_state_machine;
mod game;
mod game_levels;
mod level_selection;
mod main_menu;
mod player_input;
mod settings_dialog;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AppStateMachinePlugin)
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::MainMenu),
            )
            .add_collection_to_loading_state::<_, MusicCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, SoundCollection>(AppState::AssetLoading)
            .add_plugin(PlayerInputPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(LevelSelectionPlugin)
            .add_plugin(SettingsDialogPlugin);
    }
}
