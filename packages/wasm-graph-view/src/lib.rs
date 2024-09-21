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

#[wasm_bindgen]
pub fn show_graph(canvas_id: String, nodes_js: String, edges_js: String) {
    let nodes: Vec<NodeWrapper> = serde_json::from_str(&nodes_js).unwrap();
    let edges: Vec<EdgeWrapper> = serde_json::from_str(&edges_js).unwrap();
    
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
            resolution: (318., 200.).into(),
            canvas: Some("#".to_owned() + &canvas_id),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(GraphViewPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, |mut commands: Commands| {
            let initial_camera_location = Vec3::new(-2.0, 2.5, 9.0);

            commands.spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        clear_color: ClearColorConfig::Custom(Color::rgb(2.0 / 255.0, 6.0 / 255.0, 23.0 / 255.0)),
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

