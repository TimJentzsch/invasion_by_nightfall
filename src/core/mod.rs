//! The core game play logic.
//!
//! Everything else depends on this module.

use bevy::prelude::*;
use bevy_turborand::prelude::*;

use self::inventory::{Inventory, Item};

pub mod inventory;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RngPlugin::default())
            .add_event::<SpawnUnit>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (coin_generation, spawn_unit, move_units)
                    .chain()
                    .in_set(CoreSystemSet),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]

pub struct CoreSystemSet;

#[derive(Debug, Event)]
pub struct SpawnUnit;

#[derive(Debug, Component)]
pub struct Base;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Unit;

#[derive(Debug, Component)]
pub struct UnitStats {
    speed: f32,
    direction: Vec3,
}

fn setup(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.insert_resource(Inventory {
        coins: Item::empty(1000),
    });

    commands.spawn((
        Player,
        Base,
        RngComponent::from(&mut global_rng),
        TransformBundle {
            local: Transform::from_xyz(-400., 0., -10.),
            ..default()
        },
        VisibilityBundle::default(),
    ));
}

fn coin_generation(mut inventory: ResMut<Inventory>, time: Res<Time>) {
    inventory.coins.add_until_full(10. * time.delta_seconds());
}

fn spawn_unit(
    mut spawn_unit_event: EventReader<SpawnUnit>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
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
            UnitStats {
                speed: 10.,
                direction: Vec3::new(1., 0., 0.),
            },
            rng_component,
            TransformBundle {
                local: transform,
                ..default()
            },
            VisibilityBundle::default(),
        ));
    }
}

fn move_units(
    mut unit_query: Query<(&mut Transform, &UnitStats), (With<Player>, With<Unit>)>,
    time: Res<Time>,
) {
    for (mut transform, stats) in unit_query.iter_mut() {
        transform.translation += stats.direction * stats.speed * time.delta_seconds();
    }
}
