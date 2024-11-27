use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::components::kinematic::*;

const GRAVITY: f32 = -18.0;
const MAX_HEIGHT: f32 = 25.0;
const TARGET_DISTANCE: f32 = 66.0;
const TARGET_HEIGHT: f32 = 9.0;

#[derive(Component)]
pub struct BasketBallExercise;

#[derive(Resource)]
pub struct BasketBallEndTimer(pub Timer);

pub struct BasketBallPlugin;

impl Plugin for BasketBallPlugin {
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
  let mut ball = KinematicObject::new(Kinematic {
    acceleration: Vec3::new(0.0, GRAVITY, 0.0),
    ..default()
  });

  let target = KinematicObject::new(Kinematic {
    displacement: Vec3::new(TARGET_DISTANCE, TARGET_HEIGHT, 0.0),
    ..default()
  });

  let time = ball.set_initial_velocity_for_arc_throw(
    target.kinematic.displacement,
    MAX_HEIGHT,
    GRAVITY
  );

  commands.spawn((
    BasketBallExercise,
    ball.clone(),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 8.0 })),
      material: materials.add(Color::hsl(0.0, 0.0, 1.0)),
      transform: Transform::from_translation(ball.kinematic.displacement),
      ..default()
    },
  ));

  commands.spawn((
    BasketBallExercise,
    target.clone(),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Annulus::new(9.0, 10.0))),
      material: materials.add(Color::hsl(0.0, 0.95, 0.7)),
      transform: Transform::from_translation(target.kinematic.displacement),
      ..default()
    },
  ));

  commands.insert_resource(BasketBallEndTimer(Timer::from_seconds(time, TimerMode::Once)));
}

fn update(mut query: Query<(&mut Transform, &mut KinematicObject), With<BasketBallExercise>>, time: Res<Time>, mut timer: ResMut<BasketBallEndTimer>) {
  if !timer.0.tick(time.delta()).finished() {
    for (mut transform, mut obj) in &mut query {
      let delta_time: f32 = time.delta_seconds();
      transform.translation += delta_time * obj.update_velocity(delta_time);
    }
  }
}
