// TODO: Replace this with https://github.com/nicopap/ui-navigation
use bevy::prelude::*;
use bevy::reflect::TypePath;
use leafwing_input_manager::orientation::Direction;
use leafwing_input_manager::prelude::*;

use super::AppState;

// This plugin maps inputs to an input-type agnostic action-state
// We need to provide it with an enum which stores the possible actions a player could take
pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<UiAction>::default())
            .add_systems(OnEnter(AppState::InGameLevel), (setup_player,));
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player::default(),
        focus: Focus(None),
        ui_action_manager: InputManagerBundle {
            input_map: UiAction::default_input_map(),
            ..default()
        },
    });
}

pub enum PlayerNumber {
    One,
    Two,
}

#[derive(Component)]
pub struct Focus(pub Option<Entity>);

// I think having cursor related stuff in the player might be good?
#[derive(Component)]
pub struct Player {
    pub player_number: PlayerNumber,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            player_number: PlayerNumber::One,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    pub focus: Focus,

    pub ui_action_manager: InputManagerBundle<UiAction>,
}

#[derive(Actionlike, PartialEq, Clone, Copy, Debug, TypePath)]
pub enum UiAction {
    Move(Direction),

    Select,
}

impl UiAction {
    pub fn default_input_map() -> InputMap<Self> {
        use UiAction::*;
        let mut input_map = InputMap::default();

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
