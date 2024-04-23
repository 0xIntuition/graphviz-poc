mod assets;
mod components;
mod events;
mod graph;
mod resources;
mod utils;

use assets::AssetsPlugin;
use bevy::prelude::*;
use events::{AddGraphNodesEdges, EventsPlugin};
use graph::GraphPlugin;
use resources::{Edge, EdgeType, Node, ResourcesPlugin};

use rand::Rng;
use std::u32::MAX;

pub struct GraphViewPlugin;

#[derive(Component)]
struct Group;

impl Plugin for GraphViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetsPlugin)
            .add_plugins(ResourcesPlugin)
            .add_plugins(EventsPlugin)
            .add_plugins(GraphPlugin)
            .add_systems(Startup, add_test_data);
    }
}

fn add_test_data(mut ev: EventWriter<AddGraphNodesEdges>) {
    const NODES: usize = 900;
    const EDGES: usize = 900;
    let mut nodes: Vec<crate::resources::Node> = Vec::new();
    let mut edges: Vec<crate::resources::Edge> = Vec::new();

    for i in 0..NODES {
        nodes.push(Node {
            id: format!("did:example:{}", i),
            label: format!("Node {}", i),
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
