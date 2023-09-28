use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule};

use super::{game_state_machine::GameState, gravity::GravityBound, player::Player};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            ((movement, jump).before(apply_friction), apply_friction)
                .run_if(in_state(GameState::Playing))
                .in_set(MovementSystemSet),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

const PLAYER_SPEED: f32 = 6.4;
const AIR_CONTROL_FACTOR: f32 = 0.2;

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    // debug_gizmos: Res<DebugGizmos>,
    // mut gizmos: Gizmos,
    mut players: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &ShapeHits,
            &GravityBound,
        ),
        With<Player>,
    >,
    sensors_query: Query<&Sensor>,
) {
    for (mut transform, mut linear_velocity, shape_hits, gravity_bound) in &mut players {
        let gravity_force = gravity_bound.gravity_force;
        let gravity_up = -gravity_force.normalize();
        let touching_ground = shape_hits
            .iter()
            .any(|hit| sensors_query.get(hit.entity).is_err());
        let forward = -transform.forward();
        let right = -transform.right();

        // gizmos.ray(
        //     transform.translation,
        //     transform.translation + right,
        //     Color::YELLOW,
        // );

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
            move_dir = move_dir.normalize();

            // Normalize the movement vector and adjust it for speed and delta time
            let target_position = transform.translation + move_dir;

            // if debug_gizmos.enabled {
            //     gizmos.ray(transform.translation, target_position, Color::RED);
            // }

            let new_rotation = transform.rotation.lerp(
                transform.looking_at(target_position, gravity_up).rotation,
                0.1,
            );

            transform.rotation = new_rotation;

            move_dir = move_dir * PLAYER_SPEED;

            // If the character is floating in space, don't apply movement
            if gravity_force == Vec3::ZERO {
                continue;
            }

            // Apply the movement to the player
            if touching_ground {
                linear_velocity.0 += move_dir;
            } else {
                // apply smaller movement when in air
                linear_velocity.0 += move_dir * AIR_CONTROL_FACTOR;
            }
        }
    }
}

pub fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut ExternalImpulse, &ShapeHits, &GravityBound), With<Player>>,
    sensors_query: Query<&Sensor>,
) {
    for (mut external_impulse, shape_hits, gravity_bound) in &mut players {
        let gravity_force = gravity_bound.gravity_force;

        // If the player is floating in space, don't apply jump
        if gravity_force == Vec3::ZERO {
            continue;
        }

        let gravity_up = -gravity_force.normalize();

        if keyboard_input.just_pressed(KeyCode::Space) {
            let touching_ground = shape_hits
                .iter()
                .any(|hit| sensors_query.get(hit.entity).is_err());

            if touching_ground {
                external_impulse.apply_impulse(gravity_up * 16.0);
            }
        }
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FrictionSystemSet;

const FRICTION_FACTOR: f32 = 0.4;

pub fn apply_friction(
    mut players: Query<(&mut LinearVelocity, &ShapeHits, &GravityBound), With<Player>>,
    sensors_query: Query<&Sensor>,
) {
    // We need to apply friction to the player when they're on the ground
    for (mut linear_velocity, shape_hits, gravity_bound) in &mut players {
        let touching_ground = shape_hits
            .iter()
            .any(|hit| sensors_query.get(hit.entity).is_err());

        if touching_ground {
            let gravity_force = gravity_bound.gravity_force;
            let gravity_up = -gravity_force.normalize();
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
