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
            .add_event::<Attack>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    coin_generation,
                    spawn_unit,
                    (unit_behavior, (move_units, attack_animation)).chain(),
                    move_units,
                )
                    .chain()
                    .in_set(CoreSystemSet),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]

pub struct CoreSystemSet;

#[derive(Debug, Event)]
pub struct SpawnUnit;

#[derive(Debug, Event)]
pub struct Attack;

#[derive(Debug, Component)]
pub struct Base;

#[derive(Debug, Component)]
pub struct Foe;

#[derive(Debug, Component)]
pub struct Unit;

#[derive(Debug, Component)]
pub struct UnitStats {
    speed: f32,
    direction: Vec3,
}

#[derive(Debug, Component)]
pub enum Attacking {
    Start,
    Foreswing(Timer),
    Backswing(Timer),
}

fn setup(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.insert_resource(Inventory {
        coins: Item::empty(1000),
    });

    commands.spawn((
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
    base_transform: Query<&Transform, (With<Base>, Without<Foe>)>,
) {
    for _ in spawn_unit_event.read() {
        let mut rng_component = RngComponent::from(&mut global_rng);

        let mut transform = *base_transform.single();
        transform.translation.z += 100. + rng_component.f32() * 10.;
        transform.translation.y += rng_component.f32() * 2.;

        commands.spawn((
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

fn unit_behavior(
    mut commands: Commands,
    unit_query: Query<(Entity, &Transform, &UnitStats), (With<Unit>, Without<Attacking>)>,
) {
    for (entity, transform, stats) in unit_query.iter() {
        let is_in_attack_range = unit_query.iter().any(|(_, other_transform, other_stats)| {
            if (other_stats.direction - stats.direction).length() < 0.5 {
                // Only attack units moving in the opposite direction
                return false;
            }

            let x = transform.translation.x;
            let other_x = other_transform.translation.x;

            // Only attack units in front of you
            other_x - x >= stats.direction.x
        });

        if is_in_attack_range {
            commands.entity(entity).insert(Attacking::Start);
        }
    }
}

fn move_units(
    mut unit_query: Query<(&mut Transform, &UnitStats), (With<Unit>, Without<Attacking>)>,
    time: Res<Time>,
) {
    for (mut transform, stats) in unit_query.iter_mut() {
        transform.translation += stats.direction * stats.speed * time.delta_seconds();
    }
}

fn attack_animation(
    mut commands: Commands,
    mut attack_event: EventWriter<Attack>,
    mut unit_query: Query<(Entity, &mut Attacking), With<Unit>>,
    time: Res<Time>,
) {
    for (entity, mut attacking) in unit_query.iter_mut() {
        match &mut *attacking {
            Attacking::Start => {
                // Start the foreswing anymation
                *attacking = Attacking::Foreswing(Timer::from_seconds(1., TimerMode::Once))
            }
            Attacking::Foreswing(ref mut timer) => {
                if timer.tick(time.delta()).finished() {
                    // After the foreswing is complete, execute the attack and start the backswing
                    attack_event.send(Attack);
                    *attacking = Attacking::Backswing(Timer::from_seconds(0.5, TimerMode::Once));
                }
            }
            Attacking::Backswing(ref mut timer) => {
                if timer.tick(time.delta()).finished() {
                    // After the backswing is complete, the unit is no longer attacking
                    commands.entity(entity).remove::<Attacking>();
                }
            }
        }
    }
}
