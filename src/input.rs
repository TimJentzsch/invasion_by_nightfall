//! Process user input and trigger game logic events with it.

use bevy::prelude::*;

use crate::core::{
    game_state::GameState, inventory::Inventory, CoreSystemSet, SpawnUnit, UnitType,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            InputSystemSet
                .before(CoreSystemSet)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, handle_input.in_set(InputSystemSet));
    }
}

pub struct InputData {
    pub key: KeyCode,
    pub glyph: String,
}

impl InputData {
    pub fn from_slot(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self {
                key: KeyCode::KeyQ,
                glyph: "Q".to_string(),
            }),
            1 => Some(Self {
                key: KeyCode::KeyW,
                glyph: "W".to_string(),
            }),
            2 => Some(Self {
                key: KeyCode::KeyE,
                glyph: "E".to_string(),
            }),
            3 => Some(Self {
                key: KeyCode::KeyR,
                glyph: "R".to_string(),
            }),
            _ => None,
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InputSystemSet;

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut inventory: ResMut<Inventory>,
    mut spawn_unit_event: EventWriter<SpawnUnit>,
) {
    for (index, &unit_type) in UnitType::player_units().iter().enumerate() {
        let key = InputData::from_slot(index).unwrap().key;

        if keyboard_input.just_released(key) && inventory.coins.try_remove(unit_type.cost()) {
            spawn_unit_event.send(SpawnUnit {
                is_foe: false,
                unit_type,
            });
        }
    }
}
