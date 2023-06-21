use std::{fs::File, io::Write};

use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, scene::DynamicSceneBundle,
    tasks::IoTaskPool,
};
use bevy_xpbd_3d::prelude::{Friction, Mass, Position, RigidBody};

use crate::{GameState, Junk, Planet};

pub struct SavingPlugin;

impl Plugin for SavingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Planet>()
            .register_type::<Junk>()
            .register_type::<Transform>()
            .register_type::<Friction>()
            .register_type::<Mass>()
            .register_type::<Position>()
            .register_type::<RigidBody>()
            .add_system(
                save_scene_system.in_schedule(OnEnter(GameState::Playing)), // .run_if(input_toggle_active(false, KeyCode::Escape)),
            );
    }
}

const NEW_SCENE_FILE_PATH: &str = "scenes/scene_one.scn.ron";

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load(NEW_SCENE_FILE_PATH),
        ..default()
    });
}

fn save_scene_system(world: &mut World) {
    // The TypeRegistry resource contains information about all registered types (including
    // components). This is used to construct scenes.
    let mut query = world.query_filtered::<Entity, With<Transform>>();
    let type_registry = world.resource::<AppTypeRegistry>();

    // Scenes can be created from any ECS World. You can either create a new one for the scene or
    // use the current World.
    let scene_world = World::new();
    let mut scene_builder = DynamicSceneBuilder::from_world(&scene_world);

    scene_builder.extract_entities(query.iter(&world));

    let scene = scene_builder.build();

    // Scenes can be serialized like this:
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
