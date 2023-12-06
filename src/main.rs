use bevy::prelude::*;
use plugins::{a3d_scene::A3dScenePlugin, sky::SkyPlugin};
pub mod plugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SkyPlugin, A3dScenePlugin))
        .run();
}
