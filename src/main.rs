use bevy::sprite::{ MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin };
use bevy::prelude::*;

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

pub struct KinematicPlugin;
impl Plugin for KinematicPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(FixedUpdate, translate);
  }
}

#[derive(Resource)]
struct EndTimer(Timer);

fn main() {
  let mut app = App::new();
  app.add_plugins((
    DefaultPlugins,
    KinematicPlugin,
    Wireframe2dPlugin,
  ));

  app.add_systems(Update, toggle_wireframe);
  app.run();
}
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(Camera2dBundle::default());

  const START_A: f32 = 80.0;
  const INITIAL_VELOCITY_A: f32 = 19.0;
  const ACCELERATION_A: f32 = 3.0;

  const START_B: f32 = 0.0;
  const INITIAL_VELOCITY_B: f32 = 1.0;
  const ACCELERATION_B: f32 = 17.0;

  commands.spawn((
    Kinematic::new(INITIAL_VELOCITY_A, ACCELERATION_A),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
      material: materials.add(Color::hsl(0.0, 0.0, 1.0)),
      transform: Transform::from_xyz(START_A, 0.0, 0.0),
      ..default()
    },
  ));

  commands.spawn((
    Kinematic::new(INITIAL_VELOCITY_B, ACCELERATION_B),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
      material: materials.add(Color::hsl(0.0, 0.0, 1.0)),
      transform: Transform::from_xyz(START_B, 0.0, 0.1),
      ..default()
    },
  ));

  let h: f32 = (START_B - START_A).abs();
  let a: f32 = ACCELERATION_B - ACCELERATION_A;
  let b: f32 = 2.0 * (INITIAL_VELOCITY_B - INITIAL_VELOCITY_A);
  let c: f32 = -2.0 * h;
  let e: f32 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);

  commands.insert_resource(EndTimer(Timer::from_seconds(e, TimerMode::Once)));
}

fn translate(mut query: Query<(&mut Transform, &mut Kinematic)>, time: Res<Time>, mut timer: ResMut<EndTimer>) {
  if !timer.0.tick(time.delta()).finished() {
    for (mut transform, mut kinematic) in &mut query {
      kinematic.curent_velocity += kinematic.acceleration * time.delta_seconds();
      transform.translation.x += kinematic.curent_velocity * time.delta_seconds();
    }
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
