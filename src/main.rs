use core::CorePlugin;

use bevy::prelude::*;
use input::InputPlugin;
use rendering::RenderingPlugin;

mod core;
mod input;
mod rendering;

fn main() {
    App::new()
        .add_plugins((CorePlugin, RenderingPlugin, InputPlugin))
        .run();
}
