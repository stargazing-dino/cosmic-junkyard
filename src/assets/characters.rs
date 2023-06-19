#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::{
    prelude::{Handle, Resource},
    scene::Scene,
};
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum AstronautType {
    BarbaraTheBee,
    FernandoTheFlamingo,
    FinnTheFrog,
    RaeTheRedPanda,
}

#[derive(AssetCollection, Resource)]
pub struct AstronautCollection {
    #[asset(path = "models/characters/Astronaut_BarbaraTheBee.gltf#Scene0")]
    pub barbara_the_bee: Handle<Scene>,

    #[asset(path = "models/characters/Astronaut_FernandoTheFlamingo.gltf#Scene0")]
    pub fernando_the_flamingo: Handle<Scene>,

    #[asset(path = "models/characters/Astronaut_FinnTheFrog.gltf#Scene0")]
    pub finn_the_frog: Handle<Scene>,

    #[asset(path = "models/characters/Astronaut_RaeTheRedPanda.gltf#Scene0")]
    pub rae_the_red_panda: Handle<Scene>,
}

impl AstronautType {
    /// Gets the corresponding coral model for the given coral type
    pub fn model_from(&self, collection: &AstronautCollection) -> Handle<Scene> {
        match self {
            AstronautType::BarbaraTheBee => collection.barbara_the_bee.clone(),
            AstronautType::FernandoTheFlamingo => collection.fernando_the_flamingo.clone(),
            AstronautType::FinnTheFrog => collection.finn_the_frog.clone(),
            AstronautType::RaeTheRedPanda => collection.rae_the_red_panda.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum EnemyType {
    ExtraSmall,
    Flying,
    Large,
    Small,
}

#[derive(AssetCollection, Resource)]
pub struct EnemyCollection {
    #[asset(path = "models/characters/Enemy_ExtraSmall.gltf#Scene0")]
    pub extra_small: Handle<Scene>,

    #[asset(path = "models/characters/Enemy_Flying.gltf#Scene0")]
    pub flying: Handle<Scene>,

    #[asset(path = "models/characters/Enemy_Large.gltf#Scene0")]
    pub large: Handle<Scene>,

    #[asset(path = "models/characters/Enemy_Small.gltf#Scene0")]
    pub small: Handle<Scene>,
}

impl EnemyType {
    /// Gets the corresponding coral model for the given coral type
    pub fn model_from(&self, collection: &EnemyCollection) -> Handle<Scene> {
        match self {
            EnemyType::ExtraSmall => collection.extra_small.clone(),
            EnemyType::Flying => collection.flying.clone(),
            EnemyType::Large => collection.large.clone(),
            EnemyType::Small => collection.small.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum MechType {
    BarbaraTheBee,
    FernandoTheFlamingo,
    FinnTheFrog,
    RaeTheRedPanda,
}

#[derive(AssetCollection, Resource)]
pub struct MechCollection {
    #[asset(path = "models/characters/Mech_BarbaraTheBee.gltf#Scene0")]
    pub barbara_the_bee: Handle<Scene>,

    #[asset(path = "models/characters/Mech_FernandoTheFlamingo.gltf#Scene0")]
    pub fernando_the_flamingo: Handle<Scene>,

    #[asset(path = "models/characters/Mech_FinnTheFrog.gltf#Scene0")]
    pub finn_the_frog: Handle<Scene>,

    #[asset(path = "models/characters/Mech_RaeTheRedPanda.gltf#Scene0")]
    pub rae_the_red_panda: Handle<Scene>,
}

impl MechType {
    /// Gets the corresponding coral model for the given coral type
    pub fn model_from(&self, collection: &MechCollection) -> Handle<Scene> {
        match self {
            MechType::BarbaraTheBee => collection.barbara_the_bee.clone(),
            MechType::FernandoTheFlamingo => collection.fernando_the_flamingo.clone(),
            MechType::FinnTheFrog => collection.finn_the_frog.clone(),
            MechType::RaeTheRedPanda => collection.rae_the_red_panda.clone(),
        }
    }
}
