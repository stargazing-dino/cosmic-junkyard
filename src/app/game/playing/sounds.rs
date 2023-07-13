use std::time::Duration;

use bevy::{audio::Volume, prelude::*, time::Stopwatch};

use crate::{app::AppState, assets::sounds::SoundCollection};

use super::JunkCollisionEvent;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoundCollisionStopWatch(Stopwatch::new()))
            .add_systems(
                Update,
                (junk_collisions,).run_if(in_state(AppState::InGameLevel)),
            );
    }
}

#[derive(Resource)]
pub struct SoundCollisionStopWatch(Stopwatch);

#[derive(Component)]
pub struct CollisionSound;

fn junk_collisions(
    mut commands: Commands,
    sound_collection: Res<SoundCollection>,
    collision_sink_query: Query<&AudioSink, With<CollisionSound>>,
    mut junk_collision_event_reader: EventReader<JunkCollisionEvent>,
    mut sound_collision_stopwatch: ResMut<SoundCollisionStopWatch>,
    time: Res<Time>,
) {
    sound_collision_stopwatch.0.tick(time.delta());

    if sound_collision_stopwatch.0.elapsed() < Duration::from_secs_f32(1.0) {
        return;
    }

    for _junk_collision in junk_collision_event_reader.iter() {
        let sound = sound_collection.fatal.clone();

        if let Ok(sink) = collision_sink_query.get_single() {
            sink.play();
        } else {
            commands.spawn(AudioBundle {
                source: sound,
                settings: PlaybackSettings::ONCE.with_volume(Volume::new_relative(0.5)),
                ..default()
            });
        }

        sound_collision_stopwatch.0.reset();
    }
}
