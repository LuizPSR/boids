use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_egui::{
    egui::{self, Align2, Slider},
    EguiContexts, EguiPlugin,
};

use super::boids::Factors;

#[derive(Component)]
struct FPSText;

fn fps_text_setup(
    mut commands: Commands
) {
    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(1000.0),
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            ..default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FPSText);
}

fn fps_text_update_system(
    time: Res<Time>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", 1. / time.delta_seconds());
    }
}

fn set_goal_system(
    win: Query<&Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut factors: ResMut<Factors>,
) {
    // set goal
    if buttons.just_released(MouseButton::Right) {
        let window = win.get_single().unwrap();
        
        if let Some(mouse_pos) = window.cursor_position() {
            factors.goal_pos = Some(mouse_pos);
            //println!("{}", mouse_pos)
        }
    }

    // remove goal
    if keys.just_released(KeyCode::Escape) {
        factors.goal_pos = None;
    }
}

fn edit_factors_system(
    mut egui_context: EguiContexts,
    mut factors: ResMut<Factors>,
) {
    egui::Window::new("Factors")
        .anchor(Align2::LEFT_TOP, [-10.0, -10.0])
        .vscroll(true)
        .show(egui_context.ctx_mut(), |ui| {

            ui.collapsing("Boids Stats", |ui| {
                ui.add(Slider::new(&mut factors.speed, 0.0..=200.0).text("Speed"));
                ui.add(Slider::new(&mut factors.flock_radius, 20.0..=300.0).text("Flocking Distance"));
                ui.add(Slider::new(&mut factors.separation_radius, 5.0..=75.0).text("Separation Distance"));
            });
            
            

            ui.collapsing("Flocking Behavior", |ui| {
                ui.add(Slider::new(&mut factors.alignment, 0.0..=100.0).text("Alignment"));
                ui.add(Slider::new(&mut factors.cohesion, 0.0..=100.0).text("Cohesion"));
                ui.add(Slider::new(&mut factors.separation, 0.0..=100.0).text("Separation"));
            });

            ui.collapsing("Goal", |ui| {
                ui.label("Left click on the screen to select a goal that the boids will try to achieve. \nEsc will deselect the goal\n");
                if factors.goal_pos.is_some() {
                    ui.label(format!("Current goal: {}", factors.goal_pos.unwrap()));
                } else {
                    ui.label("Current goal: None");
                }
                
                ui.add(Slider::new(&mut factors.goal_weight, 0.0..=100.0).text("Weight"));
            });
        });
}

#[derive(Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Startup, fps_text_setup);
        app.add_systems(
            Update,
            (
                edit_factors_system, 
                set_goal_system,
                fps_text_update_system
            )
        );
    }
}
