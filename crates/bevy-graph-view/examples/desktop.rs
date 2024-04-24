use bevy::input::common_conditions::input_toggle_active;
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};
use bevy_panorbit_camera::*;

use bevy::time::common_conditions::on_timer;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_graph_view::events::{AddGraphNodesEdges, LayoutSettingsEvent};
use bevy_graph_view::resources::{Edge, EdgeType, Graph, Node};

use rand::Rng;
use std::{time::Duration, u32::MAX};
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_graph_view::GraphViewPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                add_test_data.run_if(on_timer(Duration::from_secs(2))),
                add_test_data2.run_if(on_timer(Duration::from_secs(3))),
                ui.run_if(input_toggle_active(false, KeyCode::KeyS)),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

fn add_test_data2(mut ev: EventWriter<AddGraphNodesEdges>, graph: Res<Graph>) {
    let nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    if !graph.nodes.is_empty() {
        for _ in 0..5 {
            let all_node_ids: Vec<String> =
                graph.nodes.iter().map(|node| node.id.clone()).collect();
            let from = all_node_ids
                .iter()
                .nth(rand::thread_rng().gen_range(0..all_node_ids.len()))
                .unwrap();
            let to = all_node_ids
                .iter()
                .filter(|id| id != &from)
                .nth(rand::thread_rng().gen_range(0..(all_node_ids.len() - 1)))
                .unwrap();
            edges.push(Edge {
                id: format!("did:connection:{}", rand::thread_rng().gen_range(0..MAX)),
                from: from.clone(),
                to: to.clone(),
                edge_type: EdgeType::Unspecified,
            });
        }
    }
    ev.send(AddGraphNodesEdges { nodes, edges });
}
fn add_test_data(mut ev: EventWriter<AddGraphNodesEdges>) {
    const NODES: usize = 10;
    const EDGES: usize = 10;
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    for _ in 0..NODES {
        let random_id = rand::thread_rng().gen_range(0..MAX);
        nodes.push(Node {
            id: format!("did:example:{}", random_id),
            label: format!("Node {}", random_id),
            ..Default::default()
        });
    }

    for _ in 0..EDGES {
        let all_node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let from = all_node_ids
            .iter()
            .nth(rand::thread_rng().gen_range(0..all_node_ids.len()))
            .unwrap();
        let to = all_node_ids
            .iter()
            .filter(|id| id != &from)
            .nth(rand::thread_rng().gen_range(0..(all_node_ids.len() - 1)))
            .unwrap();
        edges.push(Edge {
            id: format!("did:connection:{}", rand::thread_rng().gen_range(0..MAX)),
            from: from.clone(),
            to: to.clone(),
            edge_type: EdgeType::Unspecified,
        });
    }

    ev.send(AddGraphNodesEdges { nodes, edges });
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
            gravity: 0.1,
            attraction: 0.8,
            repulsion: 0.4,
            speed: 0.1,
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
