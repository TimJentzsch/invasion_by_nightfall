use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct Inventory {
    pub coins: f32,
}
