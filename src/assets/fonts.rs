use bevy::asset::AssetServer;
use bevy::prelude::{Handle, Resource};
use bevy::text::Font;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct FontCollection {
    #[asset(path = "fonts/Comfortaa-Bold.ttf")]
    pub comfortaa_bold: Handle<Font>,

    #[asset(path = "fonts/Comfortaa-Light.ttf")]
    pub comfortaa_light: Handle<Font>,

    #[asset(path = "fonts/Comfortaa-Medium.ttf")]
    pub comfortaa_medium: Handle<Font>,

    #[asset(path = "fonts/Comfortaa-Regular.ttf")]
    pub comfortaa_regular: Handle<Font>,

    #[asset(path = "fonts/Comfortaa-SemiBold.ttf")]
    pub comfortaa_semibold: Handle<Font>,
}
