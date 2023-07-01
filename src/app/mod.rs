#![allow(dead_code)]
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use crate::assets::{
    backgrounds::BackgroundCollection, fonts::FontCollection, images::ImageCollection,
    music::MusicCollection, sounds::SoundCollection,
};

use self::{
    app_state_machine::{AppState, AppStateMachinePlugin, AppTransitionEvent},
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
            .add_system(back_button)
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::MainMenu),
            )
            .add_collection_to_loading_state::<_, MusicCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, SoundCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, BackgroundCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, ImageCollection>(AppState::AssetLoading)
            .add_collection_to_loading_state::<_, FontCollection>(AppState::AssetLoading)
            .add_plugin(PlayerInputPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(LevelSelectionPlugin)
            .add_plugin(SettingsDialogPlugin);
    }
}

#[derive(Component)]
pub struct BackButton;

fn back_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<BackButton>),
    >,
    mut transition_writer: EventWriter<AppTransitionEvent>,
) {
    for interaction in &mut interaction_query {
        // check if interaction is clicked
        if *interaction != Interaction::Clicked {
            continue;
        };

        transition_writer.send(AppTransitionEvent::GoBack);
    }
}
