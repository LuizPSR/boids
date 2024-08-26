extern crate bevy;
extern crate bevy_egui;
extern crate rand;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, 
    prelude::*
};
use bevy_egui::EguiPlugin;

mod ui;
mod boid;

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin, EguiPlugin))
        .add_plugins((boid::BoidPlugin, ui::UiPlugin))
        .add_systems(Startup, camera_setup)
        .run();
}
