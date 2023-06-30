use bevy::prelude::*;

use crate::{app::app_state_machine::AppState, utility::despawn_components, NORMAL_BUTTON};

use self::{input::InputPlugin, music::MusicPlugin};

use super::game_state_machine::{GameState, GameTransitionEvent};

mod input;
mod music;

pub struct PreparePlugin;

impl Plugin for PreparePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MusicPlugin)
            .add_plugin(InputPlugin)
            .add_systems((setup_ui,).in_schedule(OnEnter(GameState::Preparing)))
            .add_systems(
                (despawn_components::<StartLevelButton>,).in_schedule(OnExit(GameState::Preparing)),
            )
            .add_system(
                start_play_button.run_if(
                    in_state(AppState::InGameLevel).and_then(in_state(GameState::Preparing)),
                ),
            );
    }
}

#[derive(Component)]
pub struct PreparingMarker;

#[derive(Component)]
pub struct StartLevelButton;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(24.)),
                    size: Size::all(Val::Percent(100.)),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::End,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            PreparingMarker,
        ))
        .with_children(|parent| {
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
                    StartLevelButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn start_play_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<StartLevelButton>),
    >,
    mut transition_writer: EventWriter<GameTransitionEvent>,
) {
    for interaction in &mut interaction_query {
        // check if interaction is clicked
        if *interaction != Interaction::Clicked {
            continue;
        };

        transition_writer.send(GameTransitionEvent::Play);
    }
}
