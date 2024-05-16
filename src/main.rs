mod debug;
mod drag_n_drop;
mod volumetric;

use bevy::prelude::*;
use debug::DebugPlugin;
use drag_n_drop::DragNDropPlugin;
use volumetric::VolumetricPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)  
    .add_plugins(DebugPlugin)
    .add_plugins(DragNDropPlugin)
    .add_plugins(VolumetricPlugin)
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