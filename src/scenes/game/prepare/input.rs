use bevy::prelude::{
    in_state, App, Entity, EventWriter, GamepadButtonType, KeyCode, Plugin, Query, Vec2, With,
};
use leafwing_input_manager::{orientation::Direction, prelude::*, Actionlike};

use crate::scenes::{game::Planet, player_input::Focus, GameState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlanetChangeEvent>()
            .add_plugin(InputManagerPlugin::<PlanetAction>::default())
            .add_systems((change_planets,).distributive_run_if(in_state(GameState::Preparation)))
            .add_system(change_planets);
    }
}

#[derive(Actionlike, PartialEq, Clone, Copy, Debug)]
pub enum PlanetAction {
    Size(f32),

    Move(Direction),
}

impl PlanetAction {
    pub fn default_input_map() -> InputMap<Self> {
        use PlanetAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(KeyCode::Up, Move(Direction::NORTH));
        input_map.insert(GamepadButtonType::DPadUp, Move(Direction::NORTH));

        input_map.insert(KeyCode::Down, Move(Direction::SOUTH));
        input_map.insert(GamepadButtonType::DPadDown, Move(Direction::SOUTH));

        input_map.insert(KeyCode::Left, Move(Direction::WEST));
        input_map.insert(GamepadButtonType::DPadLeft, Move(Direction::WEST));

        input_map.insert(KeyCode::Right, Move(Direction::EAST));
        input_map.insert(GamepadButtonType::DPadRight, Move(Direction::EAST));

        input_map
    }
}

fn change_planets(
    mut query: Query<(Entity, &ActionState<PlanetAction>, &Focus), With<Player>>,
    mut planet_state_events: EventWriter<PlanetChangeEvent>,
    mut planets: Query<&mut Planet>,
) {
    for (emitter, action_state, focus) in query.iter_mut() {
        // We need a valid target (planet) in order to move it
        let Focus(Some(target)) =  focus else {
            continue;
        };
        let Ok(mut planet) = planets.get_mut(*target) else {
            continue;
        };
        let mut intended_direction = Vec2::ZERO;

        [
            Direction::NORTH,
            Direction::SOUTH,
            Direction::EAST,
            Direction::WEST,
        ]
        .iter()
        .filter(|input_direction| action_state.pressed(PlanetAction::Move(**input_direction)))
        .for_each(|direction| intended_direction += direction.unit_vector());

        // Normalize the vector to prevent faster diagonal movement
        if intended_direction.length() > 1.0 {
            intended_direction = intended_direction.normalize();
        }

        let next_state = if intended_direction != Vec2::ZERO {
            MovementState::Moving {
                direction: intended_direction,
            }
        } else {
            MovementState::Idle
        };

        // TODO: I'll somehow have to match off the input size. Dunno
        // let intended_size = action_state
        //     .get(PlanetAction::Size)
        //     .map(|size| size * SIZE_UNIT)
        //     .unwrap_or(planet.size);

        if planet.state != next_state {
            planet_state_events.send(PlanetChangeEvent {
                movement_state: next_state,
                size: 0.0,
                emitter,
            });
        }

        planet.state = next_state;
    }
}
