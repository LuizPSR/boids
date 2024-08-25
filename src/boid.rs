extern crate bevy;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BoidFactors {
    pub speed: f32,
    pub vision: f32,

    pub flocking: bool,

    pub cohesion: f32,
    pub separation: f32,
    pub alignment: f32,
    pub collision_avoidance: f32,
}

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoidFactors::default());
    }
}