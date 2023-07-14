use bevy::prelude::*;

use crate::{
    assets::{backgrounds::BackgroundCollection, fonts::FontCollection},
    utility::despawn_components,
};

use super::{
    app_state_machine::AppTransitionEvent,
    game_levels::LEVELS,
    navigation::BackButton,
    theme::{change_button_colors, NORMAL_BUTTON},
    AppState,
};

pub struct LevelSelectionPlugin;

impl Plugin for LevelSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LevelSelection), setup)
            .add_systems(
                Update,
                (change_button_colors, select_level_action)
                    .run_if(in_state(AppState::LevelSelection)),
            )
            .add_systems(
                OnExit(AppState::LevelSelection),
                despawn_components::<LevelSelectionMarker>,
            );
    }
}

#[derive(Component)]
pub struct LevelSelectionMarker;

#[derive(Component)]
pub struct SelectLevelButton(usize);

fn setup(
    mut commands: Commands,
    background_collection: Res<BackgroundCollection>,
    font_collection: Res<FontCollection>,
) {
    commands.spawn((Camera2dBundle::default(), LevelSelectionMarker));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            LevelSelectionMarker,
            UiImage::new(background_collection.green_nebula_1.clone()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(64.0),
                            width: Val::Px(160.0),
                            margin: UiRect::all(Val::Px(16.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            align_self: AlignSelf::FlexEnd,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    BackButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("Back"),
                        TextStyle {
                            font: font_collection.comfortaa_bold.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    LEVELS.into_iter().enumerate().for_each(|(index, _)| {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(160.0),
                                        height: Val::Px(64.0),
                                        margin: UiRect::all(Val::Px(16.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                SelectLevelButton(index),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    format!("Level {}", index + 1),
                                    TextStyle {
                                        font: font_collection.comfortaa_bold.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ));
                            });
                    });
                });
        });
}

fn select_level_action(
    mut interaction_query: Query<
        (&Interaction, &SelectLevelButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut transition_writer: EventWriter<AppTransitionEvent>,
) {
    for (interaction, button) in &mut interaction_query {
        // check if interaction is clicked
        if *interaction != Interaction::Pressed {
            continue;
        };

        transition_writer.send(AppTransitionEvent::SelectLevel(Some(button.0)));
    }
}
