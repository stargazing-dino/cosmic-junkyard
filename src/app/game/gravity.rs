use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::app::game::game_state_machine::GameState;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn GravitySource, PointGravity>()
            .register_component_as::<dyn GravitySource, PlanarGravity>()
            .add_systems(
                Update,
                (update_gravity,).run_if(in_state(GameState::Playing)),
            );
    }
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
        // Replace 0.001 with a suitable small number
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
    pub normal: Vec3,

    pub gravity_strength: f32,
}

impl GravitySource for PlanarGravity {
    /// Fg = m * g
    fn calculate_force(&self, _position: Vec3, _other_position: Vec3, mass: f32) -> Vec3 {
        let gravity_vector = self.normal.normalize() * self.gravity_strength * mass;

        gravity_vector
    }
}

// TODO: We'll have to hold off until bevy 0.11
// pub struct CurvedGravity {
//     curve: QuadraticBezier3,
// }

// This function gets all rigid bodies currently in a collision with a sensor. If that sensor is
// has a GravitySource component, then it calculates the force due to that gravity source and
// applies it to the rigid body.
fn update_gravity(
    mut rigid_body_query: Query<
        (RigidBodyQuery, &mut ExternalForce, &CollidingEntities),
        Without<Sensor>,
    >,
    gravity_source_query: Query<(&dyn GravitySource, &Position), With<Sensor>>,
) {
    for (rb_item, mut external_force, colliding_entities) in rigid_body_query.iter_mut() {
        if !rb_item.rb.is_dynamic() {
            continue;
        }

        let mut total_force = Vec3::ZERO;

        for colliding_entity in colliding_entities.0.iter() {
            if let Ok((gravity_sources, position)) = gravity_source_query.get(*colliding_entity) {
                for gravity_source in gravity_sources {
                    let force = gravity_source.calculate_force(
                        position.0,
                        rb_item.position.0,
                        rb_item.mass.0,
                    );

                    total_force += force;
                }
            }
        }

        external_force.set_force(total_force);
    }
}
