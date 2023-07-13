use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use self::sounds::SoundsPlugin;

use super::{game_state_machine::GameState, gravity::GravityPlugin, Junk, Planet};

mod sounds;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SoundsPlugin, GravityPlugin))
            .add_event::<JunkCollisionEvent>()
            .add_systems(OnEnter(GameState::Paused), resume_physics)
            .add_systems(OnExit(GameState::Playing), pause_physics)
            .add_systems(
                Update,
                (
                    junk_collisions,
                    // constrain_to_bounds,
                    // update_player_animations,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn pause_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.pause();
}

fn resume_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.resume();
}

#[derive(Event)]
pub struct JunkCollisionEvent {
    pub normal: Vec3,

    pub penetration: f32,

    /// In global coordinates.
    pub contact_point: Vec3,
}

fn junk_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut junk_collision_write: EventWriter<JunkCollisionEvent>,
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
            junk_collision_write.send(JunkCollisionEvent {
                normal: contact.normal,
                penetration: contact.penetration,
                contact_point,
            });
        }
    }
}
