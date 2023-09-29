use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule};

use super::{game_state_machine::GameState, gravity::GravityBound, player::Player};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            ((movement,).before(apply_friction), apply_friction)
                .run_if(in_state(GameState::Playing))
                .in_set(MovementSystemSet),
        )
        // I'd like for jump to alongside movement, friction etc but because it uses
        // just_pressed which relies on timing, a FixedUpdate schedule will miss a
        // lot of the presses.
        .add_systems(
            Update,
            jump.run_if(in_state(GameState::Playing))
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
    mut gizmos: Gizmos,
    mut players: Query<
        (
            &Transform,
            &mut Rotation,
            &mut LinearVelocity,
            &ShapeHits,
            &GravityBound,
        ),
        With<Player>,
    >,
    sensors_query: Query<&Sensor>,
) {
    for (transform, mut rotation, mut linear_velocity, shape_hits, gravity_bound) in &mut players {
        let gravity_force = gravity_bound.gravity_force;
        let gravity_up = -gravity_force.normalize();
        let touching_ground = shape_hits
            .iter()
            .any(|hit| sensors_query.get(hit.entity).is_err());
        let forward = transform.forward();
        let right = transform.right();
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

            // Check the angle between move_dir and forward direction
            let dot_product = move_dir.dot(forward);
            let angle = dot_product.acos();

            // If the angle is not close to π radians, update the rotation. This prevents
            // the player from rotating when moving directly backwards
            if (angle - std::f32::consts::PI).abs() > 0.1 {
                let target_position = transform.translation + move_dir;

                gizmos.ray(transform.translation, move_dir, Color::YELLOW);

                let target_rotation = transform.looking_at(target_position, gravity_up).rotation;
                let new_rotation = transform.rotation.slerp(target_rotation, 0.1);
                rotation.0 = new_rotation;
            }

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

const JUMP_STRENGTH: f32 = 16.0;

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

        let touching_ground = shape_hits
            .iter()
            .any(|hit| sensors_query.get(hit.entity).is_err());

        if !touching_ground {
            continue;
        }

        let gravity_up = -gravity_force.normalize();

        if keyboard_input.just_pressed(KeyCode::Space) {
            external_impulse.apply_impulse(gravity_up * JUMP_STRENGTH);
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

        if !touching_ground {
            continue;
        }

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
