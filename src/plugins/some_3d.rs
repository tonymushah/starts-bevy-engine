use bevy::{input::keyboard::KeyboardInput, prelude::*};

#[derive(Component)]
struct MainCamera;

#[derive(Component, Default)]
struct Player;

#[derive(Component)]
struct MoveSpeed(f32);

impl Default for MoveSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    pbr: PbrBundle,
    player: Player,
    move_speed: MoveSpeed,
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
        move_speed: MoveSpeed(2.0),
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

type HandleMouvementPlayerQueryCondition = (With<Player>, Without<MainCamera>);

fn handle_mouvement(
    mut key_board_event: EventReader<KeyboardInput>,
    mut player_query: Query<(&mut Transform, &MoveSpeed), HandleMouvementPlayerQueryCondition>,
    time: Res<Time>,
) {
    let (mut player, move_speed) = player_query.single_mut();
    for event in key_board_event.read() {
        match event.key_code {
            KeyCode::KeyW => {
                player.translation += Vec3::X * move_speed.0 * time.delta_seconds();
            }
            KeyCode::KeyS => {
                player.translation += Vec3::X * -1.0 * move_speed.0 * time.delta_seconds();
            }
            KeyCode::KeyA => {
                player.translation += Vec3::Y * -1.0 * move_speed.0 * time.delta_seconds();
            }
            KeyCode::KeyD => {
                player.translation += Vec3::Y * move_speed.0 * time.delta_seconds();
            }
            _ => {}
        }
    }
}

pub struct Some3D;

impl Plugin for Some3D {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (camera_follow_look, handle_mouvement));
    }
}
