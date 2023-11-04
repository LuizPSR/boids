use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Factors {
    pub speed: f32,
    pub flock_radius: f32,
    pub separation_radius: f32,


    pub alignment: f32,
    pub cohesion: f32,
    pub separation: f32,

    pub goal_pos: Option<Vec2>,
    pub goal_weight: f32,
}

impl Default for Factors {
    fn default() -> Self {
        Self {
            speed: 100.,
            flock_radius: 100.,
            separation_radius: 30.,

            alignment: 20.,
            cohesion: 10.,
            separation: 30.,

            goal_pos: None,
            goal_weight: 40.,
        }
    }
}

#[derive(Debug, Component)]
struct Boid {
    pos: Vec2,
    dir: Vec2,
}

#[derive(Component)]
struct Flock {
    pos: Option<Vec2>,
    dir: Option<Vec2>,
    avoid_pos: Option<Vec2>,
}

fn spawn_boids(
    mut commands: Commands,
    number: f32,
    width: f32,
    height: f32
) {
    let mut rng = rand::thread_rng();

    for _ in 0..number {
        // get initial position and direction for boid
        let pos = Vec2::new(
            rng.gen_range(0.0 .. width),
            rng.gen_range(0.0 .. height),
        );

        let mut dir = Vec2::new(
            rng.gen_range(0.0 .. 1.0),
            rng.gen_range(0.0 .. 1.0),
        );
        // must have a direction
        while dir.normalize_or_zero() == Vec2::ZERO {
            dir = Vec2::new(
                rng.gen_range(0.0 .. 1.0),
                rng.gen_range(0.0 .. 1.0),
            );
        }

        // spawn
        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: pos.extend(0.),
                    rotation: Quat::from_rotation_arc(Vec3::Y, dir.extend(0.0)),
                    scale: Vec3::new(0.5, 1., 1.),
                },
                texture: asset_server.load("boid.png"),
                ..default()
            })
            .insert(Boid {
                pos: pos, 
                dir: dir,
            })
            .insert(Flock {
                pos: None,
                dir: None,
                avoid_pos: None,                
            });
    }
}

const INITIAL_POPULATIONS: u8 = 100;
fn spawn_boids_system(
    mut commands: Commands,
    win: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = win.get_single().unwrap();
    let height = window.height();
    let width = window.width();

    spawn_boids(commands, INITIAL_POPULATIONS, width, height);

    // spawn camera
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(width / 2., height / 2., 0.),
            ..default()
        });
}

fn respawn_boids_system(
    mut commands: Commands,
    win: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    boids: Query<Entity, With<Boid>>
) {
    if keys.just_released(KeyCode::R) {
        for e in boids.iter() {
            commands.entity(e).despawn();
        }

        let window = win.get_single().unwrap();
        let height = window.height();
        let width = window.width();
        
        spawn_boids(commands, INITIAL_POPULATIONS, width, height);
    }
}

fn vision_system(
    mut boids: Query<(&Boid, Entity)>,
    mut flocks: Query<(&mut Flock, Entity)>,
    factors: Res<Factors>,
) {
    // recalculate local flock of each boid
    for (mut flock, entity) in flocks.iter_mut() {

        let mut pos = Vec2::ZERO;
        let mut dir = Vec2::ZERO;
        let mut avoid_pos = Vec2::ZERO;

        let mut neighbor_count = 0;
        let mut close_neighbor_count = 0;
        
        for (_, b, e) in boids.iter() {
            if entity == e {
                continue;
            }

            let on_flock = pos.distance(b.pos) <= factors.flock_radius;
            let avoid = pos.distance(b.pos) < factors.separation_radius;

            if on_flock && !avoid {
                pos += b.pos;
                dir += b.dir;
                neighbor_count += 1;
            }

            if avoid {
                avoid_pos += b.pos;
                close_neighbor_count += 1;
            }
        }

        if neighbor_count < 1 {
            flock.pos = None;
            flock.dir = None;
        } else {
            flock.pos = Some(pos / neighbor_count as f32);
            flock.dir = Some(dir / neighbor_count as f32);
        }

        if close_neighbor_count < 1 {
            flock.avoid_pos = None;
        } else {
            flock.avoid_pos = Some(avoid_pos / close_neighbor_count as f32);
        }
    }
}

fn go_towards(
    &boid: Boid,
    pos: Option<Vec2>,
    weight: f32
) {
    if pos.is_some() {
        if let Some(dir) = (pos.unwrap() - boid.pos).try_normalize() {
            boid.dir = boid
                .dir
                .lerp(dir, strength)
        }
    }
}

fn go_away(
    &boid: Boid,
    pos: Option<Vec2>,
    weight: f32
) {
    if pos.is_some() {
        if let Some(dir) = (pos.unwrap() - boid.pos).try_normalize() {
            boid.dir = boid
                .dir
                .lerp(dir, strength)
        }
    }
}

fn movement_system(
    mut boids: Query<(&mut Transform, &mut Boid, Flock)>,
    mut flocks: Query<(&Flock)>,
    time: Res<Time>,
    factors: Res<Factors>,
) {
    // calculate boid current movement
    for (_, mut boid, flock) in boids.iter_mut() {

        go_towards(&boid, flock.pos, factors.cohesion * time.delta_seconds());
        go_towards(&boid, factors.goal_pos, factors.goal_weight * time.delta_seconds());
        go_away(&boid, flock.avoid_pos, factors.separation * time.delta_seconds())

        if let Some(dir) = flock.dir {
            let strength = factors.alignment * time.delta_seconds();
            boid.dir = boid
                .dir
                .lerp(dir, strength)
        }
    }

    // apply new direction and movement on transform
    for (mut trans, boid, _) in boids.iter_mut() {
        trans.rotation = Quat::from_rotation_arc(Vec3::Y, boid.dir.extend(0.0));
        let delta = boid.dir * factors.speed * time.delta_seconds();
        trans.translation += delta.extend(0.);
        //println!("Boid {:?}", boid);
        //println!("Treansform {:?}", trans);
    }
}

#[derive(Default)]
pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_boids_system);
        app.add_systems(Update, (
            vision_system,
            movement_system,
            respawn_boids_system
        ));
        app.insert_resource(Factors::default());
    }
}
