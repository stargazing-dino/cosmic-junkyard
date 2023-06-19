#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::{
    prelude::{Handle, Resource},
    scene::Scene,
};
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ItemType {
    Bullets,
    Crate,
    Health,
    Jar,
    KeyCarrd,
    Sphere,
    Thunder,
}

#[derive(AssetCollection, Resource)]
pub struct ItemCollection {
    #[asset(path = "models/items/Pickup_Bullets.gltf#Scene0")]
    pub bullets: Handle<Scene>,

    #[asset(path = "models/items/Pickup_Crate.gltf#Scene0")]
    pub crate_: Handle<Scene>,

    #[asset(path = "models/items/Pickup_Health.gltf#Scene0")]
    pub health: Handle<Scene>,

    #[asset(path = "models/items/Pickup_Jar.gltf#Scene0")]
    pub jar: Handle<Scene>,

    #[asset(path = "models/items/Pickup_KeyCard.gltf#Scene0")]
    pub key_card: Handle<Scene>,

    #[asset(path = "models/items/Pickup_Sphere.gltf#Scene0")]
    pub sphere: Handle<Scene>,

    #[asset(path = "models/items/Pickup_Thunder.gltf#Scene0")]
    pub thunder: Handle<Scene>,
}

impl ItemType {
    pub fn model_from(&self, collection: &ItemCollection) -> Handle<Scene> {
        match self {
            ItemType::Bullets => collection.bullets.clone(),
            ItemType::Crate => collection.crate_.clone(),
            ItemType::Health => collection.health.clone(),
            ItemType::Jar => collection.jar.clone(),
            ItemType::KeyCarrd => collection.key_card.clone(),
            ItemType::Sphere => collection.sphere.clone(),
            ItemType::Thunder => collection.thunder.clone(),
        }
    }
}
