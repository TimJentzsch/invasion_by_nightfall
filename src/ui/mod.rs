use bevy::prelude::*;

use self::{in_game::InGameUiPlugin, post_game::PostGameUiPlugin};

mod in_game;
mod post_game;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InGameUiPlugin, PostGameUiPlugin));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UiSystemSet;
