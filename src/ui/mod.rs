use bevy::prelude::*;

use self::in_game::InGameUiPlugin;

mod in_game;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InGameUiPlugin);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UiSystemSet;
