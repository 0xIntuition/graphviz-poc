mod assets;
mod components;
mod events;
mod graph;
mod resources;
mod utils;

use crate::resources::EdgeType;
use assets::{AssetsPlugin, MyAssets};
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, render::camera::ScalingMode};
use events::{AddGraphIdentifiers, EventsPlugin};
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
            .add_systems(Startup, (setup, add_test_data))
            .add_systems(Update, rotate_system);
    }
}

fn rotate_system(time: Res<Time>, mut query: Query<(&Group, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.1));
    }
}

fn setup(mut commands: Commands, my_assets: ResMut<MyAssets>) {
    const NODES: usize = 1000;

    commands
        .spawn((Group, SpatialBundle::default()))
        .with_children(|parent| {
            for _ in 0..NODES {
                let (x, y, z) = utils::random_point_on_sphere_surface(4.0);
                parent.spawn((MaterialMeshBundle {
                    mesh: my_assets.identifier_mesh_handle.clone(),
                    material: my_assets.identifier_material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, z),
                        ..Default::default()
                    },
                    ..Default::default()
                },));
            }
        });

    commands.spawn((Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        projection: OrthographicProjection {
            near: -500.0,
            far: 500.0,
            scale: 12.5,
            scaling_mode: ScalingMode::FixedVertical(0.8),
            ..default()
        }
        .into(),

        transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}

fn add_test_data(mut graph: ResMut<Graph>, mut ev: EventWriter<AddGraphIdentifiers>) {
    const NODES: usize = 100;
    const EDGES: usize = 100;
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
}
