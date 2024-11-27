mod components;
mod resources;
mod plugins;

use crate::plugins::environment::EnvironmentPlugin;
use crate::plugins::kinematic::bullet::*;
use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins((
      EnvironmentPlugin,
      KinematicBulletPlugin,
    ))
    .run()
  ;
}
