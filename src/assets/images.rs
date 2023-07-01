use bevy::asset::AssetServer;
use bevy::prelude::Image;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageCollection {
    #[asset(path = "images/main_menu_aim.png")]
    pub main_menu_aim: Handle<Image>,

    #[asset(path = "images/target.png")]
    pub target: Handle<Image>,
}
