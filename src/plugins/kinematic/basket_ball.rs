use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
// use crate::components::Kinematic;
// use crate::resources::EndTimer;

// const GRAVITY: f32 = -18.0;
// const MAX_HEIGHT: f32 = 35.0;
const TARGET_DISTANCE: f32 = 66.0;
const TARGET_HEIGHT: f32 = 9.0;

pub struct KinematicBasketBallPlugin;

impl Plugin for KinematicBasketBallPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, startup)
    ;
  }
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn((
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 8.0 })),
      material: materials.add(Color::hsl(0.0, 0.0, 1.0)),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    },
  ));

  commands.spawn((
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Annulus::new(9.0, 10.0))),
      material: materials.add(Color::hsl(0.0, 0.95, 0.7)),
      transform: Transform::from_xyz(TARGET_DISTANCE, TARGET_HEIGHT, 0.0),
      ..default()
    },
  ));
}
