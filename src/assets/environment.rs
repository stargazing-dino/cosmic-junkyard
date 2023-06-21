#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::gltf::Gltf;
use bevy::prelude::{Handle, Resource};
use bevy::reflect::Reflect;
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum BuildingType {
    BaseLarge,
    BuildingL,
    GeodesicDome,
    HouseCylinder,
    HouseLong,
    HouseOpen,
}

#[derive(AssetCollection, Resource)]
pub struct BuildingCollection {
    #[asset(path = "models/environment/Base_Large.gltf")]
    pub base_large: Handle<Gltf>,

    #[asset(path = "models/environment/Building_L.gltf")]
    pub building_l: Handle<Gltf>,

    #[asset(path = "models/environment/GeodesicDome.gltf")]
    pub geodesic_dome: Handle<Gltf>,

    #[asset(path = "models/environment/House_Cylinder.gltf")]
    pub house_cylinder: Handle<Gltf>,

    #[asset(path = "models/environment/House_Long.gltf")]
    pub house_long: Handle<Gltf>,

    #[asset(path = "models/environment/House_Open.gltf")]
    pub house_open: Handle<Gltf>,
}

impl BuildingType {
    pub fn model_from(&self, collection: &BuildingCollection) -> Handle<Gltf> {
        match self {
            BuildingType::BaseLarge => collection.base_large.clone(),
            BuildingType::BuildingL => collection.building_l.clone(),
            BuildingType::GeodesicDome => collection.geodesic_dome.clone(),
            BuildingType::HouseCylinder => collection.house_cylinder.clone(),
            BuildingType::HouseLong => collection.house_long.clone(),
            BuildingType::HouseOpen => collection.house_open.clone(),
        }
    }
}

pub enum StructureType {
    Connector,
    HouseSingleSupport,
    HouseSingle,
    HouseOpenBack,
    MetalSupport,
    Ramp,
    RoofAntenna,
    RoofOpening,
    RoofRadar,
    RoofVentL,
    RoofVentR,
    SolarPanelGround,
    SolarPanelRoof,
    SolarPanelStructure,
    Stairs,
}

#[derive(AssetCollection, Resource)]
pub struct StructureCollection {
    #[asset(path = "models/environment/Connector.gltf")]
    pub connector: Handle<Gltf>,

    #[asset(path = "models/environment/House_Single_Support.gltf")]
    pub house_single_support: Handle<Gltf>,

    #[asset(path = "models/environment/House_Single.gltf")]
    pub house_single: Handle<Gltf>,

    #[asset(path = "models/environment/House_OpenBack.gltf")]
    pub house_open_back: Handle<Gltf>,

    #[asset(path = "models/environment/MetalSupport.gltf")]
    pub metal_support: Handle<Gltf>,

    #[asset(path = "models/environment/Ramp.gltf")]
    pub ramp: Handle<Gltf>,

    #[asset(path = "models/environment/Roof_Antenna.gltf")]
    pub roof_antenna: Handle<Gltf>,

    #[asset(path = "models/environment/Roof_Opening.gltf")]
    pub roof_opening: Handle<Gltf>,

    #[asset(path = "models/environment/Roof_Radar.gltf")]
    pub roof_radar: Handle<Gltf>,

    #[asset(path = "models/environment/Roof_VentL.gltf")]
    pub roof_vent_l: Handle<Gltf>,

    #[asset(path = "models/environment/Roof_VentR.gltf")]
    pub roof_vent_r: Handle<Gltf>,

    #[asset(path = "models/environment/SolarPanel_Ground.gltf")]
    pub solar_panel_ground: Handle<Gltf>,

    #[asset(path = "models/environment/SolarPanel_Roof.gltf")]
    pub solar_panel_roof: Handle<Gltf>,

    #[asset(path = "models/environment/SolarPanel_Structure.gltf")]
    pub solar_panel_structure: Handle<Gltf>,

    #[asset(path = "models/environment/Stairs.gltf")]
    pub stairs: Handle<Gltf>,
}

impl StructureType {
    pub fn model_from(&self, collection: &StructureCollection) -> Handle<Gltf> {
        match self {
            StructureType::Connector => collection.connector.clone(),
            StructureType::HouseSingleSupport => collection.house_single_support.clone(),
            StructureType::HouseSingle => collection.house_single.clone(),
            StructureType::HouseOpenBack => collection.house_open_back.clone(),
            StructureType::MetalSupport => collection.metal_support.clone(),
            StructureType::Ramp => collection.ramp.clone(),
            StructureType::RoofAntenna => collection.roof_antenna.clone(),
            StructureType::RoofOpening => collection.roof_opening.clone(),
            StructureType::RoofRadar => collection.roof_radar.clone(),
            StructureType::RoofVentL => collection.roof_vent_l.clone(),
            StructureType::RoofVentR => collection.roof_vent_r.clone(),
            StructureType::SolarPanelGround => collection.solar_panel_ground.clone(),
            StructureType::SolarPanelRoof => collection.solar_panel_roof.clone(),
            StructureType::SolarPanelStructure => collection.solar_panel_structure.clone(),
            StructureType::Stairs => collection.stairs.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum BushType {
    Bush1,
    Bush2,
    Bush3,
}

#[derive(AssetCollection, Resource)]
pub struct BushCollection {
    #[asset(path = "models/environment/Bush_1.gltf")]
    pub bush_1: Handle<Gltf>,

    #[asset(path = "models/environment/Bush_2.gltf")]
    pub bush_2: Handle<Gltf>,

    #[asset(path = "models/environment/Bush_3.gltf")]
    pub bush_3: Handle<Gltf>,
}

impl BushType {
    pub fn model_from(&self, collection: &BushCollection) -> Handle<Gltf> {
        match self {
            BushType::Bush1 => collection.bush_1.clone(),
            BushType::Bush2 => collection.bush_2.clone(),
            BushType::Bush3 => collection.bush_3.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum GrassType {
    Grass1,
    Grass2,
    Grass3,
}

#[derive(AssetCollection, Resource)]
pub struct GrassCollection {
    #[asset(path = "models/environment/Grass_1.gltf")]
    pub grass_1: Handle<Gltf>,

    #[asset(path = "models/environment/Grass_2.gltf")]
    pub grass_2: Handle<Gltf>,

    #[asset(path = "models/environment/Grass_3.gltf")]
    pub grass_3: Handle<Gltf>,
}

impl GrassType {
    pub fn model_from(&self, collection: &GrassCollection) -> Handle<Gltf> {
        match self {
            GrassType::Grass1 => collection.grass_1.clone(),
            GrassType::Grass2 => collection.grass_2.clone(),
            GrassType::Grass3 => collection.grass_3.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum PlantType {
    Plant1,
    Plant2,
    Plant3,
}

#[derive(AssetCollection, Resource)]
pub struct PlantCollection {
    #[asset(path = "models/environment/Plant_1.gltf")]
    pub plant_1: Handle<Gltf>,

    #[asset(path = "models/environment/Plant_2.gltf")]
    pub plant_2: Handle<Gltf>,

    #[asset(path = "models/environment/Plant_3.gltf")]
    pub plant_3: Handle<Gltf>,
}

impl PlantType {
    pub fn model_from(&self, collection: &PlantCollection) -> Handle<Gltf> {
        match self {
            PlantType::Plant1 => collection.plant_1.clone(),
            PlantType::Plant2 => collection.plant_2.clone(),
            PlantType::Plant3 => collection.plant_3.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum RockType {
    Rock1,
    Rock2,
    Rock3,
    Rock4,
    RockLarge1,
    RockLarge2,
    RockLarge3,
}

#[derive(AssetCollection, Resource)]
pub struct RockCollection {
    #[asset(path = "models/environment/Rock_1.gltf")]
    pub rock_1: Handle<Gltf>,

    #[asset(path = "models/environment/Rock_2.gltf")]
    pub rock_2: Handle<Gltf>,

    #[asset(path = "models/environment/Rock_3.gltf")]
    pub rock_3: Handle<Gltf>,

    #[asset(path = "models/environment/Rock_4.gltf")]
    pub rock_4: Handle<Gltf>,

    #[asset(path = "models/environment/Rock_Large_1.gltf")]
    pub rock_large_1: Handle<Gltf>,

    #[asset(path = "models/environment/Rock_Large_2.gltf")]
    pub rock_large_2: Handle<Gltf>,

    #[asset(path = "models/environment/Rock_Large_3.gltf")]
    pub rock_large_3: Handle<Gltf>,
}

impl RockType {
    pub fn model_from(&self, collection: &RockCollection) -> Handle<Gltf> {
        match self {
            RockType::Rock1 => collection.rock_1.clone(),
            RockType::Rock2 => collection.rock_2.clone(),
            RockType::Rock3 => collection.rock_3.clone(),
            RockType::Rock4 => collection.rock_4.clone(),
            RockType::RockLarge1 => collection.rock_large_1.clone(),
            RockType::RockLarge2 => collection.rock_large_2.clone(),
            RockType::RockLarge3 => collection.rock_large_3.clone(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, EnumIter, Reflect)]
pub enum PlanetType {
    #[default]
    Planet1,
    Planet2,
    Planet3,
    Planet4,
    Planet5,
    Planet6,
    Planet7,
    Planet8,
    Planet9,
    Planet10,
    Planet11,
}

#[derive(AssetCollection, Resource)]
pub struct PlanetCollection {
    #[asset(path = "models/environment/Planet_1.gltf")]
    pub planet_1: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_2.gltf")]
    pub planet_2: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_3.gltf")]
    pub planet_3: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_4.gltf")]
    pub planet_4: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_5.gltf")]
    pub planet_5: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_6.gltf")]
    pub planet_6: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_7.gltf")]
    pub planet_7: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_8.gltf")]
    pub planet_8: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_9.gltf")]
    pub planet_9: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_10.gltf")]
    pub planet_10: Handle<Gltf>,

    #[asset(path = "models/environment/Planet_11.gltf")]
    pub planet_11: Handle<Gltf>,
}

impl PlanetType {
    pub fn model_from(&self, collection: &PlanetCollection) -> Handle<Gltf> {
        match self {
            PlanetType::Planet1 => collection.planet_1.clone(),
            PlanetType::Planet2 => collection.planet_2.clone(),
            PlanetType::Planet3 => collection.planet_3.clone(),
            PlanetType::Planet4 => collection.planet_4.clone(),
            PlanetType::Planet5 => collection.planet_5.clone(),
            PlanetType::Planet6 => collection.planet_6.clone(),
            PlanetType::Planet7 => collection.planet_7.clone(),
            PlanetType::Planet8 => collection.planet_8.clone(),
            PlanetType::Planet9 => collection.planet_9.clone(),
            PlanetType::Planet10 => collection.planet_10.clone(),
            PlanetType::Planet11 => collection.planet_11.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum TreeType {
    Blob1,
    Blob2,
    Blob3,
    Floating1,
    Floating2,
    Floating3,
    Lava1,
    Lava2,
    Lava3,
    Light1,
    Light2,
    Spikes1,
    Spikes2,
    Spiral1,
    Spiral2,
    Spiral3,
    Swirl1,
    Swirl2,
}

#[derive(AssetCollection, Resource)]
pub struct TreeCollection {
    #[asset(path = "models/environment/Tree_Blob_1.gltf")]
    pub blob_1: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Blob_2.gltf")]
    pub blob_2: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Blob_3.gltf")]
    pub blob_3: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Floating_1.gltf")]
    pub floating_1: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Floating_2.gltf")]
    pub floating_2: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Floating_3.gltf")]
    pub floating_3: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Lava_1.gltf")]
    pub lava_1: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Lava_2.gltf")]
    pub lava_2: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Lava_3.gltf")]
    pub lava_3: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Light_1.gltf")]
    pub light_1: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Light_2.gltf")]
    pub light_2: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Spikes_1.gltf")]
    pub spikes_1: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Spikes_2.gltf")]
    pub spikes_2: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Spiral_1.gltf")]
    pub spiral_1: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Spiral_2.gltf")]
    pub spiral_2: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Spiral_3.gltf")]
    pub spiral_3: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Swirl_1.gltf")]
    pub swirl_1: Handle<Gltf>,

    #[asset(path = "models/environment/Tree_Swirl_2.gltf")]
    pub swirl_2: Handle<Gltf>,
}

impl TreeType {
    pub fn model_from(&self, collection: &TreeCollection) -> Handle<Gltf> {
        match self {
            TreeType::Blob1 => collection.blob_1.clone(),
            TreeType::Blob2 => collection.blob_2.clone(),
            TreeType::Blob3 => collection.blob_3.clone(),
            TreeType::Floating1 => collection.floating_1.clone(),
            TreeType::Floating2 => collection.floating_2.clone(),
            TreeType::Floating3 => collection.floating_3.clone(),
            TreeType::Lava1 => collection.lava_1.clone(),
            TreeType::Lava2 => collection.lava_2.clone(),
            TreeType::Lava3 => collection.lava_3.clone(),
            TreeType::Light1 => collection.light_1.clone(),
            TreeType::Light2 => collection.light_2.clone(),
            TreeType::Spikes1 => collection.spikes_1.clone(),
            TreeType::Spikes2 => collection.spikes_2.clone(),
            TreeType::Spiral1 => collection.spiral_1.clone(),
            TreeType::Spiral2 => collection.spiral_2.clone(),
            TreeType::Spiral3 => collection.spiral_3.clone(),
            TreeType::Swirl1 => collection.swirl_1.clone(),
            TreeType::Swirl2 => collection.swirl_2.clone(),
        }
    }
}
