use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    gltf::{Gltf, GltfMesh},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_xpbd_3d::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

use crate::{
    assets::environment::{PlanetCollection, PlanetType},
    scenes::GameState,
};

use super::{Junk, MovementState, Planet, PlanetBundle};

mod graphics;
mod input;
mod music;

pub struct PreparePlugin;

impl Plugin for PreparePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup_level_gen,).in_schedule(OnEnter(GameState::Preparation)));
    }
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
        BloomSettings::default(),
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
        let (scene, collider) = create_collider(
            *planet_type,
            &planet_collection,
            &gltf_assets,
            &gltf_meshes,
            &mut meshes,
        );
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
