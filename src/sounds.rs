use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_asset_loader::prelude::LoadingStateAppExt;

use crate::{assets::sounds::SoundCollection, GameState, JunkCollision};

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, SoundCollection>(GameState::AssetLoading)
            .insert_resource(SoundCollisionStopWatch(Stopwatch::new()))
            .add_systems((junk_collisions,).distributive_run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
pub struct SoundCollisionStopWatch(Stopwatch);

fn junk_collisions(
    sound_collection: Res<SoundCollection>,
    audio: Res<Audio>,
    mut junk_collision_event_reader: EventReader<JunkCollision>,
    mut sound_collision_stopwatch: ResMut<SoundCollisionStopWatch>,
    time: Res<Time>,
) {
    sound_collision_stopwatch.0.tick(time.delta());

    if sound_collision_stopwatch.0.elapsed() < Duration::from_secs_f32(1.0) {
        return;
    }

    for junk_collision in junk_collision_event_reader.iter() {
        let sound = sound_collection.fatal.clone();

        audio.play_spatial_with_settings(
            sound,
            PlaybackSettings::ONCE.with_volume(0.75),
            Transform::IDENTITY,
            1.0,
            junk_collision.contact_point,
        );

        sound_collision_stopwatch.0.reset();
    }
}
