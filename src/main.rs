use bevy::prelude::*;
use plugins::window_test::WindowTestPlugin;
pub mod plugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WindowTestPlugin))
        .run();
}
