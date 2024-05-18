//! The core game play logic.
//!
//! Everything else depends on this module.

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_turborand::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, RngPlugin::default()))
            .add_event::<SpawnUnit>()
            .add_systems(Startup, setup)
            .add_systems(Update, (coin_generation, spawn_unit, move_units).chain());
    }
}

#[derive(Debug, Event)]
pub struct SpawnUnit;

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

#[derive(Debug, Resource, Default)]
pub struct Resources {
    pub coins: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.init_resource::<Resources>();

    let custom_meshes = CustomMeshes {
        unit: Mesh2dHandle(meshes.add(Capsule2d::new(6.0, 15.0))),
    };
    let custom_materials = CustomMaterials {
        unit: materials.add(Color::WHITE),
    };

    commands.spawn((
        Player,
        Base,
        RngComponent::from(&mut global_rng),
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

fn coin_generation(mut resources: ResMut<Resources>, time: Res<Time>) {
    resources.coins += 10. * time.delta_seconds();
    println!("Coins: {}", resources.coins);
}

fn spawn_unit(
    mut spawn_unit_event: EventReader<SpawnUnit>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    meshes: Res<CustomMeshes>,
    materials: Res<CustomMaterials>,
    base_transform: Query<&Transform, (With<Base>, With<Player>)>,
) {
    for _ in spawn_unit_event.read() {
        let mut rng_component = RngComponent::from(&mut global_rng);

        let mut transform = *base_transform.single();
        transform.translation.z += 100. + rng_component.f32() * 10.;
        transform.translation.y += rng_component.f32() * 2.;

        commands.spawn((
            Player,
            Unit,
            rng_component,
            MaterialMesh2dBundle {
                mesh: meshes.unit.clone(),
                material: materials.unit.clone(),
                transform,
                ..default()
            },
        ));
    }
}

fn move_units(mut unit_query: Query<&mut Transform, (With<Player>, With<Unit>)>, time: Res<Time>) {
    for mut transform in unit_query.iter_mut() {
        transform.translation += Vec3::new(10., 0., 0.) * time.delta_seconds();
    }
}
