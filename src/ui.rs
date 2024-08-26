extern crate bevy;
extern crate bevy_egui;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, 
    prelude::*
};

use bevy_egui::{egui, EguiContexts};

use crate::boid::BoidFactors;

#[derive(Component)]
struct FpsText;

fn fps_text_setup (mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("BaskervvilleSC-Regular.ttf"),
                    font_size: 25.0,
                    ..default()
                },
            ),
            TextSection::new(
                "Value",
                TextStyle {
                    font: asset_server.load("BaskervvilleSC-Regular.ttf"),
                    font_size: 25.0,
                    ..default()
                },
            ),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));

    commands.insert_resource(BoidFactors {
        speed: 0.0,
        vision: 0.0,

        flocking: false,

        cohesion: 0.0,
        alignment: 0.0,
        separation: 0.0,
        personal_space: 0.0,

    });
}

fn fps_text_update_system(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}

fn settings_system (
    mut egui_context: EguiContexts,
    mut factors: ResMut<BoidFactors>
) {
    egui::Window::new("Settings")
    .anchor(egui::Align2::LEFT_BOTTOM, [-10.,-10.])
    .show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut factors.speed, 0.0..=250.0).text("Speed"));
        ui.add(egui::Slider::new(&mut factors.vision, 0.0..=100.0).text("Vision"));
        ui.add(egui::Checkbox::new(&mut factors.flocking, "Flocking"));

        let max_personal_space = factors.vision;
        ui.collapsing("Behavior", |ui| {
            ui.add(egui::Slider::new(&mut factors.alignment, 0.0..=50.0).text("Alignment"));
            ui.add(egui::Slider::new(&mut factors.cohesion, 0.0..=50.0).text("Cohesion"));
            ui.add(egui::Slider::new(&mut factors.separation, 0.0..=50.0).text("Separation"));
            ui.add(egui::Slider::new(&mut factors.personal_space, 0.0..=max_personal_space).text("Separation Distance"));
        });
    });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fps_text_setup);
        app.add_systems(Update, (fps_text_update_system, settings_system));
    }
}