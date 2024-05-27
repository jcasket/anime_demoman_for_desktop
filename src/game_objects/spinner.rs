use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Query, Res},
        world::Mut,
    },
    math::{EulerRot, Quat, Vec3},
    pbr::{
        environment_map::EnvironmentMapLight, light_consts, AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle
    },
    reflect::Array,
    render::color::Color,
    scene::SceneBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
};
use bevy_kira_audio::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Spinner {
    time_since_noise: f64,
    interval_to_noise: f64,
    spin_speeds: Vec<f64>,
}

const VOICES: [&'static str; 5] = [
    "laugh.ogg",
    "drunk1.ogg",
    "drunk2.ogg",
    "drunk3.ogg",
    "freedom.ogg",
];

pub fn rotate_spinner(
    mut demo: Query<(&mut Spinner, &mut Transform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for v in demo.iter_mut() {
        let (mut spinner, mut transform): (Mut<Spinner>, Mut<Transform>) = v;
        transform.rotate_y(f32::to_radians(180.0) * 1.0 * time.delta_seconds_f64() as f32);
        spinner.time_since_noise += time.delta_seconds_f64();

        if spinner.time_since_noise >= spinner.interval_to_noise {
            let mut rng = rand::thread_rng();
            let directory = format!("audio/{}", VOICES[rng.gen_range(0..VOICES.len())]);
            #[cfg(not(feature = "mute"))]
            audio.play(asset_server.load(directory));
            spinner.time_since_noise = 0.0;
            spinner.interval_to_noise = 6.0 + rng.gen::<f64>() * 10.0;
        }
    }
}

pub fn spawn_spinner(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    #[cfg(not(feature = "mute"))]
    audio.play(asset_server.load("audio/theme.ogg")).looped();
    let spinner = asset_server.load("models/demoman.glb#Scene0");

    commands.spawn((
        SceneBundle {
            scene: spinner,
            transform: Transform::from_xyz(0.0, 0.0125, -0.25),
            ..default()
        },
        Spinner {
            time_since_noise: 0.0,
            interval_to_noise: 0.0,
            spin_speeds: vec![80.0,360.0],
        },
    ));

    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE.into(),
    //     brightness: 1.0,
    // });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT * 0.3,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 4.0, 2.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                f32::to_radians(0.0),
                f32::to_radians(30.0),
                f32::to_radians(-60.0),
            ),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

}
