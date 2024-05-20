//! The core game play logic.
//!
//! Everything else depends on this module.

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_turborand::prelude::*;

use self::{
    inventory::{Inventory, Item},
    stats::{Health, UnitStats},
};

pub mod inventory;
pub mod stats;

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
                    (
                        coin_generation,
                        generate_waves.run_if(on_timer(Duration::from_secs(5))),
                    ),
                    spawn_unit,
                    (
                        unit_behavior,
                        (move_units, (attack_animation, attack, die).chain()),
                    )
                        .chain(),
                )
                    .chain()
                    .in_set(CoreSystemSet),
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]

pub struct CoreSystemSet;

#[derive(Debug, Event)]
pub struct SpawnUnit {
    pub is_foe: bool,
    pub unit_type: UnitType,
}

#[derive(Debug, Event)]
pub struct Attack {
    is_foe: bool,
    unit_stats: UnitStats,
    transform: Transform,
    direction: Vec3,
}

#[derive(Debug, Component)]
pub struct Base;

#[derive(Debug, Component)]
pub struct Foe;

#[derive(Debug, Component)]
pub struct Unit;

#[derive(Debug, Component, Clone, Copy)]
pub enum UnitType {
    Farmer,
    Shadow,
}

impl UnitType {
    pub fn cost(&self) -> u32 {
        match *self {
            Self::Farmer => 10,
            Self::Shadow => 0,
        }
    }

    pub fn stats(&self) -> UnitStats {
        match *self {
            Self::Farmer => UnitStats {
                health: Health::from_max(5.),
                speed: 10.,
                direction: Vec3::new(1., 0., 0.),
                attack_range: 20.,
                attack_damage: 2.,
            },

            Self::Shadow => UnitStats {
                health: Health::from_max(10.),
                speed: 10.,
                direction: Vec3::new(-1., 0., 0.),
                attack_range: 10.,
                attack_damage: 2.,
            },
        }
    }
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

    commands.spawn((
        Base,
        Foe,
        RngComponent::from(&mut global_rng),
        TransformBundle {
            local: Transform::from_xyz(400., 0., -10.),
            ..default()
        },
        VisibilityBundle::default(),
    ));
}

fn coin_generation(mut inventory: ResMut<Inventory>, time: Res<Time>) {
    inventory.coins.add_until_full(10. * time.delta_seconds());
}

fn generate_waves(mut spawn_unit_event: EventWriter<SpawnUnit>) {
    spawn_unit_event.send(SpawnUnit {
        is_foe: true,
        unit_type: UnitType::Shadow,
    });
}

fn spawn_unit(
    mut spawn_unit_event: EventReader<SpawnUnit>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    friend_base: Query<&Transform, (With<Base>, Without<Foe>)>,
    foe_base: Query<&Transform, (With<Base>, With<Foe>)>,
) {
    for SpawnUnit { is_foe, unit_type } in spawn_unit_event.read() {
        let mut rng_component = RngComponent::from(&mut global_rng);

        let mut transform = if *is_foe {
            *foe_base.single()
        } else {
            *friend_base.single()
        };
        transform.translation.z += 100. + rng_component.f32() * 10.;
        transform.translation.y += rng_component.f32() * 2.;

        let id = commands
            .spawn((
                Unit,
                unit_type.stats(),
                *unit_type,
                rng_component,
                TransformBundle {
                    local: transform,
                    ..default()
                },
                VisibilityBundle::default(),
            ))
            .id();

        if *is_foe {
            commands.entity(id).insert(Foe);
        }
    }
}

fn unit_behavior(
    mut commands: Commands,
    unit_query: Query<(Entity, &Transform, &UnitStats, Has<Foe>), (With<Unit>, Without<Attacking>)>,
    other_query: Query<(&Transform, Has<Foe>), Or<(With<Unit>, With<Base>)>>,
) {
    for (entity, transform, stats, is_foe) in unit_query.iter() {
        let is_in_attack_range = other_query.iter().any(|(other_transform, is_other_foe)| {
            if is_foe == is_other_foe {
                // Only attack units from the other fraction
                return false;
            }

            let x = transform.translation.x;
            let other_x = other_transform.translation.x;

            let distance = other_x - x;

            if distance.signum() != stats.direction.x {
                // Only attack units in front of you
                return false;
            }

            // Only attack units within the attack range
            distance.abs() <= stats.attack_range
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
    mut unit_query: Query<(Entity, &mut Attacking, &Transform, &UnitStats, Has<Foe>), With<Unit>>,
    time: Res<Time>,
) {
    for (entity, mut attacking, transform, unit_stats, is_foe) in unit_query.iter_mut() {
        match &mut *attacking {
            Attacking::Start => {
                // Start the foreswing anymation
                *attacking = Attacking::Foreswing(Timer::from_seconds(1., TimerMode::Once))
            }
            Attacking::Foreswing(ref mut timer) => {
                if timer.tick(time.delta()).finished() {
                    // After the foreswing is complete, execute the attack and start the backswing
                    attack_event.send(Attack {
                        direction: if is_foe {
                            Vec3::new(-1., 0., 0.)
                        } else {
                            Vec3::new(1., 0., 0.)
                        },
                        is_foe,
                        transform: *transform,
                        unit_stats: unit_stats.clone(),
                    });
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

fn attack(
    mut attack_event: EventReader<Attack>,
    mut unit_query: Query<(&Transform, Has<Foe>, &mut UnitStats), With<Unit>>,
) {
    for Attack {
        is_foe,
        unit_stats,
        transform,
        direction,
    } in attack_event.read()
    {
        let closest_unit = unit_query
            .iter_mut()
            .filter(|(other_transform, is_other_foe, _)| {
                if is_foe == is_other_foe {
                    return false;
                }

                let x = transform.translation.x;
                let other_x = other_transform.translation.x;

                let distance = other_x - x;

                if distance.signum() != direction.x {
                    // Only attack units in front of you
                    return false;
                }

                // Only attack units within the attack range
                distance.abs() <= unit_stats.attack_range
            })
            .min_by(|(a_transform, _, _), (b_transform, _, _)| {
                let a_distance = transform.translation.distance(a_transform.translation);
                let b_distance = transform.translation.distance(b_transform.translation);

                a_distance
                    .partial_cmp(&b_distance)
                    .unwrap_or(std::cmp::Ordering::Less)
            });

        if let Some((_, _, mut stats)) = closest_unit {
            stats.health.apply_damage(unit_stats.attack_damage);
        }
    }
}

fn die(mut commands: Commands, unit_query: Query<(Entity, &UnitStats)>) {
    for (unit, stats) in unit_query.iter() {
        if stats.health.is_dead() {
            commands.entity(unit).despawn_recursive();
        }
    }
}
