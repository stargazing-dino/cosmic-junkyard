use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        // app;
    }
}

#[derive(Component)]
pub struct Player;
