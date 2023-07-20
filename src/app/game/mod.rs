use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::{
    math::PI,
    prelude::{
        CoefficientCombine, Collider, ColliderMassProperties, ExternalForce, Friction, Inertia,
        Mass, PhysicsLoop, PhysicsPlugins, Position, Restitution, RigidBody, Rotation, Sensor,
        ShapeCaster,
    },
    resources::Gravity,
    PhysicsSchedule, PhysicsStepSet,
};
use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;

use crate::{
    assets::environment::{PlanetCollection, PlanetType},
    utility::collider_from_gltf,
};

use self::{
    game_state_machine::{GameState, GameStateMachinePlugin},
    graphics::{GraphicsPlugin, MainCameraTarget},
    gravity::{GravityBound, GravityPlugin, GravitySourceBundle, GravitySystemSet, PointGravity},
    junk::JunkPlugin,
    player::{Player, PlayerPlugin, PlayerSystemSet},
    sounds::SoundsPlugin,
};

mod bounds;
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
        .insert_resource(Gravity::ZERO)
        .add_plugins((
            // BoundsPlugin,
            PhysicsPlugins::default(),
            JunkPlugin,
            GraphicsPlugin,
            GravityPlugin,
            PlayerPlugin,
            SoundsPlugin,
            GameStateMachinePlugin,
        ))
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
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planet_types = PlanetType::iter().collect::<Vec<_>>();
    let mut rng = thread_rng();

    let planet_type = planet_types.choose(&mut rng).unwrap();
    let gltf_handle = planet_type.model_from(&planet_collection);
    let (scene, collider) =
        collider_from_gltf(gltf_handle, &gltf_assets, &gltf_meshes, &mut meshes);
    let mass = 150.0;
    let position_vector = Vec3::new(0.0, 0.0, 0.0);

    commands
        .spawn((
            PlanetBundle {
                planet: Planet {
                    planet_type: *planet_type,
                    state: MovementState::Idle,
                },
                position: Position(position_vector),
                rigid_body: RigidBody::Kinematic,
                mass: Mass(mass as f32),
                friction: Friction::new(0.8).with_combine_rule(CoefficientCombine::Max),
                scene: SceneBundle { scene, ..default() },
                collider_mass_properties: ColliderMassProperties::ZERO,
                // TODO: How do you scale colliders?
                collider,
            },
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        ))
        .with_children(|parent| {
            let radius = 24.0;

            parent.spawn((
                PointGravity {
                    // TODO: This looks off bruh
                    center_mass: mass as f32,
                    gravity_strength: 4.8,
                },
                // PlanarGravity {
                //     gravity_strength: 9.8,
                //     normal: Vec3::X,
                // },
                GravitySourceBundle {
                    position: Position(position_vector),
                    rigid_body: RigidBody::Kinematic,
                    collider: Collider::ball(radius),
                    sensor: Sensor,
                },
            ));
        });

    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(shape.into()),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(14.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     Collider::ball(1.0),
    //     ColliderMassProperties::ZERO,
    //     Inertia(Mat3::IDENTITY),
    //     Mass(1.0),
    //     RigidBody::Dynamic,
    //     Position(Vec3::new(14.0, 0.0, 0.0)),
    //     ExternalForce::default(),
    //     Friction::new(6.0),
    //     Junk {},
    //     MainCameraTarget,
    // ));

    let shape = shape::Capsule {
        radius: 0.4,
        depth: 1.0,
        ..default()
    };

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape.into()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Position(Vec3::new(6.0, 0.0, 0.0)),
        Collider::capsule(1.0, 0.4),
        // Cast the player shape downwards to detect when the player is grounded
        ShapeCaster::new(
            Collider::capsule(0.9, 0.35),
            Vec3::X * 0.05,
            Quat::default(),
            Vec3::X,
        )
        .with_ignore_origin_penetration(true) // Don't count player's collider
        .with_max_time_of_impact(0.2)
        .with_max_hits(1),
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        ColliderMassProperties::ZERO,
        Inertia(Mat3::IDENTITY),
        Mass(1.0),
        ExternalForce::default().with_persistence(false),
        MainCameraTarget,
        Player,
        GravityBound::default(),
    ));
}

fn pause_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.pause();
}

fn resume_physics(mut physics_loop: ResMut<PhysicsLoop>) {
    physics_loop.resume();
}
