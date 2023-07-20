use std::time::Duration;

use bevy::{audio::Volume, prelude::*};

use crate::assets::sounds::SoundCollection;

use super::{game_state_machine::GameState, junk::JunkCollisionEvent};

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoundCollisionTimer(Timer::new(
            Duration::from_secs_f32(1.0),
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            (junk_collisions,).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Resource)]
pub struct SoundCollisionTimer(Timer);

#[derive(Component)]
pub struct CollisionSound;

fn junk_collisions(
    mut commands: Commands,
    sound_collection: Res<SoundCollection>,
    collision_sink_query: Query<&AudioSink, With<CollisionSound>>,
    mut junk_collision_event_reader: EventReader<JunkCollisionEvent>,
    mut sound_collision_stopwatch: ResMut<SoundCollisionTimer>,
    time: Res<Time>,
) {
    sound_collision_stopwatch.0.tick(time.delta());

    if !sound_collision_stopwatch.0.finished() {
        return;
    }

    for _junk_collision in junk_collision_event_reader.iter() {
        let sound = sound_collection.fatal.clone();

        // if let Ok(sink) = collision_sink_query.get_single() {
        //     sink.play();
        // } else {
        //     commands.spawn(AudioBundle {
        //         source: sound,
        //         settings: PlaybackSettings::ONCE.with_volume(Volume::new_relative(0.1)),
        //         ..default()
        //     });
        // }

        sound_collision_stopwatch.0.reset();
    }
}
