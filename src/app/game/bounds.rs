use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

use crate::app::game::game_state_machine::GameState;

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Bounds::default()).add_systems(
            Update,
            (update_bounds,).run_if(in_state(GameState::Playing)),
        );
    }
}

/// The bounds of the playable area
#[derive(Resource, Default)]
pub struct Bounds {
    pub min: Vec2,

    pub max: Vec2,
}

/// This updates the bounds based off the camera's position and projection.
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
