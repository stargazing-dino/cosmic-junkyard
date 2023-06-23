use bevy::prelude::*;
use bevy_asset_loader::prelude::LoadingStateAppExt;

use crate::{assets::music::MusicCollection, GameState};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, MusicCollection>(GameState::AssetLoading)
            .add_systems((setup,).in_schedule(OnEnter(GameState::Preparation)))
            .add_systems((teardown,).in_schedule(OnExit(GameState::Preparation)));
    }
}

#[derive(Resource)]
struct PrepareMusicController(Handle<AudioSink>);

fn setup(
    mut commands: Commands,
    music_collection: Res<MusicCollection>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let weak_handle = music_collection.goodbye_sweet_alien.clone();
    let strong_handle = audio_sinks.get_handle(&audio.play_with_settings(
        weak_handle,
        PlaybackSettings {
            repeat: true,
            ..default()
        },
    ));

    commands.insert_resource(PrepareMusicController(strong_handle));
}

fn teardown(
    mut commands: Commands,
    audio_sinks: Res<Assets<AudioSink>>,
    prepare_music_controller: ResMut<PrepareMusicController>,
) {
    if let Some(sink) = audio_sinks.get(&prepare_music_controller.0) {
        sink.stop();
    }

    commands.remove_resource::<PrepareMusicController>()
}
