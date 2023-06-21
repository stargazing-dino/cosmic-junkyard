#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::prelude::AudioSource;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum MusicType {
    IntoTheSpaceship,
    SatelliteInterruption,
    GoodbyeSweetAlien,
}

#[derive(AssetCollection, Resource)]
pub struct MusicCollection {
    #[asset(path = "music/1. Into The Spaceship.wav")]
    pub into_the_spaceship: Handle<AudioSource>,

    #[asset(path = "music/2. Satellite Interruption.wav")]
    pub satellite_interruption: Handle<AudioSource>,

    #[asset(path = "music/3. Goodbye Sweet Alien.wav")]
    pub goodbye_sweet_alien: Handle<AudioSource>,
}
