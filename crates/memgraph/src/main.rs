mod memgraph;
use bevy::input::common_conditions::input_toggle_active;
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_graph_view::{events::LayoutSettingsEvent, GraphViewPlugin};
use bevy_panorbit_camera::*;
use memgraph::MemgraphPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(GraphViewPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(MemgraphPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (ui.run_if(input_toggle_active(false, KeyCode::KeyS)),),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let initial_camera_location = Vec3::new(-2.0, 2.5, 9.0);

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_translation(initial_camera_location)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

pub struct LayoutSettings {
    pub gravity: f32,
    pub attraction: f32,
    pub repulsion: f32,
    pub speed: f32,
}
impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            gravity: 0.6,
            attraction: 0.3,
            repulsion: 0.7,
            speed: 0.35,
        }
    }
}
fn ui(
    mut egui_contexts: EguiContexts,
    mut ev_layout: EventWriter<LayoutSettingsEvent>,
    mut layout_settings: Local<LayoutSettings>,
) {
    let egui_context: &mut egui::Context = egui_contexts.ctx_mut();

    egui::SidePanel::new(egui::panel::Side::Right, "Sidebar")
        .default_width(250.0)
        .resizable(true)
        .show(egui_context, |ui| {
            ui.vertical(|ui| {
                ui.label("Settings");
                ui.add(
                    egui::Slider::new(&mut layout_settings.gravity, 0.001..=1.0).text("Gravity"),
                );
                ui.add(
                    egui::Slider::new(&mut layout_settings.attraction, 0.001..=1.0)
                        .text("Attraction"),
                );
                ui.add(
                    egui::Slider::new(&mut layout_settings.repulsion, 0.001..=1.0)
                        .text("Repulsion"),
                );
                ui.add(egui::Slider::new(&mut layout_settings.speed, 0.001..=1.0).text("Speed"));
                if ui.button("Apply").clicked() {
                    ev_layout.send(LayoutSettingsEvent {
                        gravity: layout_settings.gravity,
                        attraction: layout_settings.attraction,
                        repulsion: layout_settings.repulsion,
                        speed: layout_settings.speed,
                    });
                }
            });
        });
}
