use bevy::{
  diagnostic::{ FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin },
  prelude::*,
  sprite::{ Wireframe2dPlugin },
  window::{ PresentMode, WindowTheme },
};
use bevy::core::FrameCount;
use bevy::sprite::Wireframe2dConfig;

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
              window_theme: Some(WindowTheme::Dark),
              visible: false,
              enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
              },
              ..default()
            }),
            ..default()
          }))
          .add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            Wireframe2dPlugin,
          ))
          .add_systems(Startup,
            append_camera
          )
          .add_systems(Update, (
            make_visible,
            toggle_vsync,
            toggle_wireframe,
          ))
        ;
    }
}

fn append_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
  if frames.0 == 3 {
    window.single_mut().visible = true;
  }
}

fn toggle_vsync(input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
  if input.just_pressed(KeyCode::KeyV) {
    let mut window = windows.single_mut();

    window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
      PresentMode::AutoNoVsync
    } else {
      PresentMode::AutoVsync
    };
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
