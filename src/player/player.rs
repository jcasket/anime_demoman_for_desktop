use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::Res;
use bevy::pbr::environment_map::EnvironmentMapLight;
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

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>,) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("textures/chromaticKTX2/diffuse.ktx2"),
            specular_map: asset_server.load("textures/chromaticKTX2/specular.ktx2"),
            intensity: 1500.0,
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
