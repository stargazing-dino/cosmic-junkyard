use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::{
    prelude::{Friction, Mass, PhysicsPlugins, Position, RigidBody},
    resources::Gravity,
};
use leafwing_input_manager::{prelude::*, Actionlike};

use crate::assets::environment::{PlanetCollection, PlanetType};

use self::{playing::PlayingPlugin, prepare::PreparePlugin};

use super::{player_input::Player, GameState, TransitionEvent};

mod playing;
mod prepare;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins)
            .insert_resource(Gravity::ZERO)
            .add_plugin(PreparePlugin)
            .add_plugin(PlayingPlugin)
            .insert_resource(Bounds::default())
            .add_collection_to_loading_state::<_, PlanetCollection>(GameState::AssetLoading)
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1.0 / 5.0f32,
            })
            // TODO: This is where we'd use a stack likely
            .add_system(
                resume_play
                    .run_if(in_state(GameState::Playing).or_else(in_state(GameState::Prepare))),
            )
            .add_system(
                pause_play
                    .run_if(in_state(GameState::Playing).or_else(in_state(GameState::Prepare))),
            );
    }
}

/// The bounds of the playable area
#[derive(Resource, Default)]
pub struct Bounds {
    pub min: Vec2,

    pub max: Vec2,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Junk {}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Goal {}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Planet {
    pub planet_type: PlanetType,

    pub gravity_strength: f32,

    pub state: MovementState,
}

#[derive(Debug, Reflect, Default, Copy, Clone, PartialEq)]
pub enum MovementState {
    #[default]
    Idle,

    Moving {
        direction: Vec2,
    },
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,

    pub position: Position,

    pub rigid_body: RigidBody,

    pub mass: Mass,

    pub friction: Friction,
}

/// This updates the bounds based off the camera's position and projection.
/// If there is no camera, the bounds will be unchanged. We'd need to reconsider
/// this in headless mode.
fn update_bounds(
    window: Query<&Window, With<PrimaryWindow>>,
    camera_projection: Query<(&Transform, &Projection), With<Camera>>,
    mut bounds: ResMut<Bounds>,
) {
    let (camera_transform, projection) = camera_projection.single();
    let resolution = &window.single().resolution;
    let camera_transform = camera_transform.translation;
    let aspect_ratio = resolution.width() / resolution.height();
    let (horizontal_view, vertical_view) =
        if let Projection::Orthographic(orthographic) = projection {
            let scale = orthographic.scale;
            let (fixed_dim, other_dim) =
                if let ScalingMode::FixedVertical(vertical) = orthographic.scaling_mode {
                    ((vertical * aspect_ratio), vertical)
                } else if let ScalingMode::FixedHorizontal(horizontal) = orthographic.scaling_mode {
                    (horizontal, (horizontal / aspect_ratio))
                } else {
                    unimplemented!()
                };
            (fixed_dim * scale, other_dim * scale)
        } else {
            unimplemented!()
        };

    bounds.min = Vec2::new(
        camera_transform.x - horizontal_view / 2.0,
        camera_transform.y - vertical_view / 2.0,
    );
    bounds.max = Vec2::new(
        camera_transform.x + horizontal_view / 2.0,
        camera_transform.y + vertical_view / 2.0,
    );
}

#[derive(Actionlike, PartialEq, Clone, Copy, Debug)]
pub enum PlayerAction {
    Pause,

    Continue,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(KeyCode::Space, Continue);
        input_map.insert(KeyCode::Space, Pause);

        input_map
    }
}

fn resume_play(
    mut query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut transition_writer: EventWriter<TransitionEvent>,
) {
    for (action_state, _player) in query.iter_mut() {
        if action_state.just_pressed(PlayerAction::Continue) {
            transition_writer.send(TransitionEvent::UnpauseGame);
        }
    }
}

fn pause_play(
    mut query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut transition_writer: EventWriter<TransitionEvent>,
) {
    for (action_state, _player) in query.iter_mut() {
        if action_state.just_pressed(PlayerAction::Pause) {
            transition_writer.send(TransitionEvent::PauseGame);
        }
    }
}
