mod utils;

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy_graph_view::GraphViewPlugin;
use bevy_graph_view::events::AddGraphNodesEdges;
use bevy_graph_view::resources::{Node, Edge}; // Added import for Node and Edge
use bevy_panorbit_camera::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;
use bevy_graph_view::resources::EdgeType; // Added import for EdgeType

#[derive(Serialize, Deserialize, Clone)]
struct NodeWrapper {
    id: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EdgeWrapper {
    id: String,
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct GraphSettings {
    resolution: [f32; 2],
    initial_camera_location: [f32; 3],
    clear_color: [f32; 3],
}

#[wasm_bindgen]
pub fn show_graph(canvas_id: String, nodes_js: String, edges_js: String, settings_js: String) {
    utils::set_panic_hook();
    // Log input strings for debugging
    web_sys::console::log_1(&format!("Nodes: {}", nodes_js).into());
    web_sys::console::log_1(&format!("Edges: {}", edges_js).into());
    web_sys::console::log_1(&format!("Settings: {}", settings_js).into());

    // Parse JSON with error handling
    let nodes: Vec<NodeWrapper> = match serde_json::from_str(&nodes_js) {
        Ok(n) => n,
        Err(e) => {
            web_sys::console::error_1(&format!("Error parsing nodes: {}", e).into());
            return;
        }
    };

    let edges: Vec<EdgeWrapper> = match serde_json::from_str(&edges_js) {
        Ok(e) => e,
        Err(e) => {
            web_sys::console::error_1(&format!("Error parsing edges: {}", e).into());
            return;
        }
    };

    let settings: GraphSettings = match serde_json::from_str(&settings_js) {
        Ok(s) => s,
        Err(e) => {
            web_sys::console::error_1(&format!("Error parsing settings: {}", e).into());
            return;
        }
    };

    // Convert NodeWrapper and EdgeWrapper to Node and Edge
    let nodes: Vec<Node> = nodes.into_iter().map(|n| Node {
        id: n.id.clone(),
        label: n.id,
        image: None,
    }).collect();
    let edges: Vec<Edge> = edges.into_iter().map(|e| Edge {
        id: e.id.clone(),
        from: e.from.clone(),
        to: e.to.clone(),
        edge_type: EdgeType::Unspecified,
    }).collect();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (settings.resolution[0], settings.resolution[1]).into(),
            canvas: Some("#".to_owned() + &canvas_id),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(GraphViewPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, move |mut commands: Commands| {
            let initial_camera_location = Vec3::new(
                settings.initial_camera_location[0],
                settings.initial_camera_location[1],
                settings.initial_camera_location[2]
            );

            commands.spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        clear_color: ClearColorConfig::Custom(Color::rgb(
                            settings.clear_color[0],
                            settings.clear_color[1],
                            settings.clear_color[2]
                        )),
                        ..default()
                    },
                    tonemapping: Tonemapping::TonyMcMapface,
                    transform: Transform::from_translation(initial_camera_location)
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                PanOrbitCamera::default(),
            ));
        })
        .add_systems(Startup, move |mut ev: EventWriter<AddGraphNodesEdges>| {
            ev.send(AddGraphNodesEdges { nodes: nodes.clone(), edges: edges.clone() });
        });
    app.run();
}

