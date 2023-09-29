use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::{
    prelude::{
        AngularDamping, CoefficientCombine, Collider, ColliderMassProperties, ExternalForce,
        Friction, Inertia, Mass, PhysicsDebugConfig, PhysicsLoop, PhysicsPlugins, Position,
        Restitution, RigidBody, Sensor, ShapeCaster, SpatialQueryFilter,
    },
    resources::Gravity,
    PhysicsSchedule, PhysicsStepSet,
};

use crate::{
    app::game::graphics::MainFollowTarget,
    assets::{
        characters::AstronautCollection,
        environment::{PlanetCollection, PlanetType},
    },
    utility::collider_from_gltf,
};

use self::{
    game_state_machine::{GameState, GameStateMachinePlugin},
    graphics::GraphicsPlugin,
    gravity::{
        GravityBound, GravityPlugin, GravitySourceBundle, GravitySystemSet, PointGravity, Upright,
    },
    junk::JunkPlugin,
    movement::{FrictionSystemSet, MovementPlugin, MovementSystemSet},
    player::{Player, PlayerPlugin},
    sounds::SoundsPlugin,
};

mod game_state_machine;
mod graphics;
mod gravity;
mod junk;
mod movement;
mod player;
mod sounds;

pub struct GamePlugin;

#[derive(Resource)]
pub struct DebugGizmos {
    pub enabled: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, PlanetCollection>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, AstronautCollection>(GameState::AssetLoading)
        .insert_resource(Gravity::ZERO)
        .insert_resource(DebugGizmos { enabled: true })
        .insert_resource(PhysicsDebugConfig {
            // enabled: false,
            enabled: true,
            ..Default::default()
        })
        .add_plugins((
            PhysicsPlugins::default(),
            JunkPlugin,
            GraphicsPlugin,
            GravityPlugin,
            PlayerPlugin,
            MovementPlugin,
            SoundsPlugin,
            GameStateMachinePlugin,
        ))
        .add_systems(OnEnter(GameState::Paused), pause_physics)
        .add_systems(
            OnEnter(GameState::Playing),
            (setup_level_gen, resume_physics),
        )
        .configure_sets(
            PhysicsSchedule,
            (MovementSystemSet, GravitySystemSet, FrictionSystemSet)
                .chain()
                // I'd preferably like this to run before PhysicsStep::Prepare
                .before(PhysicsStepSet::BroadPhase),
        );
    }
}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Planet {
    pub planet_type: PlanetType,

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

    pub scene: SceneBundle,

    pub collider: Collider,

    pub collider_mass_properties: ColliderMassProperties,
}

fn setup_level_gen(
    mut commands: Commands,
    planet_collection: Res<PlanetCollection>,
    astronaut_collection: Res<AstronautCollection>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let planet_type = PlanetType::Planet1;
    let gltf_handle = planet_type.model_from(&planet_collection);
    let (scene, collider) =
        collider_from_gltf(gltf_handle, &gltf_assets, &gltf_meshes, &mut meshes);
    let mass = 150.0;
    let planet_position = Vec3::new(0.0, 0.0, 0.0);

    commands
        .spawn((
            PlanetBundle {
                planet: Planet {
                    planet_type,
                    state: MovementState::Idle,
                },
                position: Position(planet_position),
                rigid_body: RigidBody::Kinematic,
                mass: Mass(mass as f32),
                friction: Friction::new(0.4).with_static_coefficient(0.8),
                scene: SceneBundle { scene, ..default() },
                collider_mass_properties: ColliderMassProperties::ZERO,
                // TODO: How do you scale colliders?
                collider,
            },
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Max),
        ))
        .with_children(|parent| {
            let radius = 24.0;

            parent.spawn((
                PointGravity {
                    // TODO: This looks off bruh
                    center_mass: mass as f32,
                    gravity_strength: 6.8,
                },
                GravitySourceBundle {
                    position: Position(planet_position),
                    rigid_body: RigidBody::Kinematic,
                    collider: Collider::ball(radius),
                    sensor: Sensor,
                },
            ));
        });

    let astronaut = astronaut_collection.fernando_the_flamingo.clone();
    let collider = Collider::ball(0.3);
    let player_position = Vec3::new(0.0, 10.0, 0.0);
    // let direction_to_center = (player_position - planet_position).normalize();
    // let rotation_axis = Vec3::Y.cross(direction_to_center).normalize();
    // let rotation_angle = direction_to_center.angle_between(Vec3::Y);
    // let rotation_quat = Quat::from_axis_angle(rotation_axis, rotation_angle);

    let mut player_commands = commands.spawn_empty();
    let player_id = player_commands.id();

    player_commands
        .insert((
            // Adding this next line breaks for some reason :(
            // Rotation(rotation_quat),
            SpatialBundle::default(),
            RigidBody::Dynamic,
            Position(player_position),
            collider.clone(),
            // Cast the player shape downwards to detect when the player is grounded
            ShapeCaster::new(
                Collider::capsule(0.9, 0.35),
                Vec3::ZERO,
                // Vec3::Y * 0.05,
                Quat::default(),
                -Vec3::Y,
            )
            .with_ignore_origin_penetration(true) // Don't count player's collider
            .with_max_hits(3)
            .with_query_filter(SpatialQueryFilter::new().without_entities([player_id]))
            .with_max_time_of_impact(0.2),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            ColliderMassProperties::ZERO,
            Inertia(Mat3::IDENTITY),
            Friction::new(0.6),
            Mass(1.0),
            (
                ExternalForce::default().with_persistence(false),
                MainFollowTarget,
                Player,
                GravityBound::default(),
                Upright,
                // TODO: Not sure if we should use Linear damping or Angular
                // damping here because we have funky axes and stuff.
                AngularDamping(1.6),
            ),
        ))
        .with_children(|parent| {
            let mut transform = Transform::from_xyz(0.0, -0.35, 0.0).with_scale(Vec3::splat(0.3));
            transform.rotate_y(std::f32::consts::PI);

            parent.spawn(SceneBundle {
                scene: astronaut,
                transform,
                ..default()
            });
        });
}

fn pause_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.pause();
}

fn resume_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.resume();
}
