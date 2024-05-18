//! Process user input and trigger game logic events with it.

use bevy::prelude::*;

use crate::core::{inventory::Inventory, CoreSystemSet, SpawnUnit};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, InputSystemSet.before(CoreSystemSet))
            .add_systems(Update, handle_input.in_set(InputSystemSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InputSystemSet;

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut inventory: ResMut<Inventory>,
    mut spawn_unit_event: EventWriter<SpawnUnit>,
) {
    if keyboard_input.just_released(KeyCode::KeyQ) && inventory.coins >= 100. {
        inventory.coins -= 100.;
        spawn_unit_event.send(SpawnUnit);
    }
}
