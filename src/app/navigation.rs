use bevy::prelude::*;

use super::app_state_machine::AppTransitionEvent;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, back_button);
    }
}

#[derive(Component)]
pub struct BackButton;

pub fn back_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<BackButton>),
    >,
    mut transition_writer: EventWriter<AppTransitionEvent>,
) {
    for interaction in &mut interaction_query {
        // check if interaction is clicked
        if *interaction != Interaction::Pressed {
            continue;
        };

        transition_writer.send(AppTransitionEvent::GoBack);
    }
}
