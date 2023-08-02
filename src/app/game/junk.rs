use bevy::prelude::*;
use bevy_xpbd_3d::prelude::Collision;

use super::{game_state_machine::GameState, Planet};

pub struct JunkPlugin;

impl Plugin for JunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JunkCollisionEvent>().add_systems(
            Update,
            (junk_collisions,).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Event)]
pub struct JunkCollisionEvent {
    pub normal: Vec3,

    pub penetration: f32,

    /// In global coordinates.
    pub contact_point: Vec3,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Junk {}

fn junk_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut junk_collision_write: EventWriter<JunkCollisionEvent>,
    planet_query: Query<Entity, With<Planet>>,
    junk_query: Query<Entity, With<Junk>>,
) {
    for Collision(contact) in collision_event_reader.iter() {
        let is_planet_entity1 = planet_query.get(contact.entity1).is_ok();
        let is_junk_entity2 = junk_query.get(contact.entity2).is_ok();

        let is_junk_entity1 = junk_query.get(contact.entity1).is_ok();
        let is_planet_entity2 = planet_query.get(contact.entity2).is_ok();

        for manifold in &contact.manifolds {
            for contact in &manifold.contacts {
                let contact_point = (contact.point1 + contact.point2) / 2.0;

                if (is_planet_entity1 && is_junk_entity2) || (is_junk_entity1 && is_planet_entity2)
                {
                    junk_collision_write.send(JunkCollisionEvent {
                        normal: contact.normal,
                        penetration: contact.penetration,
                        contact_point,
                    });
                }
            }
        }
    }
}
