use bevy::prelude::*;
use scenes::GameStateMachinePlugin;

mod assets;
mod saving;
mod scenes;
mod utility;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cosmic Junkyard".to_string(), // ToDo
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        canvas: Some("#bevy".to_owned()),
                        position: WindowPosition::At((0, 0).into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    // This tells the AssetServer to watch for changes to assets.
                    // It enables our scenes to automatically reload in game when we modify their files.
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(GameStateMachinePlugin)
        .run();
}
