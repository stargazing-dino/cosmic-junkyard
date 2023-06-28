use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::utility::despawn_components;

use super::{GameState, TransitionEvent};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup,).in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(button_colors.run_if(in_state(GameState::MainMenu)))
            .add_systems(
                (despawn_components::<MainMenuMarker>,).in_schedule(OnExit(GameState::MainMenu)),
            );
    }
}

#[derive(Component)]
pub struct MainMenuMarker;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainMenuMarker));

    let button_style = Style {
        size: Size {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
        },
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(24.)),
                    flex_direction: FlexDirection::Row,
                    size: Size::all(Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            MainMenuMarker,
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

fn button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut transition_writer: EventWriter<TransitionEvent>,
) {
    for (interaction, action) in &mut interaction_query {
        // check if interaction is clicked
        if *interaction != Interaction::Clicked {
            continue;
        };

        match action {
            MenuButtonAction::Continue => {
                // transition_writer.send(TransitionEvent::Continue);
            }
            MenuButtonAction::SelectLevel => {
                transition_writer.send(TransitionEvent::SelectLevel(None));
            }
            MenuButtonAction::Settings => {
                transition_writer.send(TransitionEvent::Settings);
            }
        }
    }
}
