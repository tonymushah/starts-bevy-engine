use std::{
    f32::consts::PI,
    ops::{Deref, Mul, MulAssign},
};

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_xpbd_3d::prelude::*;

#[derive(Bundle)]
pub struct MyCube {
    pbr: PbrBundle,
    comp: ControllableCube,
    rigid: RigidBody,
    collider: Collider,
    density: ColliderDensity,
    player: Player,
    have_dashed: HaveDashed,
    move_speed: MoveSpeed,
    is_on_ground: IsOnGround,
}

impl Default for MyCube {
    fn default() -> Self {
        let pbr = PbrBundle::default();
        Self {
            pbr,
            comp: ControllableCube,
            rigid: RigidBody::Dynamic,
            collider: Collider::default(),
            density: ColliderDensity(1.0),
            player: Player,
            have_dashed: HaveDashed(false),
            move_speed: MoveSpeed(2.0),
            is_on_ground: IsOnGround(false),
        }
    }
}

#[derive(Clone, Copy, Component, Debug)]
pub struct ControllableCube;

#[derive(Clone, Copy, Component, Debug)]
pub struct MoveSpeed(f32);

impl MulAssign for MoveSpeed {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Mul<f32> for MoveSpeed {
    type Output = f32;
    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

#[derive(Clone, Copy, Component, Debug)]
pub struct IsOnGround(bool);

impl Deref for IsOnGround {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IsOnGround {
    pub fn set_inner(&mut self, inner: bool) {
        self.0 = inner;
    }
}

#[derive(Clone, Copy, Component, Debug)]
pub struct HaveDashed(bool);

impl Deref for HaveDashed {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HaveDashed {
    pub fn set_inner(&mut self, inner: bool) {
        self.0 = inner;
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_size = 150.0;
    // plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(plane_size, 0.0, plane_size),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(plane_size).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground,
        ColliderDensity(2.5),
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
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.5, 2.75, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            ..default()
        },
        AtmosphereCamera::default(),
        MainCamera,
    ));
}

fn follow_camera(
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(mut camera) = camera.get_single_mut() {
        if let Ok(player) = player.get_single() {
            *camera = Transform::from_translation(player.translation + Vec3::new(2.5, 2.75, 4.0))
                .looking_at(player.translation, Vec3::Y);
        }
    }
}

type MFPlayer<'world, 'state, 'a> = Query<
    'world,
    'state,
    (
        &'a mut Transform,
        &'a IsOnGround,
        &'a MoveSpeed,
        &'a mut HaveDashed,
        Entity,
    ),
    (With<Player>, With<ControllableCube>),
>;

fn move_forward(
    mut commands: Commands,
    mut player: MFPlayer,
    input: Res<Input<KeyCode>>,
    timer: Res<Time>,
) {
    if input.pressed(KeyCode::W) {
        if let Ok((mut transform, is_on_ground, move_speed, mut have_dashed, entity)) =
            player.get_single_mut()
        {
            let next = transform.forward();
            if **is_on_ground {
                transform.translation += next * (*move_speed * timer.delta_seconds());
            } else if !**have_dashed {
                commands
                    .entity(entity)
                    .insert(ExternalImpulse::new(next * 2.0));
                have_dashed.set_inner(true);
            }
        }
    }
}

type HMControllableCubes<'world, 'state, 'a> = Query<
    'world,
    'state,
    (&'a mut Transform, &'a MoveSpeed),
    (With<ControllableCube>, With<Player>),
>;

fn handle_mouvement(
    mut controllable_cubes: HMControllableCubes,
    input: Res<Input<KeyCode>>,
    timer: Res<Time>,
) {
    if let Ok((mut transform, inner)) = controllable_cubes.get_single_mut() {
        if input.pressed(KeyCode::A) {
            transform.rotate_local_y(*inner * (-PI / 2.0) * timer.delta_seconds());
            //move_forward(&mut transform, inner, &timer);
        }
        if input.pressed(KeyCode::D) {
            transform.rotate_local_y(*inner * (PI / 2.0) * timer.delta_seconds());
            //move_forward(&mut transform, inner, &timer);
        }
        if input.pressed(KeyCode::S) {
            transform.rotate_local_y(*inner * PI * timer.delta_seconds());
            //move_forward(&mut transform, inner, &timer);
        }
    }
}

type HIGControllableCubes<'world, 'state, 'a> = Query<
    'world,
    'state,
    (
        &'a mut IsOnGround,
        &'a CollidingEntities,
        &'a mut HaveDashed,
    ),
    (With<Player>, With<ControllableCube>),
>;

fn handle_is_ground(
    mut controllable_cubes: HIGControllableCubes,
    grounds: Query<Entity, With<Ground>>,
) {
    if let Ok(ground) = grounds.get_single() {
        if let Ok((mut state, entities, mut have_dashed)) = controllable_cubes.get_single_mut() {
            if !**state && entities.iter().any(|e| e.eq(&ground)) {
                println!("setting is_on_ground");
                state.set_inner(true);
                have_dashed.set_inner(false);
            }
        }
    } else {
        println!("no ground found")
    }
}

type HJPlayer<'world, 'state, 'a> =
    Query<'world, 'state, (Entity, &'a IsOnGround), (With<ControllableCube>, With<Player>)>;

fn handle_jump(mut commands: Commands, player: HJPlayer, input: Res<Input<KeyCode>>) {
    if let Ok((entity, state)) = player.get_single() {
        if input.pressed(KeyCode::Space) && **state {
            commands
                .entity(entity)
                .insert(ExternalImpulse::new(Vec3::Y));
        }
    }
}

fn handle_reset(
    mut player: Query<&mut Transform, (With<Player>, Without<MainCamera>)>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::R) {
        if let Ok(mut player) = player.get_single_mut() {
            player.translation = Vec3::new(0.0, 0.5, 0.0);
            player.rotation = Quat::default();
        }
    }
}

pub struct A3dScenePlugin;

impl Plugin for A3dScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    handle_reset,
                    move_forward,
                    handle_mouvement,
                    handle_jump,
                    handle_is_ground,
                    follow_camera,
                ),
            );
    }
}
