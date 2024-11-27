use bevy::{
  prelude::*,
  window::{ PresentMode },
};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
              title: "Kinematic example".into(),
              name: Some("bevy.jcl.kinematic".into()),
              resolution: (1280., 720.).into(),
              present_mode: PresentMode::AutoVsync,
              ..default()
            }),
            ..default()
          }))
          .add_systems(Startup,
            append_camera
          )
        ;
    }
}

fn append_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}
