mod assets;
mod game_objects;
mod player;

use crate::player::player::PlayerPlugin;
use bevy::app::{App, Startup, Update};
#[cfg(not(feature = "no_diagnostics"))]
use bevy::diagnostic::LogDiagnosticsPlugin;
#[cfg(not(feature = "no_diagnostics"))]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::default;
use bevy::prelude::*;
use bevy::window::{
    close_on_esc, CursorGrabMode, PresentMode, Window, WindowMode, WindowPlugin, WindowResolution,
    WindowTheme,
};
use bevy::DefaultPlugins;
use bevy_kira_audio::AudioPlugin;
use game_objects::spinner::{rotate_spinner, spawn_spinner};
use mouse_position::mouse_position::Mouse;

fn main() {
    #[cfg(debug_assertions)]
    println!("DEBUG BUILD");
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    transparent: true,
                    decorations: false,
                    resolution: WindowResolution::new(450.0, 800.0),
                    position: WindowPosition::new(IVec2::ZERO),
                    mode: WindowMode::Windowed,
                    window_level: bevy::window::WindowLevel::AlwaysOnTop,
                    title: "Demoman".into(), //TODO add code that grabs the name from a config file so we can bundle and characters into a generic spinner program and change the name based on who's spinning
                    name: Some("Demoman".into()),
                    present_mode: PresentMode::Fifo,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            #[cfg(not(feature = "no_diagnostics"))]
            LogDiagnosticsPlugin::default(),
            #[cfg(not(feature = "no_diagnostics"))]
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(AudioPlugin)
        .add_systems(Update, move_window)
        .add_systems(Update, close_on_esc)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_spinner)
        .add_systems(Update, rotate_spinner)
        .run()
}

fn move_window(mut windows: Query<&mut Window>, mouse: Res<ButtonInput<MouseButton>>) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        if window.cursor.grab_mode == CursorGrabMode::None {
            window.cursor.grab_mode = CursorGrabMode::Locked;
        } else {
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }

    if window.cursor.grab_mode == CursorGrabMode::Locked {
        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => {
                let position = IVec2::new(
                    x - (window.resolution.physical_width() / 2) as i32,
                    y - (window.resolution.physical_height() / 2) as i32,
                );
                window.position.set(position);
            }
            Mouse::Error => eprintln!("Send help"),
        }
    }
}
