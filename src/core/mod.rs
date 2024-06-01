//! The core game play logic.
//!
//! Everything else depends on this module.

use std::fmt::Display;
use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_turborand::prelude::*;
use stats::AttackStats;

use self::{
    game_state::GameState,
    inventory::{Inventory, Item},
    stats::{Health, MovementStats},
};

pub mod game_state;
pub mod inventory;
pub mod stats;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RngPlugin::default())
            .add_event::<SpawnUnit>()
            .add_event::<Attack>()
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::InGame), setup_in_game)
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
                        (
                            move_units,
                            (attack_animation, attack, die, game_end).chain(),
                        ),
                    )
                        .chain(),
                )
                    .chain()
                    .in_set(CoreSystemSet)
                    .run_if(in_state(GameState::InGame)),
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
    stats: AttackStats,
    transform: Transform,
    direction: Vec3,
}

#[derive(Debug, Component)]
pub struct Base;

#[derive(Debug, Component)]
pub struct Foe;

#[derive(Debug, Component)]
pub struct Unit;

#[derive(Debug, Component)]
pub struct Projectile {
    pub is_foe: bool,
    pub speed: f32,
}

#[derive(Debug, Component, Clone, Copy)]
pub enum UnitType {
    Farmer,
    Archer,
    Shadow,
}

impl UnitType {
    pub fn cost(&self) -> u32 {
        match *self {
            Self::Farmer => 10,
            Self::Archer => 20,
            Self::Shadow => 0,
        }
    }

    pub fn player_units() -> Vec<Self> {
        vec![Self::Farmer, Self::Archer]
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::Farmer => "Farmer",
            Self::Archer => "Archer",
            Self::Shadow => "Shadow",
        };

        write!(f, "{name}")
    }
}

#[derive(States, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Winner {
    Player,
    Enemy,
}

#[derive(Debug, Resource)]
pub struct GameStats {
    pub winner: Winner,
}

#[derive(Debug, Component)]
pub enum Attacking {
    Start,
    Foreswing(Timer),
    Backswing(Timer),
}

fn setup_in_game(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.insert_resource(Inventory {
        coins: Item::empty(100),
    });

    commands.spawn((
        Base,
        Health::from_max(100.),
        RngComponent::from(&mut global_rng),
        TransformBundle {
            local: Transform::from_xyz(-200., 0., -10.),
            ..default()
        },
        VisibilityBundle::default(),
    ));

    commands.spawn((
        Base,
        Foe,
        Health::from_max(100.),
        RngComponent::from(&mut global_rng),
        TransformBundle {
            local: Transform::from_xyz(200., 0., -10.),
            ..default()
        },
        VisibilityBundle::default(),
    ));
}

fn coin_generation(mut inventory: ResMut<Inventory>, time: Res<Time>) {
    inventory.coins.add_until_full(2. * time.delta_seconds());
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

        let base_transform = if *is_foe {
            foe_base.get_single()
        } else {
            friend_base.get_single()
        };
        let Ok(base_transform) = base_transform else {
            continue;
        };
        let mut transform = *base_transform;

        transform.translation.z += 100. + rng_component.f32() * 10.;
        transform.translation.y += rng_component.f32() * 2.;

        let id = commands
            .spawn((
                Unit,
                MovementStats::from(*unit_type),
                AttackStats::from(*unit_type),
                Health::from(*unit_type),
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
    unit_query: Query<
        (Entity, &Transform, &AttackStats, Has<Foe>),
        (With<Unit>, Without<Attacking>),
    >,
    other_query: Query<(&Transform, Has<Foe>), Or<(With<Unit>, With<Base>)>>,
) {
    for (entity, transform, stats, is_foe) in unit_query.iter() {
        let direction = if is_foe { -1. } else { 1. };

        let is_in_attack_range = other_query.iter().any(|(other_transform, is_other_foe)| {
            if is_foe == is_other_foe {
                // Only attack units from the other fraction
                return false;
            }

            let x = transform.translation.x;
            let other_x = other_transform.translation.x;

            let distance = other_x - x;

            if distance.signum() != direction {
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
    mut unit_query: Query<
        (&mut Transform, &MovementStats, Has<Foe>),
        (With<Unit>, Without<Attacking>),
    >,
    time: Res<Time>,
) {
    for (mut transform, stats, is_foe) in unit_query.iter_mut() {
        let direction = if is_foe { -1. } else { 1. };

        transform.translation += Vec3::new(direction, 0., 0.) * stats.speed * time.delta_seconds();
    }
}

fn attack_animation(
    mut commands: Commands,
    mut attack_event: EventWriter<Attack>,
    mut unit_query: Query<(Entity, &mut Attacking, &Transform, &AttackStats, Has<Foe>), With<Unit>>,
    time: Res<Time>,
) {
    for (entity, mut attacking, transform, attack_stats, is_foe) in unit_query.iter_mut() {
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
                        stats: attack_stats.clone(),
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
    mut target_query: Query<(&Transform, Has<Foe>, &mut Health)>,
) {
    for Attack {
        is_foe,
        stats: unit_stats,
        transform,
        direction,
    } in attack_event.read()
    {
        let closest_unit = target_query
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

        if let Some((_, _, mut health)) = closest_unit {
            health.apply_damage(unit_stats.attack_damage);
        }
    }
}

fn die(mut commands: Commands, unit_query: Query<(Entity, &Health)>) {
    for (unit, health) in unit_query.iter() {
        if health.is_dead() {
            commands.entity(unit).despawn_recursive();
        }
    }
}

fn game_end(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    foe_bases: Query<Entity, (With<Base>, With<Foe>)>,
    player_bases: Query<Entity, (With<Base>, Without<Foe>)>,
) {
    let winner = if foe_bases.iter().count() == 0 {
        Some(Winner::Player)
    } else if player_bases.iter().count() == 0 {
        Some(Winner::Enemy)
    } else {
        None
    };

    if let Some(winner) = winner {
        commands.insert_resource(GameStats { winner });
        next_state.set(GameState::PostGame);
    }
}
