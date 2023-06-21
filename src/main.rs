use assets::environment::PlanetCollection;
use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_xpbd_3d::prelude::*;
use graphics::GraphicsPlugin;
use input::InputPlugin;
use noisy_bevy::NoisyShaderPlugin;
use rand::{seq::SliceRandom, thread_rng};
use saving::SavingPlugin;
use strum::IntoEnumIterator;

use crate::assets::environment::PlanetType;

mod assets;
mod graphics;
mod input;
mod saving;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cosmic Junkyard".to_string(), // ToDo
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        canvas: Some("#bevy".to_owned()),
                        position: WindowPosition::At((0, 0).into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    // This tells the AssetServer to watch for changes to assets.
                    // It enables our scenes to automatically reload in game when we modify their files.
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .insert_resource(Gravity::ZERO)
        .insert_resource(Bounds::default())
        // An outer space dark purple
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Prepare),
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
            (setup_level_gen,)
                .in_set(SimulationSet::Logic)
                .in_schedule(OnEnter(GameState::Prepare)),
        )
        .add_systems(
            (
                update_gravity,
                // play_initial_animations,
                // constrain_to_bounds,
                // update_player_animations,
            )
                .distributive_run_if(in_state(GameState::Playing))
                .in_set(SimulationSet::Logic),
        )
        .add_plugin(NoisyShaderPlugin)
        .add_plugin(InputPlugin)
        // .add_plugin(SavingPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugins(PhysicsPlugins)
        .run();
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Junk {}

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

/// The bounds of the playable area
#[derive(Resource, Default)]
pub struct Bounds {
    pub min: Vec2,

    pub max: Vec2,
}

// System sets can be used to group systems and configured to
// control relative ordering. For example, we always want our input
// to occur before our movement logic so we're consistent.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationSet {
    Input,
    Logic,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,

    /// The player is sizing the planets, moving them around, etc.
    Prepare,

    /// The simulation (gravity and so on) is running
    Playing,

    Paused,

    GameOver,
}

fn create_collider(
    planet_type: PlanetType,
    planet_collection: &Res<PlanetCollection>,
    gltf_assets: &Res<Assets<Gltf>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    meshes: &ResMut<Assets<Mesh>>,
) -> (Handle<Scene>, Collider) {
    let gltf_handle = planet_type.model_from(&planet_collection);
    let gltf = gltf_assets.get(&gltf_handle).unwrap();
    let scene = gltf.scenes.first().unwrap().clone();
    let gltf_mesh_handle = gltf.meshes.first().unwrap().clone();
    let gltf_mesh = gltf_meshes.get(&gltf_mesh_handle).unwrap();
    let mesh_handle = gltf_mesh.primitives.first().unwrap().mesh.clone();
    let mesh = meshes.get(&mesh_handle).unwrap();

    (scene, Collider::trimesh_from_bevy_mesh(mesh).unwrap())
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
        let (scene, collider) = create_collider(
            *planet_type,
            &planet_collection,
            &gltf_assets,
            &gltf_meshes,
            &mut meshes,
        );
        // let x = rng.gen_range(-20..=20);
        // let y = rng.gen_range(-14..=14);
        // MassPropertiesBundle

        commands.spawn((
            SceneBundle {
                scene,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            collider,
            PlanetBundle {
                planet: Planet {
                    planet_type: *planet_type,
                    gravity_strength: 9.8,
                    state: MovementState::Idle,
                },
                position: Position(Vector::new(position.x, position.y, 0.0)),
                rigid_body: RigidBody::Kinematic,
                mass: Mass(100.0),
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
        RigidBody::Dynamic,
        Position(Vector::new(14.0, 0.0, 0.0)),
        ExternalForce::default(),
        Mass(1.0),
        Friction::new(6.0),
        Junk {},
    ));
}

fn update_gravity(
    planet_query: Query<(&Planet, &Position, &Mass), With<RigidBody>>,
    mut body_query: Query<
        (&Position, &Mass, &mut ExternalForce),
        (With<RigidBody>, Without<Planet>),
    >,
) {
    for (body_position, body_mass, mut external_force) in body_query.iter_mut() {
        // Initialize a new force vector to (0,0)
        let mut total_force = Vec3::ZERO;

        for (planet, planet_position, planet_mass) in planet_query.iter() {
            // Compute distance between planet and body
            let distance = planet_position.0.distance(body_position.0);

            // Prevent division by very small numbers
            let safe_distance = distance.max(0.001); // Replace 0.001 with a suitable small number

            // Compute gravitational force as per Newton's law
            let gravity_force_magnitude =
                planet.gravity_strength * planet_mass.0 * body_mass.0 / safe_distance.powi(2);
            let gravity_vector =
                (planet_position.0 - body_position.0).normalize() * gravity_force_magnitude;

            // Add this planet's gravitational force to the total
            total_force += gravity_vector;
        }

        // Apply the total gravitational force from all planets to the body
        external_force.0 = total_force;
    }
}
