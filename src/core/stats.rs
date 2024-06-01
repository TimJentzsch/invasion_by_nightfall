use bevy::prelude::*;

use std::fmt::Display;

use super::UnitType;

#[derive(Debug, Component, Clone)]
pub struct MovementStats {
    pub speed: f32,
}

impl From<UnitType> for MovementStats {
    fn from(unit_type: UnitType) -> Self {
        match unit_type {
            UnitType::Farmer => Self { speed: 10. },
            UnitType::Shadow => Self { speed: 10. },
            UnitType::Archer => Self { speed: 10. },
        }
    }
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn from_max(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn apply_damage(&mut self, damage: f32) {
        self.current -= damage;
    }
}

impl From<UnitType> for Health {
    fn from(value: UnitType) -> Self {
        let max = match value {
            UnitType::Farmer => 5.,
            UnitType::Shadow => 10.,
            UnitType::Archer => 2.,
        };

        Self::from_max(max)
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} / {:.0}", self.current, self.max)
    }
}

#[derive(Debug, Component, Clone)]
pub struct AttackStats {
    pub attack_range: f32,
    pub attack_damage: f32,
}

impl From<UnitType> for AttackStats {
    fn from(value: UnitType) -> Self {
        match value {
            UnitType::Farmer => Self {
                attack_range: 25.,
                attack_damage: 2.,
            },

            UnitType::Shadow => Self {
                attack_range: 20.,
                attack_damage: 2.,
            },

            UnitType::Archer => Self {
                attack_range: 100.,
                attack_damage: 2.,
            },
        }
    }
}
