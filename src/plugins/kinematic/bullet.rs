use bevy::prelude::*;
use bevy::sprite::{ MaterialMesh2dBundle, Mesh2dHandle };
use crate::components::kinematic::*;
use crate::resources::*;

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
  let bullet = KinematicObject::new(Kinematic {
    initial_velocity: Vec3::new(INITIAL_VELOCITY_B, 0.0, 0.0),
    acceleration: Vec3::new(ACCELERATION_B, 0.0, 0.0),
    displacement: Vec3::new(0.0, OFFSET_Y, 0.1),
    ..default()
  });

  let target = KinematicObject::new(Kinematic {
    initial_velocity: Vec3::new(INITIAL_VELOCITY_A, 0.0, 0.0),
    acceleration: Vec3::new(ACCELERATION_A, 0.0, 0.0),
    displacement: Vec3::new(DISPLACEMENT, OFFSET_Y, 0.0),
    ..default()
  });

  commands.spawn((
    bullet.clone(),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
      material: materials.add(Color::hsl(0.0, 0.0, 1.0)),
      transform: Transform::from_translation(bullet.kinematic.displacement),
      ..default()
    },
  ));

  commands.spawn((
    target.clone(),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
      material: materials.add(Color::hsl(0.0, 0.95, 0.7)),
      transform: Transform::from_translation(target.kinematic.displacement),
      ..default()
    },
  ));

  let collision_time: f32 = bullet.kinematic.get_collision_time(target.kinematic);
  commands.insert_resource(EndTimer(Timer::from_seconds(collision_time, TimerMode::Once)));
  println!("The bullet will be perfectly aligned hit the target in {}sec", collision_time);
}

fn update(mut query: Query<(&mut Transform, &mut KinematicObject)>, time: Res<Time>, mut timer: ResMut<EndTimer>) {
  if !timer.0.tick(time.delta()).finished() {
    for (mut transform, mut obj) in &mut query {
      let delta_time: f32 = time.delta_seconds();
      transform.translation += delta_time * obj.update_velocity(delta_time);
    }
  }
}
