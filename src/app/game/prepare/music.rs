use bevy::{audio::Volume, prelude::*};

use crate::{app::game::game_state_machine::GameState, assets::music::MusicCollection};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preparing), setup)
            .add_systems(OnExit(GameState::Preparing), teardown);
    }
}

#[derive(Component)]
struct PrepareMusic;

fn setup(
    mut commands: Commands,
    music_collection: Res<MusicCollection>,
    // audio: Res<Audio>,
    // audio_sinks: Res<Assets<AudioSink>>,
) {
    let weak_handle = music_collection.goodbye_sweet_alien.clone();

    commands.spawn((
        AudioBundle {
            source: weak_handle,
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.5)),
        },
        PrepareMusic,
    ));
}

fn teardown(
    mut commands: Commands,
    audio_sink_query: Query<(Entity, &AudioSink), With<PrepareMusic>>,
) {
    if let Ok((entity, sink)) = audio_sink_query.get_single() {
        sink.stop();
        commands.entity(entity).despawn_recursive();
    }
}
