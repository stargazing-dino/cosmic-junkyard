use std::ops::AddAssign;

use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule};

use super::{game_state_machine::GameState, gravity::GravityBound};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            movement
                .run_if(in_state(GameState::Playing))
                .in_set(PlayerSystemSet),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSystemSet;

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<
        (
            &mut Rotation,
            &mut LinearVelocity,
            &mut ExternalForce,
            &ShapeHits,
            &GravityBound,
        ),
        With<Player>,
    >,
) {
    for (mut rotation, mut linear_velocity, mut external_force, ground_hits, gravity_bound) in
        &mut players
    {
        let gravity_force = gravity_bound.gravity_force;
        if gravity_force == Vec3::ZERO {
            continue;
        }

        let gravity_up: Vec3 = -gravity_force.normalize();
        let player_up: Vec3 = rotation.0 * Vec3::Y;

        // calculate the rotation from the player's up vector to the gravity up vector
        let rotation_axis = player_up.cross(gravity_up);

        // The cross product of two vectors that are collinear (in the same line,
        // which also includes exactly opposed) is a zero vector. Normalizing a zero
        // vector results in a NaN vector because normalization is dividing each
        // component by the vector's magnitude (which is zero for a zero vector),
        // thus dividing by zero.
        if rotation_axis != Vec3::ZERO {
            let rotation_axis = rotation_axis.normalize();
            let rotation_angle = player_up.angle_between(gravity_up);
            let rotation_diff = Quat::from_axis_angle(rotation_axis, rotation_angle);

            let target_rotation = rotation_diff * rotation.0;

            // Rotate the player to align with the gravity direction
            rotation.0 = target_rotation;
        }

        // // Jump if space pressed and the player is close enough to the ground
        // if keyboard_input.just_pressed(KeyCode::Space) && !ground_hits.is_empty() {
        //     // You gotta add a force against gravity to jump
        //     external_force.add_assign(gravity_up * 3.0);
        //     println!("Jump!");
        // }

        // Get the player's forward and right vectors
        let forward = rotation.0.mul_vec3(Vec3::Z).normalize();
        let right = rotation.0.mul_vec3(Vec3::X).normalize();

        // Create a movement vector from the keyboard input
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            move_dir += forward;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            move_dir -= forward;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            move_dir -= right;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            move_dir += right;
        }

        // Normalize the movement vector and adjust it for speed and delta time
        if move_dir == Vec3::ZERO {
            continue;
        }

        move_dir = move_dir.normalize() * 1.2; // adjust speed as necessary

        linear_velocity.0 += move_dir;
    }
}
