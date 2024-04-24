use crate::assets::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::utils::random_point_on_circle_surface;
use bevy::prelude::*;
use forceatlas2::Settings;
use std::collections::HashMap;

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_node)
            .add_systems(Update, select_edge)
            .add_systems(Update, update_graph_edge_transforms)
            // .add_systems(Update, update_identifiers)
            // .add_systems(Update, update_graph_node_transforms);
            .add_systems(
                Update,
                handle_add_graph_nodes_edges_event.before(update_layout),
            )
            .add_systems(Update, handle_layout_settings_event)
            .add_systems(
                Update,
                update_layout.after(handle_add_graph_nodes_edges_event),
            );
    }
}

pub fn select_node(mut graph: ResMut<Graph>, mut ev: EventReader<SelectNodeEvent>) {
    for event in ev.read() {
        let node_id = &event.0;
        if graph.selected_nodes.contains(node_id) {
            graph.selected_nodes.retain(|x| x != node_id);
        } else {
            graph.selected_nodes.push(node_id.clone());
        }
    }
}

pub fn select_edge(mut graph: ResMut<Graph>, mut ev: EventReader<SelectEdgeEvent>) {
    for event in ev.read() {
        let edge_id = &event.0;
        if graph.selected_edges.contains(edge_id) {
            graph.selected_edges.retain(|x| x != edge_id);
        } else {
            graph.selected_edges.push(edge_id.clone());
        }
    }
}

fn handle_layout_settings_event(
    mut ev: EventReader<LayoutSettingsEvent>,
    mut graph: ResMut<Graph>,
) {
    for settings in ev.read() {
        graph.layout.set_settings(Settings {
            theta: 0.5,
            ka: settings.attraction,
            kr: settings.repulsion,
            kg: settings.gravity,
            lin_log: false,
            prevent_overlapping: None,
            speed: settings.speed,
            strong_gravity: true,
        });
    }
}

fn handle_add_graph_nodes_edges_event(
    mut ev: EventReader<AddGraphNodesEdges>,
    mut ev2: EventWriter<AddGraphIdentifiers>,
    mut graph: ResMut<Graph>,
    mut commands: Commands,
    assets: Res<MyAssets>,
    query: Query<(Entity, &GraphNode), With<GraphNode>>,
) {
    for data in ev.read() {
        let mut node_entities: HashMap<String, Entity> = HashMap::new();
        let mut new_nodes: Vec<crate::resources::Node> = vec![];
        let mut new_edges: Vec<crate::resources::Edge> = vec![];
        let new_keys: Vec<String>;

        for node in &data.nodes {
            if !graph.nodes.iter().any(|x| x.id == node.id)
                && !new_nodes.iter().any(|x| x.id == node.id)
            {
                new_nodes.push(node.clone());
                let id = commands
                    .spawn((
                        MaterialMeshBundle {
                            mesh: assets.identifier_mesh_handle.clone(),
                            material: assets.identifier_material_handle.clone(),
                            ..Default::default()
                        },
                        GraphNode {
                            id: node.id.clone(),
                        },
                    ))
                    .id();
                node_entities.insert(node.id.clone(), id);
            }
        }
        for edge in &data.edges {
            if !graph.edges.iter().any(|x| x.id == edge.id)
                && !new_edges.iter().any(|x| x.id == edge.id)
            {
                new_edges.push(edge.clone());

                let local_source = node_entities.get(&edge.from).cloned();
                let local_target = node_entities.get(&edge.to).cloned();

                let source = local_source.or_else(|| {
                    query.iter().find_map(|(entity, identifier)| {
                        if identifier.id == edge.from {
                            Some(entity)
                        } else {
                            None
                        }
                    })
                });

                let target = local_target.or_else(|| {
                    query.iter().find_map(|(entity, identifier)| {
                        if identifier.id == edge.to {
                            Some(entity)
                        } else {
                            None
                        }
                    })
                });

                if let (Some(source), Some(target)) = (source, target) {
                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: assets.connection_mesh_handle.clone(),
                            material: assets.connection_material_handle.clone(),
                            ..Default::default()
                        },
                        GraphEdge {
                            id: edge.id.clone(),
                            source,
                            target,
                        },
                    ));
                }
            }
        }

        new_keys = new_edges.iter().fold(vec![], |mut acc, edge| {
            if !acc.contains(&edge.from) && !graph.keys.contains(&edge.from) {
                acc.push(edge.from.clone());
            }
            if !acc.contains(&edge.to) && !graph.keys.contains(&edge.to) {
                acc.push(edge.to.clone());
            }
            acc
        });

        graph.nodes.extend(new_nodes);
        graph.edges.extend(new_edges.clone());
        graph.keys.extend(new_keys.clone());

        let edges: Vec<((usize, usize), f32)> = new_edges
            .iter()
            .map(|edge| {
                (
                    (
                        graph.keys.iter().position(|x| *x == edge.from).unwrap(),
                        graph.keys.iter().position(|x| *x == edge.to).unwrap(),
                    ),
                    1.0,
                )
            })
            .collect();

        let nodes = new_keys
            .iter()
            .map(|_| forceatlas2::Node {
                pos: random_point_on_circle_surface(2.0),
                speed: forceatlas2::Vec2::new(0.0, 0.0),
                old_speed: forceatlas2::Vec2::new(0.0, 0.0),
                size: 1.0,
                mass: 1.0,
            })
            .collect::<Vec<_>>();

        graph.layout.add_nodes(&edges, &nodes);
        ev2.send(AddGraphIdentifiers);
    }
}

fn update_layout(
    configuration: Res<Configuration>,
    mut commands: Commands,
    query: Query<(Entity, &GraphNode), With<GraphNode>>,
    mut graph: ResMut<Graph>,
) {
    for _ in 0..5 {
        graph.layout.iteration();
    }
    let zero = forceatlas2::Vec2::new(0.0, 0.0);
    let max_distance = graph
        .layout
        .nodes
        .iter()
        .map(|node| node.pos.distance(zero))
        .fold(0.0, f32::max);
    let scale = configuration.container_size / max_distance;

    for (index, node) in graph.layout.nodes.iter().enumerate() {
        let scaled_pos = node.pos * scale;
        let x = scaled_pos.x();
        let y = scaled_pos.y();
        let z = 0.0;
        let id = graph.keys[index].clone();
        if let Some((entity, _)) = query.iter().find(|(_, identifier)| identifier.id == id) {
            commands
                .entity(entity)
                .insert(Transform::from_xyz(x, y, z).with_scale(Vec3::ONE));
        }
    }
}

fn update_graph_edge_transforms(
    mut edge_query: Query<(&mut Transform, &GraphEdge), (With<GraphEdge>, Without<GraphNode>)>,
    node_query: Query<&Transform, (With<GraphNode>, Without<GraphEdge>)>,
) {
    for (mut transform, graph_edge) in edge_query.iter_mut() {
        if let Ok(source_transform) = node_query.get(graph_edge.source) {
            if let Ok(target_transform) = node_query.get(graph_edge.target) {
                let mid_point = source_transform
                    .translation
                    .lerp(target_transform.translation, 0.5);
                let distance = source_transform
                    .translation
                    .distance(target_transform.translation);
                let rotation = Quat::from_rotation_arc(
                    Vec3::Y,
                    (target_transform.translation - source_transform.translation).normalize(),
                );

                *transform = Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(1.0, distance, 1.0));
            }
        }
    }
}
