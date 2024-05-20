use bevy::prelude::*;

use std::fmt::Display;

use super::UnitType;

#[derive(Debug, Component, Clone)]
pub struct UnitStats {
    pub speed: f32,
    pub attack_range: f32,
    pub attack_damage: f32,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn from_unit(unit_type: &UnitType) -> Self {
        let max = match *unit_type {
            UnitType::Farmer => 5.,
            UnitType::Shadow => 10.,
        };

        Self::from_max(max)
    }

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

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} / {:.0}", self.current, self.max)
    }
}
