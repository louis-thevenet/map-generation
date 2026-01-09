use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_water::{WaterPlugin, WaterSettings};

use crate::{camera_plugin::CameraPlugin, terrain::TerrainPlugin};
mod camera_plugin;
mod terrain;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
        .insert_resource(WaterSettings {
            spawn_tiles: None,
            ..default()
        })
        .add_plugins(WaterPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup)
        .run();
}
fn setup(mut commands: Commands) {
    // Ambient light for base illumination
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.7, 0.75, 0.8),
        brightness: 150.0,
        affects_lightmapped_meshes: true,
    });

    // Main directional light (sun) with realistic illuminance
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 3_000.0,
            color: Color::srgb(1.0, 0.95, 0.9),
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Fill light from opposite direction for softer shadows
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: 800.0,
            color: Color::srgb(0.6, 0.7, 0.9),
            ..default()
        },
        Transform::from_xyz(-30.0, 60.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
