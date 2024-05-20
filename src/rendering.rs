//! Display the game on the screen.

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::core::{game_state::GameState, Base, CoreSystemSet, Foe, Unit};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .configure_sets(
                OnEnter(GameState::InGame),
                RenderingSystemSet.after(CoreSystemSet),
            )
            .configure_sets(
                Update,
                RenderingSystemSet
                    .after(CoreSystemSet)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                (setup_in_game, setup_base_graphics)
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
    friend_unit: Handle<ColorMaterial>,
    foe_unit: Handle<ColorMaterial>,
    base: Handle<ColorMaterial>,
}

fn setup_in_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let custom_meshes = CustomMeshes {
        unit: Mesh2dHandle(meshes.add(Capsule2d::new(10.0, 20.0))),
        base: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 150.0))),
    };
    let custom_materials = CustomMaterials {
        friend_unit: materials.add(Color::WHITE),
        foe_unit: materials.add(Color::BLACK),
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
    spawned_unit_query: Query<(Entity, Has<Foe>), Added<Unit>>,
) {
    for (spawned_unit, is_foe) in spawned_unit_query.iter() {
        commands.entity(spawned_unit).with_children(|parent| {
            let material = if is_foe {
                materials.foe_unit.clone()
            } else {
                materials.friend_unit.clone()
            };
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.unit.clone(),
                material,
                ..default()
            });
        });
    }
}
