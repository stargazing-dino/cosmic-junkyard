use bevy::prelude::*;

use crate::{
    utility::{button_interactions, despawn_components},
    NORMAL_BUTTON,
};

use super::GameState;

pub struct LevelSelectionPlugin;

impl Plugin for LevelSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup,).in_schedule(OnEnter(GameState::LevelSelection)))
            .add_systems(
                (button_interactions,).distributive_run_if(in_state(GameState::LevelSelection)),
            )
            .add_systems(
                (despawn_components::<LevelSelectionMarker>,)
                    .in_schedule(OnExit(GameState::LevelSelection)),
            );
    }
}

#[derive(Component)]
pub struct LevelSelectionMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), LevelSelectionMarker));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LevelSelectionMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        // width: Val::Px(150.0),
                        // height: Val::Px(65.0),
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
