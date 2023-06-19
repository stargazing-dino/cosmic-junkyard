#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::{
    prelude::{Handle, Resource},
    scene::Scene,
};
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum VehicleType {
    Rover1,
    Rover2,
    RoverRound,
    SpacehipBarbaraTheBee,
    SpacehipFernandoTheFlamingo,
    SpacehipFinnTheFrog,
    SpacehipRaeTheRedPanda,
}

#[derive(AssetCollection, Resource)]
pub struct VehicleCollection {
    #[asset(path = "models/vehicles/Rover_1.gltf#Scene0")]
    pub rover_1: Handle<Scene>,

    #[asset(path = "models/vehicles/Rover_2.gltf#Scene0")]
    pub rover_2: Handle<Scene>,

    #[asset(path = "models/vehicles/Rover_Round.gltf#Scene0")]
    pub rover_round: Handle<Scene>,

    #[asset(path = "models/vehicles/Spaceship_BarbaraTheBee.gltf#Scene0")]
    pub spaceship_barbara_the_bee: Handle<Scene>,

    #[asset(path = "models/vehicles/Spaceship_FernandoTheFlamingo.gltf#Scene0")]
    pub spaceship_fernando_the_flamingo: Handle<Scene>,

    #[asset(path = "models/vehicles/Spaceship_FinnTheFrog.gltf#Scene0")]
    pub spaceship_finn_the_frog: Handle<Scene>,

    #[asset(path = "models/vehicles/Spaceship_RaeTheRedPanda.gltf#Scene0")]
    pub spaceship_rae_the_red_panda: Handle<Scene>,
}

impl VehicleType {
    pub fn model_from(&self, collection: &VehicleCollection) -> Handle<Scene> {
        match self {
            VehicleType::Rover1 => collection.rover_1.clone(),
            VehicleType::Rover2 => collection.rover_2.clone(),
            VehicleType::RoverRound => collection.rover_round.clone(),
            VehicleType::SpacehipBarbaraTheBee => collection.spaceship_barbara_the_bee.clone(),
            VehicleType::SpacehipFernandoTheFlamingo => {
                collection.spaceship_fernando_the_flamingo.clone()
            }
            VehicleType::SpacehipFinnTheFrog => collection.spaceship_finn_the_frog.clone(),
            VehicleType::SpacehipRaeTheRedPanda => collection.spaceship_rae_the_red_panda.clone(),
        }
    }
}
