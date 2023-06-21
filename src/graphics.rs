use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::ScalingMode,
};

use crate::{GameState, SimulationSet};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            setup_graphics
                .in_set(SimulationSet::Logic)
                .in_schedule(OnEnter(GameState::Playing)),
        );
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
