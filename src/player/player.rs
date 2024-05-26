use bevy::app::{App, Plugin, Startup};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::prelude::{default, Commands, Component, Resource, Transform};
use bevy::render::camera::ClearColor;
use bevy::render::color::Color;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Resource)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::NONE));
        app.add_systems(Startup, setup_player);
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PlayerCamera,
    ));
    /*
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PlayerCamera,
    ));
    */
}
