use bevy::prelude::*;
use leafwing_input_manager::orientation::Direction;
use leafwing_input_manager::prelude::*;

use crate::{GameState, SimulationSet};

// This plugin maps inputs to an input-type agnostic action-state
// We need to provide it with an enum which stores the possible actions a player could take
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MovementAction>::default())
            .add_event::<PlayerStateEvent>()
            .add_systems(
                (set_direction, move_towards)
                    .distributive_run_if(in_state(GameState::Playing))
                    .in_set(SimulationSet::Input),
            );
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerStateEvent {
    pub state: MovementState,
    pub entity: Entity,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovementState {
    Idle,
    Moving { direction: Vec2 },
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    state: MovementState,
    lerp_factor: f32,
    acceleration: f32,
    max_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 0.0,
            state: MovementState::Idle,
            lerp_factor: 0.1,
            acceleration: 6.0,
            max_speed: 1.6,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub input_manager: InputManagerBundle<MovementAction>,
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<MovementAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use MovementAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        input_map
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MovementAction {
    Up,
    Down,
    Left,
    Right,
}

impl MovementAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        MovementAction::Up,
        MovementAction::Down,
        MovementAction::Left,
        MovementAction::Right,
    ];

    fn direction(self) -> Option<Direction> {
        match self {
            MovementAction::Up => Some(Direction::NORTH),
            MovementAction::Down => Some(Direction::SOUTH),
            MovementAction::Left => Some(Direction::WEST),
            MovementAction::Right => Some(Direction::EAST),
        }
    }
}

fn set_direction(
    mut query: Query<(Entity, &ActionState<MovementAction>, &mut Player)>,
    mut player_state_events: EventWriter<PlayerStateEvent>,
) {
    for (entity, action_state, mut player) in query.iter_mut() {
        let mut intended_direction = Vec2::ZERO;

        MovementAction::DIRECTIONS
            .iter()
            .filter(|input_direction| action_state.pressed(**input_direction))
            .filter_map(|input_direction| input_direction.direction())
            .for_each(|direction| intended_direction += Vec2::from(direction));

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

        if player.state != next_state {
            player_state_events.send(PlayerStateEvent {
                state: next_state,
                entity,
            });
        }

        player.state = next_state;
    }
}

fn move_towards(mut query: Query<(&mut Transform, &mut Player)>, time: Res<Time>) {
    let delta_seconds = time.delta_seconds();

    for (mut transform, mut player) in query.iter_mut() {
        if let MovementState::Idle = player.state {
            player.speed = (player.speed - player.acceleration * 3.0 * delta_seconds).max(0.0);
        } else if let MovementState::Moving { direction } = player.state {
            player.speed =
                (player.speed + player.acceleration * delta_seconds).min(player.max_speed);

            let target_translation = transform.translation.lerp(
                transform.translation + (direction * player.speed).extend(0.0),
                player.lerp_factor,
            );

            transform.translation = target_translation;

            // If the direction is vertical just continue
            if (direction.x - 0.0).abs() < f32::EPSILON {
                continue;
            }

            let target_rotation = transform.rotation.lerp(
                Quat::from_rotation_y(direction.angle_between(-Vec2::Y)),
                player.lerp_factor,
            );

            transform.rotation = target_rotation;
        }
    }
}
