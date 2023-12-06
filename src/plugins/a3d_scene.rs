use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_xpbd_3d::prelude::*;

pub struct A3dScenePlugin;

#[derive(Bundle)]
pub struct MyCube {
    pbr: PbrBundle,
    comp: ControllableCube,
    rigid: RigidBody,
    collider: Collider,
}

impl Default for MyCube {
    fn default() -> Self {
        let pbr = PbrBundle::default();
        Self {
            pbr,
            comp: ControllableCube::default(),
            rigid: RigidBody::Dynamic,
            collider: Collider::default(),
        }
    }
}

#[derive(Clone, Copy, Component, Debug)]
pub struct ControllableCube {
    move_speed: f32,
    is_on_ground: bool,
}

#[derive(Component)]
pub struct Ground;

impl Default for ControllableCube {
    fn default() -> Self {
        Self {
            move_speed: 2.0,
            is_on_ground: false,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(5.0, 0.0, 5.0),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground,
    ));
    // cube
    let cube_mesh = Mesh::from(shape::Cube { size: 1.0 });
    commands.spawn(MyCube {
        collider: Collider::trimesh_from_mesh(&cube_mesh).unwrap_or_default(),
        pbr: PbrBundle {
            mesh: meshes.add(cube_mesh),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ..Default::default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.5, 2.75, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        ..default()
    });
}

impl Plugin for A3dScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (handle_mouvement, handle_is_ground, handle_jump));
    }
}

fn handle_mouvement(
    mut controllable_cubes: Query<(&mut Transform, &ControllableCube)>,
    input: Res<Input<KeyCode>>,
    timer: Res<Time>,
) {
    let (mut transform, inner) = controllable_cubes.single_mut();
    if input.pressed(KeyCode::W) {
        let next = transform.forward();
        transform.translation += next * inner.move_speed * timer.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        let next = transform.left();
        transform.translation += next * inner.move_speed * timer.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        let next = transform.right();
        transform.translation += next * inner.move_speed * timer.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        let next = transform.back();
        transform.translation += next * inner.move_speed * timer.delta_seconds();
    }
}

fn handle_is_ground(
    mut controllable_cubes: Query<(&mut ControllableCube, &CollidingEntities)>,
    grounds: Query<Entity, With<Ground>>,
) {
    let (mut state, entities) = controllable_cubes.single_mut();
    state.is_on_ground = entities.iter().any(|e| grounds.single().cmp(e).is_eq());
}

fn handle_jump(
    mut commands: Commands,
    mut controllable_cubes: Query<(Entity, &ControllableCube), With<RigidBody>>,
    input: Res<Input<KeyCode>>,
) {
    let (entity, state) = controllable_cubes.single_mut();
    if input.pressed(KeyCode::Space) && state.is_on_ground {
        commands
            .entity(entity)
            .insert(ExternalImpulse::new(Vec3::Y));
    }
}
