use core::CorePlugin;

use bevy::prelude::*;
use input::InputPlugin;
use rendering::RenderingPlugin;
use ui::UiPlugin;

mod core;
mod input;
mod rendering;
mod ui;

fn main() {
    App::new()
        .add_plugins((CorePlugin, RenderingPlugin, UiPlugin, InputPlugin))
        .run();
}
