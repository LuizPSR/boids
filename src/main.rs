use bevy::prelude::{
    App,
    DefaultPlugins
};

mod boids;
mod ui;

use ui::UIPlugin;
use boids::BoidsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin)
        .add_plugins(BoidsPlugin)
        .run();
}
