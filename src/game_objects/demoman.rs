use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Query, Res},
        world::Mut,
    },
    math::{EulerRot, Quat, Vec3},
    pbr::{
        light_consts, AmbientLight, CascadeShadowConfigBuilder, DirectionalLight,
        DirectionalLightBundle,
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
pub struct Demoman {
    time_since_noise: f64,
    interval_to_noise: f64,
}

const VOICES: [&'static str; 5] = [
    "laugh.ogg",
    "drunk1.ogg",
    "drunk2.ogg",
    "drunk3.ogg",
    "freedom.ogg",
];

pub fn rotate_demoman(
    mut demo: Query<(&mut Demoman, &mut Transform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for v in demo.iter_mut() {
        let (mut demoman, mut transform): (Mut<Demoman>, Mut<Transform>) = v;
        transform.rotate_y(f32::to_radians(360.0) * 1.0 * time.delta_seconds_f64() as f32);
        demoman.time_since_noise += time.delta_seconds_f64();

        if demoman.time_since_noise >= demoman.interval_to_noise {
            let mut rng = rand::thread_rng();
            let directory = format!("audio/{}", VOICES[rng.gen_range(0..VOICES.len())]);
            audio.play(asset_server.load(directory));
            demoman.time_since_noise = 0.0;
            demoman.interval_to_noise = 6.0 + rng.gen::<f64>() * 10.0;
        }
    }
}

pub fn spawn_demoman(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audio/theme.ogg")).looped();
    let demoman = asset_server.load("models/demoman.glb#Scene0");

    commands.spawn((
        SceneBundle {
            scene: demoman,
            transform: Transform::from_xyz(0.0, 0.0125, -0.25),
            ..default()
        },
        Demoman {
            time_since_noise: 0.0,
            interval_to_noise: 0.0,
        },
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE.into(),
        brightness: 1.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT * 0.8,
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

    /*commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Demoman_Sprite.png"),
            ..default()
        },
        Demoman {
            time_since_noise: 0.0,
            interval_to_noise: 0.0,
        },
    ));*/
}
