#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::prelude::AudioSource;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiSoundCollection {
    #[asset(path = "ui_sounds/Bleep_01.ogg")]
    pub bleep_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Bleep_02.ogg")]
    pub bleep_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Bleep_03.ogg")]
    pub bleep_03: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Bleep_04.ogg")]
    pub bleep_04: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Bleep_05.ogg")]
    pub bleep_05: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Bleep_06.ogg")]
    pub bleep_06: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Bleep_07.ogg")]
    pub bleep_07: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Click_01.ogg")]
    pub click_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Click_02.ogg")]
    pub click_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Click_03.ogg")]
    pub click_03: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Click_04.ogg")]
    pub click_04: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Complete_01.ogg")]
    pub complete_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Complete_02.ogg")]
    pub complete_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Confirm_01.ogg")]
    pub confirm_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Confirm_02.ogg")]
    pub confirm_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Confirm_03.ogg")]
    pub confirm_03: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Confirm_04.ogg")]
    pub confirm_04: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Confirm_05.ogg")]
    pub confirm_05: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Confirm_06.ogg")]
    pub confirm_06: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Confirm_07.ogg")]
    pub confirm_07: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Data_Point_01.ogg")]
    pub data_point_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Data_Point_02.ogg")]
    pub data_point_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Data_Point_04.ogg")]
    pub data_point_04: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Data_Point_05.ogg")]
    pub data_point_05: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Denied_01.ogg")]
    pub denied_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Denied_02.ogg")]
    pub denied_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Denied_03.ogg")]
    pub denied_03: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Execute_01.ogg")]
    pub execute_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Execute_02.ogg")]
    pub execute_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Sequence_01.ogg")]
    pub sequence_01: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Sequence_02.ogg")]
    pub sequence_02: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Sequence_03.ogg")]
    pub sequence_03: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Sequence_04.ogg")]
    pub sequence_04: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Sequence_05.ogg")]
    pub sequence_05: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Sequence_06.ogg")]
    pub sequence_06: Handle<AudioSource>,

    #[asset(path = "ui_sounds/Sequence_07.ogg")]
    pub sequence_07: Handle<AudioSource>,
}
