use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::components::Kinematic;
use crate::resources::EndTimer;

const OFFSET_Y: f32 = -100.0;

const INITIAL_VELOCITY_A: f32 = 19.0;
const ACCELERATION_A: f32 = 3.0;
const INITIAL_VELOCITY_B: f32 = 1.0;
const ACCELERATION_B: f32 = 17.0;
const DISPLACEMENT: f32 = 80.0;

pub struct KinematicBulletPlugin;

impl Plugin for KinematicBulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)
      .add_systems(FixedUpdate, update)
    ;
  }
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn((
    Kinematic::new(INITIAL_VELOCITY_A, ACCELERATION_A),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
      material: materials.add(Color::hsl(0.0, 0.0, 1.0)),
      transform: Transform::from_xyz(0.0, OFFSET_Y, 0.0),
      ..default()
    },
  ));

  commands.spawn((
    Kinematic::new(INITIAL_VELOCITY_B, ACCELERATION_B),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
      material: materials.add(Color::hsl(0.0, 0.0, 1.0)),
      transform: Transform::from_xyz(-DISPLACEMENT, OFFSET_Y, 0.1),
      ..default()
    },
  ));

  let a: f32 = ACCELERATION_B - ACCELERATION_A;
  let b: f32 = 2.0 * (INITIAL_VELOCITY_B - INITIAL_VELOCITY_A);
  let c: f32 = -2.0 * DISPLACEMENT;
  let e: f32 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);

  let timer = EndTimer(Timer::from_seconds(e, TimerMode::Once));
  commands.insert_resource(timer);
}

fn update(mut query: Query<(&mut Transform, &mut Kinematic)>, time: Res<Time>, mut timer: ResMut<EndTimer>) {
  if !timer.0.tick(time.delta()).finished() {
    for (mut transform, mut kinematic) in &mut query {
      kinematic.curent_velocity += kinematic.acceleration * time.delta_seconds();
      transform.translation.x += kinematic.curent_velocity * time.delta_seconds();
    }
  }
}
