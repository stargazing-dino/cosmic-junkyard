use bevy::asset::AssetServer;
use bevy::prelude::Image;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct BackgroundCollection {
    #[asset(path = "backgrounds/Blue_Nebula_01-1024x1024.png")]
    pub blue_nebula_1: Handle<Image>,

    #[asset(path = "backgrounds/Blue_Nebula_02-1024x1024.png")]
    pub blue_nebula_2: Handle<Image>,

    #[asset(path = "backgrounds/Blue_Nebula_03-1024x1024.png")]
    pub blue_nebula_3: Handle<Image>,

    #[asset(path = "backgrounds/Blue_Nebula_04-1024x1024.png")]
    pub blue_nebula_4: Handle<Image>,

    #[asset(path = "backgrounds/Blue_Nebula_05-1024x1024.png")]
    pub blue_nebula_5: Handle<Image>,

    #[asset(path = "backgrounds/Blue_Nebula_06-1024x1024.png")]
    pub blue_nebula_6: Handle<Image>,

    #[asset(path = "backgrounds/Blue_Nebula_07-1024x1024.png")]
    pub blue_nebula_7: Handle<Image>,

    #[asset(path = "backgrounds/Blue_Nebula_08-1024x1024.png")]
    pub blue_nebula_8: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_01-1024x1024.png")]
    pub green_nebula_1: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_02-1024x1024.png")]
    pub green_nebula_2: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_03-1024x1024.png")]
    pub green_nebula_3: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_04-1024x1024.png")]
    pub green_nebula_4: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_05-1024x1024.png")]
    pub green_nebula_5: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_06-1024x1024.png")]
    pub green_nebula_6: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_07-1024x1024.png")]
    pub green_nebula_7: Handle<Image>,

    #[asset(path = "backgrounds/Green_Nebula_08-1024x1024.png")]
    pub green_nebula_8: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_01-1024x1024.png")]
    pub purple_nebula_1: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_02-1024x1024.png")]
    pub purple_nebula_2: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_03-1024x1024.png")]
    pub purple_nebula_3: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_04-1024x1024.png")]
    pub purple_nebula_4: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_05-1024x1024.png")]
    pub purple_nebula_5: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_06-1024x1024.png")]
    pub purple_nebula_6: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_07-1024x1024.png")]
    pub purple_nebula_7: Handle<Image>,

    #[asset(path = "backgrounds/Purple_Nebula_08-1024x1024.png")]
    pub purple_nebula_8: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_01-1024x1024.png")]
    pub starfield_1: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_02-1024x1024.png")]
    pub starfield_2: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_03-1024x1024.png")]
    pub starfield_3: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_04-1024x1024.png")]
    pub starfield_4: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_05-1024x1024.png")]
    pub starfield_5: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_06-1024x1024.png")]
    pub starfield_6: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_07-1024x1024.png")]
    pub starfield_7: Handle<Image>,

    #[asset(path = "backgrounds/Starfield_08-1024x1024.png")]
    pub starfield_8: Handle<Image>,
}
