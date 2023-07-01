use bevy::prelude::*;

use crate::{
    assets::{backgrounds::BackgroundCollection, fonts::FontCollection},
    utility::{button_interactions, despawn_components},
    NORMAL_BUTTON,
};

use super::{app_state_machine::AppTransitionEvent, game_levels::LEVELS, AppState};

pub struct LevelSelectionPlugin;

impl Plugin for LevelSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup,).in_schedule(OnEnter(AppState::LevelSelection)))
            .add_systems(
                (button_interactions, select_level_action)
                    .distributive_run_if(in_state(AppState::LevelSelection)),
            )
            .add_systems(
                (despawn_components::<LevelSelectionMarker>,)
                    .in_schedule(OnExit(AppState::LevelSelection)),
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
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
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
            LEVELS.into_iter().enumerate().for_each(|(index, _)| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
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
        if *interaction != Interaction::Clicked {
            continue;
        };

        transition_writer.send(AppTransitionEvent::SelectLevel(Some(button.0)));
    }
}
