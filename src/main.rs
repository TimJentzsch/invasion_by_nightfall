use core::CorePlugin;

use bevy::prelude::*;
use input::InputPlugin;

mod core;
mod input;

fn main() {
    App::new().add_plugins((CorePlugin, InputPlugin)).run();
}
