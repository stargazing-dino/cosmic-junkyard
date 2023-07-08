use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use self::sounds::SoundsPlugin;

use super::{game_state_machine::GameState, gravity::GravityPlugin, Junk, Planet};

mod sounds;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SoundsPlugin)
            .add_plugin(GravityPlugin)
            .add_event::<JunkCollision>()
            .add_systems((resume_physics,).in_schedule(OnEnter(GameState::Paused)))
            .add_systems((pause_physics,).in_schedule(OnExit(GameState::Playing)))
            .add_systems(
                (
                    junk_collisions,
                    // constrain_to_bounds,
                    // update_player_animations,
                )
                    .distributive_run_if(in_state(GameState::Playing)),
            );
    }
}

fn pause_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.pause();
}

fn resume_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.resume();
}

pub struct JunkCollision {
    pub normal: Vec3,

    pub penetration: f32,

    /// In global coordinates.
    pub contact_point: Vec3,
}

fn junk_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut junk_collision_write: EventWriter<JunkCollision>,
    planet_query: Query<Entity, With<Planet>>,
    junk_query: Query<Entity, With<Junk>>,
) {
    for Collision(contact) in collision_event_reader.iter() {
        let is_planet_entity1 = planet_query.get(contact.entity1).is_ok();
        let is_junk_entity2 = junk_query.get(contact.entity2).is_ok();

        let is_junk_entity1 = junk_query.get(contact.entity1).is_ok();
        let is_planet_entity2 = planet_query.get(contact.entity2).is_ok();

        let contact_point = (contact.point1 + contact.point2) / 2.0;

        if (is_planet_entity1 && is_junk_entity2) || (is_junk_entity1 && is_planet_entity2) {
            junk_collision_write.send(JunkCollision {
                normal: contact.normal,
                penetration: contact.penetration,
                contact_point,
            });
        }
    }
}
