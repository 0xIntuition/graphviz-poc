mod assets;
mod components;
mod events;
mod graph;
mod resources;
mod utils;

use assets::AssetsPlugin;
use bevy::{
    prelude::*,
    time::common_conditions::{on_timer, once_after_delay},
};
use events::{AddGraphNodesEdges, EventsPlugin};
use graph::GraphPlugin;
use resources::{Edge, EdgeType, Node, ResourcesPlugin};

use rand::Rng;
use std::{time::Duration, u32::MAX};

pub struct GraphViewPlugin;

#[derive(Component)]
struct Group;

impl Plugin for GraphViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetsPlugin)
            .add_plugins(ResourcesPlugin)
            .add_plugins(EventsPlugin)
            .add_plugins(GraphPlugin)
            .add_systems(
                Update,
                (
                    add_test_data.run_if(on_timer(Duration::from_secs(2))),
                    add_test_data2.run_if(on_timer(Duration::from_secs(3))),
                ),
            );
        // .add_systems(
        //     Update,
        //     (
        //         add_test_data.run_if(once_after_delay(Duration::from_secs(1))),
        //         add_test_data2.run_if(once_after_delay(Duration::from_secs(3))),
        //     ),
        // );
    }
}
fn add_test_data2(mut ev: EventWriter<AddGraphNodesEdges>, graph: Res<crate::resources::Graph>) {
    const NODES: usize = 10;
    const EDGES: usize = 10;
    let mut nodes: Vec<crate::resources::Node> = Vec::new();
    let mut edges: Vec<crate::resources::Edge> = Vec::new();

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
fn add_test_data(mut ev: EventWriter<AddGraphNodesEdges>, graph: Res<crate::resources::Graph>) {
    const NODES: usize = 10;
    const EDGES: usize = 10;
    let mut nodes: Vec<crate::resources::Node> = Vec::new();
    let mut edges: Vec<crate::resources::Edge> = Vec::new();

    for i in 0..NODES {
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
