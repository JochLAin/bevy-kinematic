use bevy::prelude::Component;

#[derive(Component)]
pub struct Kinematic {
  pub curent_velocity: f32,
  pub acceleration: f32,
}

impl Kinematic {
  pub fn new(initial_velocity: f32, acceleration: f32) -> Self {
    Kinematic {
      curent_velocity: initial_velocity,
      acceleration,
    }
  }
}

impl Default for Kinematic {
  fn default() -> Self {
    Kinematic {
      curent_velocity: 0.0,
      acceleration: 0.0,
    }
  }
}
