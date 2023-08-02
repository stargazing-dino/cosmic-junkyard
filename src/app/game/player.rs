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

const PLAYER_SPEED: f32 = 2.0;

pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Transform, &mut LinearVelocity), With<Player>>,
) {
    for (transform, mut linear_velocity) in &mut players {
        // Get the player's  forward and right vectors
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

        if move_dir == Vec3::ZERO {
            continue;
        }

        // Normalize the movement vector and adjust it for speed and delta time
        move_dir = move_dir.normalize() * PLAYER_SPEED;

        // Apply the movement to the player
        linear_velocity.0 += move_dir;
    }
}

pub fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut ExternalForce, &ShapeHits, &GravityBound), With<Player>>,
    sensors_query: Query<&Sensor>,
) {
    for (mut external_force, ground_hits, gravity_bound) in &mut players {
        let gravity_force = gravity_bound.gravity_force;
        if gravity_force == Vec3::ZERO {
            continue;
        }

        let gravity_up = -gravity_force.normalize();

        // Jump if space pressed and the player is close enough to the ground
        if keyboard_input.just_pressed(KeyCode::Space) {
            println!("space pressed. Ground hits: {:?}", ground_hits.len());
            if !ground_hits.is_empty() {
                // If we're only hitting sensors, then we can't jump
                let can_jump = ground_hits
                    .iter()
                    .all(|hit| sensors_query.get(hit.entity).is_err());

                if can_jump {
                    external_force.apply_force(gravity_up * 200.0);
                }
            }
        }
    }
}
