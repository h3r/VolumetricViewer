mod debug;
mod volumetric;

use bevy::prelude::*;
use debug::DebugPlugin;
use volumetric::VolumetricPlugin;

fn main() {
    App::new()
    .add_plugins(DebugPlugin)
    .add_plugins(VolumetricPlugin)
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    println!("Volumetric Viewer");

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}