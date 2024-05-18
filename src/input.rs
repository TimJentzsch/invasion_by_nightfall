//! Process user input and trigger game logic events with it.

use bevy::prelude::*;

use crate::core::{CoreSystemSet, Resources, SpawnUnit};

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
    mut resources: ResMut<Resources>,
    mut spawn_unit_event: EventWriter<SpawnUnit>,
) {
    if keyboard_input.just_released(KeyCode::KeyQ) && resources.coins >= 100. {
        resources.coins -= 100.;
        spawn_unit_event.send(SpawnUnit);
    }
}
