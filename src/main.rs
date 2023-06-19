use assets::environment::PlanetCollection;
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    gltf::{Gltf, GltfMesh},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_rapier3d::prelude::*;
use input::InputPlugin;
use noisy_bevy::NoisyShaderPlugin;

use crate::assets::environment::PlanetType;

mod assets;
mod input;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        // Window resource
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fishy".to_string(), // ToDo
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                canvas: Some("#bevy".to_owned()),
                position: WindowPosition::At((0, 0).into()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(NoisyShaderPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // An outer space dark purple
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
        // .insert_resource(Bounds::default())
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, PlanetCollection>(GameState::AssetLoading)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .configure_sets(
            (SimulationSet::Input, SimulationSet::Logic)
                .chain()
                .in_base_set(CoreSet::Update),
        )
        .add_systems(
            (setup_graphics, setup_level_gen)
                .in_set(SimulationSet::Logic)
                .in_schedule(OnEnter(GameState::Playing)),
        )
        .add_systems(
            (
                update_gravity_system,
                // play_initial_animations,
                // update_bounds,
                // constrain_to_bounds,
                // update_player_animations,
            )
                .distributive_run_if(in_state(GameState::Playing))
                .in_set(SimulationSet::Logic),
        )
        .run();
}

// System sets can be used to group systems and configured to control relative ordering
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationSet {
    Input,
    Logic,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Playing,
    GameOver,
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
                scale: 2.0,
                scaling_mode: ScalingMode::FixedVertical(4.0),
                ..default()
            }),
            transform: camera_transform,
            ..default()
        },
        BloomSettings::default(),
    ));
}

#[derive(Component, Debug)]
pub struct Planet {
    pub planet_type: PlanetType,

    pub gravity_strength: f32,
}

fn setup_level_gen(
    mut commands: Commands,
    planet_collection: Res<PlanetCollection>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // We construct our own gravity so set the global rapier one to 0.0
    rapier_config.gravity = Vec3::ZERO;

    let planet_type = PlanetType::Planet1;
    let gltf_handle = planet_type.model_from(&planet_collection);
    let gltf = gltf_assets.get(&gltf_handle).unwrap();
    let scene = gltf.scenes.first().unwrap().clone();
    let gltf_mesh_handle = gltf.meshes.first().unwrap().clone();
    let gltf_mesh = gltf_meshes.get(&gltf_mesh_handle).unwrap();
    let mesh_handle = gltf_mesh.primitives.first().unwrap().mesh.clone();
    let mesh = meshes.get(&mesh_handle).unwrap();
    let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();

    commands.spawn((
        SceneBundle {
            scene,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        collider,
        Planet {
            planet_type,
            gravity_strength: 9.8,
        },
        RigidBody::KinematicPositionBased,
        AdditionalMassProperties::Mass(1000.0),
    ));

    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0.0, 5.0, 0.0)),
        Collider::from_bevy_mesh(
            &Mesh::from(shape::Cube { size: 5.0 }),
            &ComputedColliderShape::TriMesh,
        )
        .unwrap(),
        RigidBody::Dynamic,
        ExternalForce::default(),
        AdditionalMassProperties::Mass(10.0),
    ));
}

fn update_gravity_system(
    planet_query: Query<(&Planet, &Transform), With<RigidBody>>,
    mut body_query: Query<(&Transform, &mut ExternalForce), (With<RigidBody>, Without<Planet>)>,
) {
    for (planet, planet_transform) in planet_query.iter() {
        for (body_transform, mut external_force) in body_query.iter_mut() {
            // TODO: get true centroid of planet and body
            let direction_to_center = planet_transform.translation - body_transform.translation;
            let gravity_vector = direction_to_center.normalize() * planet.gravity_strength;
            external_force.force = gravity_vector;
        }
    }
}
