extern crate bevy;
extern crate rand;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::{thread_rng, Rng};

#[derive(Resource)]
pub struct BoidFactors {
    pub speed: f32,
    pub vision: f32,

    pub flocking: bool,

    pub cohesion: f32,
    pub alignment: f32,
    pub separation: f32,
    pub personal_space: f32,
}

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Direction(Vec2);

fn spawn_boid(commands: &mut Commands, pos: Vec3, dir: Direction) {
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: pos,
                ..default()
            },
            ..default()
        }
    )
    .insert(Boid)
    .insert(dir);
}

fn initial_setup_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // insert factors
    commands.insert_resource(
        BoidFactors {
            speed: 20.,
            vision: 25.,
            flocking: true,
            cohesion: 15.,
            alignment: 15.,
            separation: 25.,
            personal_space: 10.,
        }
    );

    // create boids
    let window = window_query.get_single().unwrap();
    let max_h = window.height()/2.;
    let min_h = -window.height()/2.;
    let max_w = window.width()/2.;
    let min_w = -window.width()/2.;
    
    let mut rng = thread_rng();
    for _ in 0..4096 {
        let px = rng.gen_range(min_w..=max_w);
        let py = rng.gen_range(min_h..=max_h);

        let dx = rng.gen_range(-1. ..=1.);
        let dy = rng.gen_range(-1. ..=1.);

        spawn_boid(&mut commands, Vec3::new(px,py,0.0), Direction(Vec2::new(dx,dy).normalize()));
    }
}

fn flocking_behavior_system(
    factors: Res<BoidFactors>,
    mut boids: Query<(&Transform, &mut Direction), With<Boid>>
) {
    if !factors.flocking { return; }

    let copies: Vec<(Vec2, Vec2)> = boids.iter().map(|(t, d)| (t.translation.xy(), d.0)).collect();

    // for each boid
    boids.par_iter_mut().for_each(|(t, mut d)| {
        // find local flock center and alignment
        let point = t.translation.xy();
        let mut neighbors = 0;
        let mut neighbors_too_close = 0;
        let mut cohesion_p = Vec2::default();
        let mut alignment = Vec2::default();
        let mut separation_p = Vec2::default();
        for copy in copies.iter() {
            let d = point.distance(copy.0);
            if d < factors.vision {
                neighbors+=1;
                cohesion_p+=copy.0;
                alignment+=copy.1;
                if d < factors.personal_space { 
                    neighbors_too_close+=1;
                    separation_p+=copy.0;
                }
            }
        }

        // remove self and calculate new direction
        neighbors-=1;
        neighbors_too_close-=1;

        cohesion_p-=point;
        cohesion_p/=neighbors as f32;
        let cohesion = (cohesion_p - point).normalize_or_zero() * factors.cohesion; 

        alignment-=d.0;
        alignment/=neighbors as f32;
        alignment = alignment.normalize_or_zero() * factors.alignment;

        separation_p-=point;
        separation_p/=neighbors_too_close as f32;
        let separation = (point - separation_p).normalize_or_zero() * factors.separation; 

        // new direction
        d.0 = (d.0 * 100. + cohesion + alignment + separation).normalize_or(d.0);
    });
}

fn wrap_world_system(
    mut transforms: Query<&mut Transform, With<Boid>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();
    let max_h = window.height()/2.;
    let min_h = -window.height()/2.;
    let max_w = window.width()/2.;
    let min_w = -window.width()/2.;

    for mut t in transforms.iter_mut() {
        if t.translation.x < min_w { t.translation.x = max_w; }
        if t.translation.x > max_w { t.translation.x = min_w; }
        if t.translation.y < min_h { t.translation.y = max_h; }
        if t.translation.y > max_h { t.translation.y = min_h; }
    }
}

fn movement_system(
    factors: Res<BoidFactors>,
    timer: Res<Time>,
    mut boids: Query<(&mut Transform, &Direction), With<Boid>>,
) {
    for (mut t, d) in boids.iter_mut() {
        let trans = d.0 * factors.speed * timer.delta_seconds();
        t.translation.x += trans.x;
        t.translation.y += trans.y;
    }
}

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initial_setup_system);
        app.add_systems(Update, (movement_system, wrap_world_system, flocking_behavior_system));
    }
}