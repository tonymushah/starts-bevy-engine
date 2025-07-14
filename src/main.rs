use std::f32::consts::TAU;

use bevy::{prelude::*, render::render_resource::AsBindGroup};

#[derive(Debug, AsBindGroup, TypePath, Asset, Clone)]
struct MyCustomShader {
    #[uniform(0)]
    time: f32,
}

impl Material for MyCustomShader {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/custom_shader.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/custom_shader.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut my_custom_shaders: ResMut<Assets<MyCustomShader>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(my_custom_shaders.add(MyCustomShader { time: 0.0 })),
        Rotatable { speed: 0.3 },
    ));

    commands.spawn((
        PointLight::default(),
        Transform {
            translation: Vec3 {
                x: 4.0,
                y: 3.0,
                z: 2.0,
            },
            ..Default::default()
        },
    ));

    commands.spawn((
        Camera3d::default(),
        Transform {
            translation: Vec3 {
                x: 3.0,
                y: 4.0,
                z: -2.0,
            },
            ..Default::default()
        }
        .looking_at(Vec3::default(), Vec3::Y),
    ));
}

fn update_time_texture(mut my_custom_shaders: ResMut<Assets<MyCustomShader>>, time: Res<Time>) {
    for (_, material) in my_custom_shaders.iter_mut() {
        material.time = time.elapsed_secs();
    }
}
#[derive(Component)]
struct Rotatable {
    speed: f32,
}

fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(cube.speed * TAU * timer.delta_secs());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: if cfg!(debug_assertions) && cfg!(feature = "asset_reload")
            {
                Some(true)
            } else {
                None
            },
            ..Default::default()
        }))
        .add_plugins(MaterialPlugin::<MyCustomShader>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_time_texture)
        .add_systems(FixedUpdate, rotate_cube)
        .run();
}
