use bevy::prelude::*;
pub mod plugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::hello::HelloPlugin))
        .run();
}
