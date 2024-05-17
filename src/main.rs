use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::common_conditions::on_timer,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_unit.run_if(on_timer(Duration::from_secs(5))),
                move_units,
            )
                .chain(),
        )
        .run();
}

#[derive(Debug, Component)]
struct Base;

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct Unit;

#[derive(Debug, Resource)]
struct CustomMeshes {
    unit: Mesh2dHandle,
}

#[derive(Debug, Resource)]
struct CustomMaterials {
    unit: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let custom_meshes = CustomMeshes {
        unit: Mesh2dHandle(meshes.add(Capsule2d::new(6.0, 15.0))),
    };
    let custom_materials = CustomMaterials {
        unit: materials.add(Color::WHITE),
    };

    commands.spawn((
        Player,
        Base,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 150.0))),
            material: materials.add(Color::GRAY),
            transform: Transform::from_xyz(-400., 0., -10.),
            ..default()
        },
    ));

    commands.insert_resource(custom_meshes);
    commands.insert_resource(custom_materials);
}

fn spawn_unit(
    mut commands: Commands,
    meshes: Res<CustomMeshes>,
    materials: Res<CustomMaterials>,
    base_query: Query<&Transform, (With<Base>, With<Player>)>,
) {
    let mut transform = *base_query.single();
    transform.translation.z += 1.;

    commands.spawn((
        Player,
        Unit,
        MaterialMesh2dBundle {
            mesh: meshes.unit.clone(),
            material: materials.unit.clone(),
            transform,
            ..default()
        },
    ));
}

fn move_units(mut unit_query: Query<&mut Transform, (With<Player>, With<Unit>)>, time: Res<Time>) {
    for mut transform in unit_query.iter_mut() {
        transform.translation += Vec3::new(10., 0., 0.) * time.delta_seconds();
    }
}
