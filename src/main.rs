pub mod plugins;
use bevy::prelude::*;
use plugins::some_3d::Some3D;

fn main() {
    App::new().add_plugins((DefaultPlugins, Some3D)).run();
}
