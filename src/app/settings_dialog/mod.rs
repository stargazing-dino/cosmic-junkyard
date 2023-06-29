use bevy::prelude::*;

use crate::utility::despawn_components;

use super::AppState;

pub struct SettingsDialogPlugin;

impl Plugin for SettingsDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup,).in_schedule(OnEnter(AppState::Settings)))
            .add_systems(
                (despawn_components::<SettingsDialogMarker>,)
                    .in_schedule(OnExit(AppState::Settings)),
            );
    }
}

#[derive(Component)]
pub struct SettingsDialogMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), SettingsDialogMarker));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(24.)),
                    flex_direction: FlexDirection::Column,
                    size: Size::all(Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            SettingsDialogMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        // flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Settings",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 56.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });

                    // TODO: back button
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            ..default()
                        },
                        background_color: Color::YELLOW.into(),
                        ..default()
                    });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            ..default()
                        },
                        background_color: Color::PINK.into(),
                        ..default()
                    });
                });
        });
}
