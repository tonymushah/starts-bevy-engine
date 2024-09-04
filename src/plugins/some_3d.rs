use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    pbr: PbrBundle,
    player: Player,
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::new(5.0, 5.0))),
        material: materials.add(Color::srgb_from_array([1.0, 1.0, 1.0])),
        ..Default::default()
    });
    cmd.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 6.0, 2.0),
            ..Default::default()
        },
    ));

    let cuboid = Cuboid::new(1.0, 1.0, 1.0);
    cmd.spawn(PlayerBundle {
        pbr: PbrBundle {
            mesh: meshes.add(cuboid),
            material: materials.add(Color::srgb_from_array([0.3, 0.9, 0.3])),
            transform: Transform::from_xyz(0.0, 0.0, 0.5),
            ..Default::default()
        },
        ..Default::default()
    });
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(1.0, 1.0, 3.0),
        ..Default::default()
    });
}

fn camera_follow_look(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player = player_query.single();
    let mut camera = camera_query.single_mut();
    *camera = camera.looking_at(player.translation, Vec3::Z);
}

pub struct Some3D;

impl Plugin for Some3D {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_follow_look);
    }
}
