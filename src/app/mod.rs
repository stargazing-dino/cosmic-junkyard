#![allow(dead_code)]
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use crate::assets::{
    backgrounds::BackgroundCollection, fonts::FontCollection, images::ImageCollection,
    music::MusicCollection, sounds::SoundCollection, ui_sounds::UiSoundCollection,
};

use self::{
    app_state_machine::{AppState, AppStateMachinePlugin},
    game::GamePlugin,
    level_selection::LevelSelectionPlugin,
    main_menu::MainMenuPlugin,
    navigation::NavigationPlugin,
    player_input::PlayerInputPlugin,
    settings_dialog::SettingsDialogPlugin,
};

mod app_state_machine;
mod game;
mod game_levels;
mod level_selection;
mod main_menu;
mod navigation;
mod player_input;
mod settings_dialog;
mod theme;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AppStateMachinePlugin)
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::InGameLevel),
            )
            .add_collection_to_loading_state::<_, MusicCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, UiSoundCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, SoundCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, BackgroundCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, ImageCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, FontCollection>(AppState::AssetLoading)
            .add_plugins((
                NavigationPlugin,
                PlayerInputPlugin,
                MainMenuPlugin,
                GamePlugin,
                LevelSelectionPlugin,
                SettingsDialogPlugin,
            ));
    }
}
