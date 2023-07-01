use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    assets::backgrounds::BackgroundCollection,
    utility::{button_interactions, despawn_components},
    NORMAL_BUTTON, TEXT_COLOR,
};

use super::{app_state_machine::AppTransitionEvent, AppState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup,).in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems(
                // TODO: Maybe this should be top level system?
                (button_interactions, main_menu_actions)
                    .distributive_run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                (despawn_components::<MainMenuMarker>,).in_schedule(OnExit(AppState::MainMenu)),
            );
    }
}

#[derive(Component)]
pub struct MainMenuMarker;

// All actions that can be triggered from a button click
#[derive(Component, Debug, Copy, Clone, EnumIter)]
enum MenuButtonAction {
    Continue,
    SelectLevel,
    Settings,
}

impl MenuButtonAction {
    fn text(&self) -> &str {
        match self {
            MenuButtonAction::Continue => "Continue",
            MenuButtonAction::SelectLevel => "Select Level",
            MenuButtonAction::Settings => "Settings",
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    background_collection: Res<BackgroundCollection>,
) {
    commands.spawn((Camera2dBundle::default(), MainMenuMarker));

    let button_style = Style {
        size: Size {
            width: Val::Px(240.0),
            height: Val::Px(65.0),
        },
        margin: UiRect::all(Val::Px(16.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(24.)),
                    flex_direction: FlexDirection::Row,
                    size: Size::all(Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            MainMenuMarker,
            UiImage::new(background_collection.blue_nebula_1.clone()),
        ))
        .with_children(|parent| {
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
                                    "Cosmic\nJunkyard",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 80.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });

                    // TODO: Picture
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

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexEnd,
                                ..default()
                            },
                            background_color: Color::CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            for action in MenuButtonAction::iter() {
                                parent
                                    .spawn((
                                        ButtonBundle {
                                            style: button_style.clone(),
                                            background_color: NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        action,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            action.text(),
                                            button_text_style.clone(),
                                        ));
                                    });
                            }
                        });
                });
        });
}

fn main_menu_actions(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut transition_writer: EventWriter<AppTransitionEvent>,
) {
    for (interaction, action) in &mut interaction_query {
        // check if interaction is clicked
        if *interaction != Interaction::Clicked {
            continue;
        };

        match action {
            MenuButtonAction::Continue => {
                transition_writer.send(AppTransitionEvent::Continue);
            }
            MenuButtonAction::SelectLevel => {
                transition_writer.send(AppTransitionEvent::SelectLevel(None));
            }
            MenuButtonAction::Settings => {
                transition_writer.send(AppTransitionEvent::Settings);
            }
        }
    }
}
