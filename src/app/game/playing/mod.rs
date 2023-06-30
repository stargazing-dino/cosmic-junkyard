use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use self::sounds::SoundsPlugin;

use super::{game_state_machine::GameState, Junk, Planet};

mod input;
mod music;
mod sounds;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SoundsPlugin)
            .add_event::<JunkCollision>()
            .add_systems((resume_physics,).in_schedule(OnEnter(GameState::Paused)))
            .add_systems((pause_physics,).in_schedule(OnExit(GameState::Playing)))
            .add_systems(
                (
                    update_gravity,
                    check_collision,
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

fn update_gravity(
    planet_query: Query<(&Planet, &Position, &Mass), With<RigidBody>>,
    mut body_query: Query<
        (&Position, &Mass, &mut ExternalForce),
        (With<RigidBody>, Without<Planet>),
    >,
) {
    for (body_position, body_mass, mut external_force) in body_query.iter_mut() {
        // Initialize a new force vector to (0,0)
        let mut total_force = Vec3::ZERO;

        for (planet, planet_position, planet_mass) in planet_query.iter() {
            // Compute distance between planet and body
            let distance = planet_position.0.distance(body_position.0);

            // Prevent division by very small numbers
            let safe_distance = distance.max(0.001); // Replace 0.001 with a suitable small number

            // Compute gravitational force as per Newton's law
            let gravity_force_magnitude =
                planet.gravity_strength * planet_mass.0 * body_mass.0 / safe_distance.powi(2);
            let gravity_vector =
                (planet_position.0 - body_position.0).normalize() * gravity_force_magnitude;

            // Add this planet's gravitational force to the total
            total_force += gravity_vector;
        }

        // Apply the total gravitational force from all planets to the body
        // external_force.0 = total_force;
        external_force.0 = total_force;
    }
}

pub struct JunkCollision {
    pub normal: Vec3,

    pub penetration: f32,

    /// In global coordinates.
    pub contact_point: Vec3,
}

fn check_collision(
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
