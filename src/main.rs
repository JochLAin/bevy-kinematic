mod components;
mod resources;
mod plugins;

use crate::plugins::environment::*;
use crate::plugins::kinematic::bullet::*;
use crate::plugins::kinematic::basket_ball::*;
use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins((
      EnvironmentPlugin,
      BulletPlugin,
      BasketBallPlugin,
    ))
    .run()
  ;
}
