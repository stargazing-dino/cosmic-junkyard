// fn update_gravity(
//     planet_query: Query<(&Planet, &Position, &Mass), With<RigidBody>>,
//     mut body_query: Query<
//         (&Position, &Mass, &mut ExternalForce),
//         (With<RigidBody>, Without<Planet>),
//     >,
// ) {
//     for (body_position, body_mass, mut external_force) in body_query.iter_mut() {
//         // Initialize a new force vector to (0,0)
//         let mut total_force = Vec3::ZERO;

//         for (planet, planet_position, planet_mass) in planet_query.iter() {
//             // Compute distance between planet and body
//             let distance = planet_position.0.distance(body_position.0);

//             // Prevent division by very small numbers
//             let safe_distance = distance.max(0.001); // Replace 0.001 with a suitable small number

//             // Compute gravitational force as per Newton's law
//             let gravity_force_magnitude =
//                 planet.gravity_strength * planet_mass.0 * body_mass.0 / safe_distance.powi(2);
//             let gravity_vector =
//                 (planet_position.0 - body_position.0).normalize() * gravity_force_magnitude;

//             // Add this planet's gravitational force to the total
//             total_force += gravity_vector;
//         }

//         // Apply the total gravitational force from all planets to the body
//         // external_force.0 = total_force;
//         external_force.0 = total_force;
//     }
// }
