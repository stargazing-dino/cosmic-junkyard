#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::prelude::AudioSource;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

#[derive(AssetCollection, Resource)]
pub struct UiSoundCollection {}
