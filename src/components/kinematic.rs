use bevy::prelude::*;

/**
 * Kinematic equations :
 ** s : displacement
 ** u : initial velocity
 ** v : final velocity
 ** a : acceleration
 ** t : time
 *
 * s = (u + v) / 2 * t
 * v = u + at
 * s = ut + (at^2) / 2
 * s = vt - (at^2) / 2
 * v^2 = u^2 + 2as
 */

#[derive(Component, Clone)]
pub struct Kinematic {
  pub displacement: Vec3,
  pub initial_velocity: Vec3,
  pub final_velocity: Vec3,
  pub acceleration: Vec3,
  pub time: f32,
}

impl Kinematic {
  pub fn get_collision_time(&self, target: Kinematic) -> f32 {
    let diff_a: Vec3 = self.acceleration - target.acceleration;
    let a: f32 = diff_a.length() * diff_a.dot(Vec3::ONE).signum();

    let diff_b: Vec3 = self.initial_velocity - target.initial_velocity;
    let b: f32 = 2.0 * diff_b.length() * diff_b.dot(Vec3::ONE).signum();

    let diff_c: Vec3 = self.displacement - target.displacement;
    let c: f32 = -2.0 * diff_c.length();

    println!("a: {}, b: {}, c: {}", a, b, c);

    (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)
  }
}

impl Default for Kinematic {
  fn default() -> Self {
    Kinematic {
      displacement: Vec3::ZERO,
      initial_velocity: Vec3::ZERO,
      final_velocity: Vec3::ZERO,
      acceleration: Vec3::ZERO,
      time: 0.0,
    }
  }
}

#[derive(Component, Clone)]
pub struct KinematicObject {
  pub current_velocity: Vec3,
  pub kinematic: Kinematic,
}

impl KinematicObject {
  pub fn new(kinematic: Kinematic) -> Self {
    KinematicObject {
      current_velocity: kinematic.initial_velocity,
      kinematic,
    }
  }

  pub fn update_velocity(&mut self, time_delta: f32) -> Vec3 {
    let kinematic = &self.kinematic.clone();
    self.current_velocity += kinematic.acceleration * time_delta;
    self.current_velocity
  }
}

impl Default for KinematicObject {
  fn default() -> Self {
    KinematicObject {
      current_velocity: Vec3::ZERO,
      kinematic: Kinematic::default(),
    }
  }
}
