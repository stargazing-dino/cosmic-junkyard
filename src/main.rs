use std::time::Duration;

use app::AppPlugin;
use bevy::{asset::ChangeWatcher, prelude::*, window::PresentMode};

mod app;
mod assets;
mod saving;
mod utility;

// TODO: For reactive UI, we shouldn't rely on these.
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
                    watch_for_changes: Some(
                        ChangeWatcher::with_delay(Duration::from_millis(50)).unwrap(),
                    ),
                    ..default()
                }),
        )
        .add_plugins(AppPlugin)
        .run();
}
