use bevy::{
    prelude::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) 
    {
        app.add_systems(Startup, print_startup);
        app.add_systems(Update, print_fps);
    }
}

fn print_startup() {
    println!("Volumetric Viewer");
}

fn print_fps(time: Res<Time>) {
    println!("\rfps: {:04}", time.delta_seconds());
}

