use bevy::prelude::*;

use crate::assets::music::MusicCollection;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app;
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
