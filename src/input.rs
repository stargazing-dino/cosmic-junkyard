use bevy::prelude::*;
use leafwing_input_manager::orientation::Direction;
use leafwing_input_manager::prelude::*;

use crate::{GameState, MovementState, Planet, SimulationSet};

// This plugin maps inputs to an input-type agnostic action-state
// We need to provide it with an enum which stores the possible actions a player could take
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlanetAction>::default())
            .add_event::<PlanetStateEvent>()
            .add_systems(
                (set_direction,)
                    .distributive_run_if(in_state(GameState::Playing))
                    .in_set(SimulationSet::Input),
            )
            .add_systems(
                (setup_player,)
                    .in_set(SimulationSet::Logic)
                    .in_schedule(OnEnter(GameState::Playing)),
            );
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((PlayerBundle {
        player: Player::default(),
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
    },));
}

/// Listening to this event you can manage properties of the planet based
/// off the player's inputs.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlanetStateEvent {
    pub movement_state: MovementState,

    pub size: f32,

    pub emitter: Entity,
}

// TODO: Use relations 🌈 or maybe generics for target
// I think having cursor related stuff in the player might be good?
#[derive(Component)]
pub struct Player {
    target: Option<Entity>,
}

impl Default for Player {
    fn default() -> Self {
        Self { target: None }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub input_manager: InputManagerBundle<PlanetAction>,
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<PlanetAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
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

#[derive(Actionlike, PartialEq, Clone, Copy, Debug)]
pub enum PlanetAction {
    Size(f32),

    Move(Direction),
}

fn set_direction(
    mut query: Query<(Entity, &ActionState<PlanetAction>, &Player)>,
    mut planet_state_events: EventWriter<PlanetStateEvent>,
    mut planets: Query<&mut Planet>,
) {
    for (emitter, action_state, player) in query.iter_mut() {
        // We need a valid target (planet) in order to move it
        let Some(target) =  player.target else {
            continue;
        };
        let Ok(mut planet) = planets.get_mut(target) else {
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
            planet_state_events.send(PlanetStateEvent {
                movement_state: next_state,
                size: 0.0,
                emitter,
            });
        }

        planet.state = next_state;
    }
}
