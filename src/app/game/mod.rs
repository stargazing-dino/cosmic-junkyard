use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    gltf::{Gltf, GltfMesh},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::RaycastPickCamera;
use bevy_xpbd_3d::{
    prelude::{
        CoefficientCombine, Collider, ColliderMassProperties, ExternalForce, Friction, Inertia,
        Mass, PhysicsPlugins, Position, RigidBody,
    },
    resources::Gravity,
};
use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

use crate::{
    assets::environment::{PlanetCollection, PlanetType},
    utility::collider_from_gltf,
};

use self::{
    game_state_machine::{GameState, GameStateMachinePlugin},
    playing::PlayingPlugin,
    prepare::PreparePlugin,
};

mod game_state_machine;
mod playing;
mod prepare;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameStateMachinePlugin)
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Preparing),
            )
            .add_collection_to_loading_state::<_, PlanetCollection>(GameState::AssetLoading)
            .add_plugins(PhysicsPlugins)
            .insert_resource(Gravity::ZERO)
            .insert_resource(Bounds::default())
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1.0 / 5.0f32,
            })
            .add_plugin(PreparePlugin)
            .add_plugin(PlayingPlugin)
            .add_systems(
                (setup_graphics, setup_level_gen).in_schedule(OnEnter(GameState::Preparing)),
            );
        // .add_system(
        //     resume_play.run_if(
        //         in_state(AppState::InGameLevel).and_then(in_state(GameState::Preparing)),
        //     ),
        // );
        // .add_system(
        //     pause_play.run_if(in_state(AppState::InGameLevel).or_else(in_state(AppState::Prepare))),
        // );
    }
}

/// The bounds of the playable area
#[derive(Resource, Default)]
pub struct Bounds {
    pub min: Vec2,

    pub max: Vec2,
}

#[derive(Component, Default, Debug)]
pub struct GameCamera;

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

fn setup_graphics(mut commands: Commands) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 30.0, 0.01).looking_at(Vec3::ZERO, Vec3::Y),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 30.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            color: Color::hex("0a0a2c").unwrap(),
            intensity: 100000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(30.0, 200.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            color: Color::hex("ffddaa").unwrap(),
            intensity: 100000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        ..default()
    });

    let camera_transform = Transform::from_xyz(0.0, 0.0, 30.0);

    // Bevy is a right handed, Y-up system.
    commands.spawn((
        Camera3dBundle {
            tonemapping: Tonemapping::TonyMcMapface,
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 32.0,
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }),
            transform: camera_transform,
            ..default()
        },
        RaycastPickCamera::default(),
        BloomSettings::default(),
        GameCamera,
    ));
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
    let positions: [Vec2; 2] = [
        Vec2::new(0.0, 14.0),
        Vec2::new(0.0, -14.0),
        // Vec2::new(-20.0, 14.0),
    ];

    for position in positions.iter() {
        let planet_type = planet_types.choose(&mut rng).unwrap();
        let gltf_handle = planet_type.model_from(&planet_collection);
        let (scene, collider) =
            collider_from_gltf(gltf_handle, &gltf_assets, &gltf_meshes, &mut meshes);
        // let x = rng.gen_range(-20..=20);
        // let y = rng.gen_range(-14..=14);

        let mass = rng.gen_range(10..=200);

        commands.spawn((
            SceneBundle {
                scene,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            collider,
            ColliderMassProperties::ZERO,
            PlanetBundle {
                planet: Planet {
                    planet_type: *planet_type,
                    gravity_strength: 9.8,
                    state: MovementState::Idle,
                },
                position: Position(Vec3::new(position.x, position.y, 0.0)),
                rigid_body: RigidBody::Kinematic,
                mass: Mass(mass as f32),
                // mass: Mass(1.0),
                friction: Friction::new(100.0).with_combine_rule(CoefficientCombine::Multiply),
            },
        ));
    }

    let shape = shape::UVSphere {
        radius: 1.0,
        ..default()
    };

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape.into()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Collider::ball(1.0),
        ColliderMassProperties::ZERO,
        Inertia(Mat3::IDENTITY),
        Mass(1.0),
        RigidBody::Dynamic,
        Position(Vec3::new(14.0, 0.0, 0.0)),
        ExternalForce::default(),
        Friction::new(6.0),
        Junk {},
    ));
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
