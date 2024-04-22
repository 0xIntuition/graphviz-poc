mod assets;
mod components;
mod events;
mod graph;
mod resources;
mod utils;

use crate::resources::EdgeType;
use assets::AssetsPlugin;
use bevy::prelude::*;
use events::{AddGraphIdentifiers, EventsPlugin, LayoutEvent};
use graph::GraphPlugin;
use resources::{Edge, Graph, Node, ResourcesPlugin};

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
            .add_systems(Startup, add_test_data)
            .add_systems(Update, rotate_system);
    }
}

fn rotate_system(time: Res<Time>, mut query: Query<(&Group, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.1));
    }
}

fn add_test_data(
    mut graph: ResMut<Graph>,
    mut ev: EventWriter<AddGraphIdentifiers>,
    mut ev_l: EventWriter<LayoutEvent>,
) {
    const NODES: usize = 500;
    const EDGES: usize = 500;
    let nodes = graph.nodes.len();

    for i in 0..NODES {
        let index = nodes + i as usize;
        graph.nodes.push(Node {
            id: format!("did:example:{}", index),
            label: format!("Node {}", index),
            ..Default::default()
        });
    }

    for _ in 0..EDGES {
        let all_node_ids: Vec<String> = graph.nodes.iter().map(|node| node.id.clone()).collect();
        let from = all_node_ids
            .iter()
            .nth(rand::thread_rng().gen_range(0..all_node_ids.len()))
            .unwrap();
        let to = all_node_ids
            .iter()
            .filter(|id| id != &from)
            .nth(rand::thread_rng().gen_range(0..(all_node_ids.len() - 1)))
            .unwrap();
        graph.edges.push(Edge {
            id: format!("did:connection:{}", rand::thread_rng().gen_range(0..MAX)),
            from: from.clone(),
            to: to.clone(),
            edge_type: EdgeType::Unspecified,
        });
    }
    ev.send(AddGraphIdentifiers);
    ev_l.send(LayoutEvent {
        atlas_settings: Default::default(),
        speed: 0.6,
        // layout_type: events::LayoutType::Flat,
        layout_type: Default::default(),
        page_rank_config: Default::default(),
    });
}
