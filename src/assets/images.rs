use bevy::asset::AssetServer;
use bevy::prelude::Image;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageCollection {}
