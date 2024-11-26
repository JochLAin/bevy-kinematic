use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{
  prelude::*,
  sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
struct Kinematic {
  curent_velocity: f32,
  initial_velocity: f32,
  acceleration: f32,
}

impl Kinematic {
  fn new(initial_velocity: f32, acceleration: f32) -> Self {
    Kinematic {
      curent_velocity: initial_velocity,
      initial_velocity,
      acceleration,
    }
  }
}

impl Default for Kinematic {
  fn default() -> Self {
    Kinematic {
      curent_velocity: 0.0,
      initial_velocity: 0.0,
      acceleration: 0.0,
    }
  }
}

fn main() {
  let mut app = App::new();
  app.add_plugins((
    DefaultPlugins,
    Wireframe2dPlugin,
  ));

  app.add_systems(Startup, setup);
  app.add_systems(FixedUpdate, translate);
  app.add_systems(Update, toggle_wireframe);
  app.run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    Kinematic::new(19.0, 3.0),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
      material: materials.add(Color::hsl(360.0, 0.95, 0.7)),
      transform: Transform::from_xyz( 80.0, 0.0, 0.0),
      ..default()
    },
  ));

  commands.spawn((
    Kinematic::new(1.0, 17.0),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
      material: materials.add(Color::hsl(180.0, 0.95, 0.7)),
      transform: Transform::from_xyz( 0.0, 0.0, 0.0),
      ..default()
    },
  ));
}

fn translate(mut query: Query<(&mut Transform, &mut Kinematic)>, time: Res<Time>) {
  for (mut transform, mut kinematic) in &mut query {
    kinematic.curent_velocity += kinematic.acceleration * time.delta_seconds();
    transform.translation.x += kinematic.curent_velocity * time.delta_seconds();
  }
}

fn toggle_wireframe(
  mut wireframe_config: ResMut<Wireframe2dConfig>,
  keyboard: Res<ButtonInput<KeyCode>>,
) {
  if keyboard.just_pressed(KeyCode::Space) {
    wireframe_config.global = !wireframe_config.global;
  }
}