use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Bundle)]
pub struct ControllerBundle {
    pub controller: Controller,
}

#[derive(Component, Debug)]
pub struct Controller {
    pub run_speed: f32,

    pub pitch_sensitivity: f32,

    pub yaw_sensitivity: f32,

    pub jump_force: f32,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            run_speed: 6.4,
            pitch_sensitivity: PI / 180.0,
            yaw_sensitivity: PI / 180.0,
            jump_force: 10.0,
        }
    }
}
