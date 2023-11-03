use bevy::prelude::*;
use plugins::a3d_scene::A3dScenePlugin;
pub mod plugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, A3dScenePlugin))
        .run();
}
