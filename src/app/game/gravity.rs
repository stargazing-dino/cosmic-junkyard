use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule};

use crate::app::game::game_state_machine::GameState;

use super::DebugGizmos;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn GravitySource, PointGravity>()
            .register_component_as::<dyn GravitySource, PlanarGravity>()
            .add_systems(
                PhysicsSchedule,
                (update_gravity, keep_upright)
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .in_set(GravitySystemSet),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GravitySystemSet;

/// A component that indicates that an entity is affected by gravity.
#[derive(Component, Default)]
pub struct GravityBound {
    /// The sum of all forces due to gravity acting on this entity.
    pub gravity_force: Vec3,
}

#[derive(Bundle)]
pub struct GravitySourceBundle {
    // TODO:
    // pub gravity_source: Box<dyn GravitySource>,
    pub position: Position,

    pub rigid_body: RigidBody,

    pub collider: Collider,

    pub sensor: Sensor,
}

#[bevy_trait_query::queryable]
pub trait GravitySource {
    fn calculate_force(&self, position: Vec3, other_position: Vec3, mass: f32) -> Vec3;
}

#[derive(Component, Reflect, Default, Debug)]
pub struct PointGravity {
    pub gravity_strength: f32,

    pub center_mass: f32,
}

impl GravitySource for PointGravity {
    fn calculate_force(&self, position: Vec3, other_position: Vec3, mass: f32) -> Vec3 {
        // Compute distance between planet and body
        let distance = position.distance(other_position);

        // Prevent division by very small numbers
        let safe_distance = distance.max(0.001);

        // Compute gravitational force as per Newton's law
        let gravity_force_magnitude =
            self.gravity_strength * self.center_mass * mass / safe_distance.powi(2);
        let gravity_vector = (position - other_position).normalize() * gravity_force_magnitude;

        gravity_vector
    }
}

/// According to classical physics, the gravitational field of an
/// infinite plane is actually uniform. That is, it doesn't decrease
/// with distance! This is a result of the superposition of the
/// gravitational effects from all parts of the plane, near and far,
/// which add up to a constant at any given height above the plane.
/// The direction of the field is perpendicular to the plane,
/// towards it.
#[derive(Component, Reflect, Default, Debug)]
pub struct PlanarGravity {
    // The normal vector of the plane.
    pub normal: Vec3,

    pub gravity_strength: f32,
}

impl GravitySource for PlanarGravity {
    /// Fg = m * g
    fn calculate_force(&self, _position: Vec3, _other_position: Vec3, mass: f32) -> Vec3 {
        let gravity_vector = -self.normal.normalize() * self.gravity_strength * mass;

        gravity_vector
    }
}

#[derive(Component)]
pub struct CurvedGravity {
    // curve: Bezier
    cubic_generator: Box<dyn CubicGenerator<Vec3> + Send + Sync>,

    gravity_strength: f32,
}

impl GravitySource for CurvedGravity {
    fn calculate_force(&self, position: Vec3, _other_position: Vec3, mass: f32) -> Vec3 {
        const MAX_ITER: usize = 10;
        const EPSILON: f32 = 1e-6;
        let curve = self.cubic_generator.to_curve();

        // Newton's method to find the closest point on the curve.
        let mut t = 0.5; // starting with the middle of the curve
        for _ in 0..MAX_ITER {
            let pos = curve.position(t);
            let vel = curve.velocity(t);
            let acc = curve.acceleration(t);

            let diff = pos - position;
            let diff_norm = diff.length();
            if diff_norm < EPSILON {
                // We're close enough
                break;
            }

            let proj_vel = diff.dot(vel);
            let proj_acc = diff.dot(acc);

            let t_next = t - proj_vel / (proj_acc - proj_vel.powi(2) / diff_norm);

            // Make sure t stays within [0, 1]
            t = t_next.max(0.0).min(1.0);
        }

        let closest_point = curve.position(t);
        let direction = (closest_point - position).normalize();

        direction * self.gravity_strength * mass
    }
}

// This function gets all rigid bodies currently in a collision with a sensor. If that sensor is
// has a GravitySource component it then calculates the force due to that gravity source and
// applies it to the rigid body.
fn update_gravity(
    debug_gizmos: Res<DebugGizmos>,
    mut gizmos: Gizmos,
    mut rigid_body_query: Query<
        (
            RigidBodyQuery,
            &mut ExternalForce,
            &CollidingEntities,
            &mut GravityBound,
        ),
        Without<Sensor>,
    >,
    gravity_source_query: Query<(&dyn GravitySource, &Position), With<Sensor>>,
) {
    for (rb_item, mut external_force, colliding_entities, mut gravity_bound) in
        rigid_body_query.iter_mut()
    {
        if !rb_item.rb.is_dynamic() {
            continue;
        }

        for colliding_entity in colliding_entities.0.iter() {
            if let Ok((gravity_sources, position)) = gravity_source_query.get(*colliding_entity) {
                for gravity_source in gravity_sources {
                    let gravity_force = gravity_source.calculate_force(
                        position.0,
                        rb_item.position.0,
                        rb_item.mass.0,
                    );

                    external_force.apply_force(gravity_force);

                    if debug_gizmos.enabled {
                        gizmos.ray(position.0, position.0 - gravity_force, Color::BLUE);
                    }
                }
            }
        }

        gravity_bound.gravity_force = external_force.force();
    }
}

#[derive(Component)]
pub struct Upright;

const ROTATION_SPEED: f32 = 0.3; // Adjust this value for faster/slower rotation correction

const ROTATION_THRESHOLD: f32 = 60.0; // Angle in degrees. Adjust based on desired gameplay.

pub fn keep_upright(mut gravity_bound_query: Query<(&mut Rotation, &GravityBound), With<Upright>>) {
    for (mut rotation, gravity_bound) in &mut gravity_bound_query {
        let gravity_force = gravity_bound.gravity_force;
        if gravity_force == Vec3::ZERO {
            continue;
        }

        let gravity_up = -gravity_force.normalize();
        let player_up = rotation.0 * Vec3::Y;

        // Calculate the rotation from the player's up vector to the gravity up vector
        let rotation_angle = player_up.angle_between(gravity_up).to_degrees();

        // If the angle exceeds the threshold, don't try to correct
        if rotation_angle > ROTATION_THRESHOLD {
            continue;
        }

        let rotation_axis = player_up.cross(gravity_up);

        if rotation_axis != Vec3::ZERO {
            let rotation_axis = rotation_axis.normalize();
            let rotation_diff = Quat::from_axis_angle(rotation_axis, rotation_angle.to_radians());
            let target_rotation = rotation_diff * rotation.0;

            // Gradually rotate the player to align with the gravity direction
            rotation.0 = rotation.0.slerp(target_rotation, ROTATION_SPEED);
        }
    }
}
