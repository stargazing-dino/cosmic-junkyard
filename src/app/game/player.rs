use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule};

use super::{game_state_machine::GameState, gravity::GravityBound};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            (movement, jump)
                .run_if(in_state(GameState::Playing))
                .in_set(PlayerSystemSet),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSystemSet;

const PLAYER_SPEED: f32 = 6.4;
const AIR_CONTROL_FACTOR: f32 = 0.2;
const FRICTION_FACTOR: f32 = 0.4;

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &ShapeHits,
            &GravityBound,
        ),
        With<Player>,
    >,
    sensors_query: Query<&Sensor>,
) {
    for (transform, mut linear_velocity, mut angularVelocity, shape_hits, gravity_bound) in
        &mut players
    {
        let gravity_force = gravity_bound.gravity_force;

        // If the character is floating in space, don't apply movement
        if gravity_force == Vec3::ZERO {
            continue;
        }

        let gravity_up = -gravity_force.normalize();
        let touching_ground = shape_hits
            .iter()
            .any(|hit| sensors_query.get(hit.entity).is_err());

        let forward = -transform.forward();
        let right = -transform.right();

        // Create a movement vector from the keyboard input
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Up) {
            move_dir += forward;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            move_dir -= forward;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            move_dir -= right;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            move_dir += right;
        }

        if move_dir != Vec3::ZERO {
            // Normalize the movement vector and adjust it for speed and delta time
            move_dir = move_dir.normalize() * PLAYER_SPEED;

            // Apply the movement to the player
            if touching_ground {
                linear_velocity.0 += move_dir;
            } else {
                // apply smaller movement when in air
                linear_velocity.0 += move_dir * AIR_CONTROL_FACTOR;
            }
        }

        // We need to apply friction to the player when they're on the ground
        if touching_ground {
            // project the velocity onto the ground plane and apply friction to it
            let ground_velocity = linear_velocity.0.dot(gravity_up) * gravity_up;
            let mut tangential_velocity = linear_velocity.0 - ground_velocity;

            // apply friction to the tangential velocity
            tangential_velocity *= 1.0 - FRICTION_FACTOR;

            // combine the velocities
            linear_velocity.0 = ground_velocity + tangential_velocity;
        }
    }
}

pub fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut ExternalForce, &ShapeHits, &GravityBound), With<Player>>,
    sensors_query: Query<&Sensor>,
) {
    for (mut external_force, shape_hits, gravity_bound) in &mut players {
        let gravity_force = gravity_bound.gravity_force;
        if gravity_force == Vec3::ZERO {
            continue;
        }

        let gravity_up = -gravity_force.normalize();

        if keyboard_input.just_pressed(KeyCode::Space) {
            let touching_ground = shape_hits
                .iter()
                .any(|hit| sensors_query.get(hit.entity).is_err());

            if touching_ground {
                external_force.apply_force(gravity_up * 200.0);
            }
        }
    }
}
