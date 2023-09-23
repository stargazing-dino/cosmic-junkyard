use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::{
    prelude::{
        AngularDamping, CoefficientCombine, Collider, ColliderMassProperties, ExternalForce,
        Friction, Inertia, Mass, PhysicsDebugConfig, PhysicsLoop, PhysicsPlugins, Position,
        Restitution, RigidBody, Rotation, Sensor, ShapeCaster,
    },
    resources::Gravity,
    PhysicsSchedule, PhysicsStepSet,
};

use crate::{
    assets::{
        characters::AstronautCollection,
        environment::{PlanetCollection, PlanetType},
    },
    utility::collider_from_gltf,
};

use self::{
    game_state_machine::{GameState, GameStateMachinePlugin},
    graphics::{GraphicsPlugin, MainCameraTarget},
    gravity::{
        GravityBound, GravityPlugin, GravitySourceBundle, GravitySystemSet, PointGravity, Upright,
    },
    junk::JunkPlugin,
    player::{Player, PlayerPlugin, PlayerSystemSet},
    sounds::SoundsPlugin,
};

mod game_state_machine;
mod graphics;
mod gravity;
mod junk;
mod player;
mod sounds;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, PlanetCollection>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, AstronautCollection>(GameState::AssetLoading)
        .insert_resource(Gravity::ZERO)
        .add_plugins((
            PhysicsPlugins::default(),
            JunkPlugin,
            GraphicsPlugin,
            GravityPlugin,
            PlayerPlugin,
            SoundsPlugin,
            GameStateMachinePlugin,
        ))
        .insert_resource(PhysicsDebugConfig {
            // enabled: false,
            enabled: true,
            ..Default::default()
        })
        .configure_sets(
            PhysicsSchedule,
            (GravitySystemSet, PlayerSystemSet)
                .chain()
                .before(PhysicsStepSet::BroadPhase),
        )
        .add_systems(OnEnter(GameState::Paused), pause_physics)
        .add_systems(
            OnEnter(GameState::Playing),
            (setup_level_gen, resume_physics),
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
    let player_position = Vec3::new(10.0, 0.0, 0.0);
    let direction_to_center = (player_position - planet_position).normalize();
    let rotation_axis = Vec3::Y.cross(direction_to_center).normalize();
    let rotation_angle = direction_to_center.angle_between(Vec3::Y);
    let rotation_quat = Quat::from_axis_angle(rotation_axis, rotation_angle);

    commands
        .spawn((
            SpatialBundle::default(),
            RigidBody::Dynamic,
            Position(player_position),
            Rotation(rotation_quat),
            collider.clone(),
            // Cast the player shape downwards to detect when the player is grounded
            ShapeCaster::new(collider, -Vec3::Y * 0.05, Quat::default(), -Vec3::Y)
                .with_ignore_origin_penetration(true) // Don't count player's collider
                .with_max_time_of_impact(0.02)
                // The user can be in a lot of gravity fields and those are all colliders
                .with_max_hits(3),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            ColliderMassProperties::ZERO,
            Inertia(Mat3::IDENTITY),
            Friction::new(0.6),
            Mass(1.0),
            (
                ExternalForce::default().with_persistence(false),
                MainCameraTarget,
                Player,
                GravityBound::default(),
                Upright,
                // TODO: Not sure if we should use Linear damping or Angular
                // damping here because we have funky axes and stuff.
                AngularDamping(1.6),
            ),
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: astronaut,
                transform: Transform::from_xyz(0.0, -0.35, 0.0).with_scale(Vec3::splat(0.3)),
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
