//! Display the game on the screen.

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::core::{Base, CoreSystemSet, Unit};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .configure_sets(Startup, RenderingSystemSet.after(CoreSystemSet))
            .configure_sets(Update, RenderingSystemSet.after(CoreSystemSet))
            .add_systems(
                Startup,
                (setup, setup_base_graphics)
                    .chain()
                    .in_set(RenderingSystemSet),
            )
            .add_systems(Update, spawn_unit_graphics.in_set(RenderingSystemSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct RenderingSystemSet;

#[derive(Debug, Resource)]
struct CustomMeshes {
    unit: Mesh2dHandle,
    base: Mesh2dHandle,
}

#[derive(Debug, Resource)]
struct CustomMaterials {
    unit: Handle<ColorMaterial>,
    base: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let custom_meshes = CustomMeshes {
        unit: Mesh2dHandle(meshes.add(Capsule2d::new(6.0, 15.0))),
        base: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 150.0))),
    };
    let custom_materials = CustomMaterials {
        unit: materials.add(Color::WHITE),
        base: materials.add(Color::GRAY),
    };

    commands.insert_resource(custom_meshes);
    commands.insert_resource(custom_materials);
}

fn setup_base_graphics(
    mut commands: Commands,
    meshes: Res<CustomMeshes>,
    materials: Res<CustomMaterials>,
    spawned_base_query: Query<Entity, Added<Base>>,
) {
    for spawned_base in spawned_base_query.iter() {
        commands.entity(spawned_base).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.base.clone(),
                material: materials.base.clone(),
                ..default()
            });
        });
    }
}

fn spawn_unit_graphics(
    mut commands: Commands,
    meshes: Res<CustomMeshes>,
    materials: Res<CustomMaterials>,
    spawned_unit_query: Query<Entity, Added<Unit>>,
) {
    for spawned_unit in spawned_unit_query.iter() {
        commands.entity(spawned_unit).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.unit.clone(),
                material: materials.unit.clone(),
                ..default()
            });
        });
    }
}
