use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::app::game::game_state_machine::GameState;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn GravitySource, PointGravity>()
            .register_component_as::<dyn GravitySource, PlanarGravity>()
            .add_systems((update_gravity,).distributive_run_if(in_state(GameState::Playing)));
    }
}

#[bevy_trait_query::queryable]
pub trait GravitySource {
    fn calculate_force(&self, position: Vec3, mass: f32) -> Vec3;
}

#[derive(Component, Reflect, Default, Debug)]
pub struct PointGravity {
    pub center: Vec3,

    pub gravity_strength: f32,

    pub center_mass: f32,
}

impl GravitySource for PointGravity {
    fn calculate_force(&self, position: Vec3, mass: f32) -> Vec3 {
        // Compute distance between planet and body
        let distance = self.center.distance(position);

        // Prevent division by very small numbers
        // Replace 0.001 with a suitable small number
        let safe_distance = distance.max(0.001);

        // Compute gravitational force as per Newton's law
        let gravity_force_magnitude =
            self.gravity_strength * self.center_mass * mass / safe_distance.powi(2);
        let gravity_vector = (self.center - position).normalize() * gravity_force_magnitude;

        gravity_vector
    }
}

#[derive(Component, Reflect, Default, Debug)]
pub struct PlanarGravity {
    pub point: Vec3,

    pub normal: Vec3,
}

impl GravitySource for PlanarGravity {
    fn calculate_force(&self, position: Vec3, mass: f32) -> Vec3 {
        // Calculate the force due to a planar source here
        todo!()
    }
}

// TODO:
// pub struct CurvedGravity {
//     curve: QuadraticBezier3,
// }

// This function gets all rigid bodies currently in a collision with a sensor. If that sensor is
// has a GravitySource component, then it calculates the force due to that gravity source and
// applies it to the rigid body.
fn update_gravity(
    mut collider_query: Query<
        (RigidBodyQuery, &mut ExternalForce, &CollidingEntities),
        Without<Sleeping>,
    >,
    gravity_query: Query<(&dyn GravitySource, &Sensor)>,
) {
    for (body, mut external_force, colliding_entities) in collider_query.iter_mut() {
        let mut total_force = Vec3::ZERO;

        for colliding_entity in colliding_entities.0.iter() {
            if let Ok((gravity_sources, _)) = gravity_query.get(*colliding_entity) {
                for gravity_source in gravity_sources {
                    let force = gravity_source.calculate_force(body.position.0, body.mass.0);
                    println!("force: {:?}", force);
                    total_force += force;
                }
            }
        }

        external_force.0 = total_force;
    }
}
