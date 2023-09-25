use bevy::{
    asset::LoadState,
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, Skybox},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

use super::game_state_machine::GameState;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_systems(
            Update,
            (follow_target, track_to_target, asset_loaded).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::Playing), (setup_graphics,));
    }
}

/// The main rig tag
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
pub struct MainTrackTarget;

#[derive(Component)]
pub struct MainFollowTarget;

fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let camera_transform = Transform::from_xyz(0.0, 0.0, 16.0);
    let skybox_handle = asset_server.load("skyboxes/cubemap.png");

    // Bevy is a right handed, Y-up system.
    commands.spawn((
        Camera3dBundle {
            tonemapping: Tonemapping::TonyMcMapface,
            // projection: Projection::Orthographic(OrthographicProjection {
            //     scale: 10.0,
            //     scaling_mode: ScalingMode::FixedVertical(1.0),
            //     ..default()
            // }),
            transform: camera_transform,
            ..default()
        },
        BloomSettings::default(),
        MainCamera,
        Skybox(skybox_handle.clone()),
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded
        && asset_server.get_load_state(cubemap.image_handle.clone_weak()) == LoadState::Loaded
    {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

/// Moves the camera to follow the target
fn follow_target(
    target_query: Query<&Transform, (With<MainFollowTarget>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let Ok(target_transform) = target_query.get_single() else { return };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };

    // how high the camera is above the player
    let up_offset = 2.0;
    // how far the camera is behind the player
    let back_offset = 5.0;
    // Compute the desired camera position relative to the player
    let target_camera_position = target_transform.translation
        + target_transform.local_y() * up_offset
        - target_transform.local_z() * back_offset;

    let smooth_factor = 10.0 * time.delta_seconds();
    camera_transform.translation = camera_transform
        .translation
        .lerp(target_camera_position, smooth_factor);

    // Make the camera look at the player
    camera_transform.look_at(target_transform.translation, target_transform.up());
}

/// Tracks the target without moving the camera
fn track_to_target(
    target_query: Query<&Transform, (With<MainTrackTarget>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(target_transform) = target_query.get_single() else { return };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };

    // Make the camera look at the player
    camera_transform.look_at(target_transform.translation, target_transform.up());
}
