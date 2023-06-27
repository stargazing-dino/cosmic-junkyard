use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_xpbd_3d::prelude::Collider;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_components<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn collider_from_gltf(
    gltf_handle: Handle<Gltf>,
    gltf_assets: &Res<Assets<Gltf>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    meshes: &ResMut<Assets<Mesh>>,
) -> (Handle<Scene>, Collider) {
    let gltf = gltf_assets.get(&gltf_handle).unwrap();
    let scene = gltf.scenes.first().unwrap().clone();
    let gltf_mesh_handle = gltf.meshes.first().unwrap().clone();
    let gltf_mesh = gltf_meshes.get(&gltf_mesh_handle).unwrap();
    let mesh_handle = gltf_mesh.primitives.first().unwrap().mesh.clone();
    let mesh = meshes.get(&mesh_handle).unwrap();

    (scene, Collider::trimesh_from_bevy_mesh(mesh).unwrap())
}
