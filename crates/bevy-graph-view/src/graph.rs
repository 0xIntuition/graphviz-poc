use crate::assets::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::utils::random_point_on_circle_surface;
use crate::utils::random_point_on_sphere_surface;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::PickableBundle;
use forceatlas2::Settings;
use std::collections::HashMap;

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_labels.after(update_identifiers))
            .add_systems(Update, select_node)
            .add_systems(Update, select_edge)
            .add_systems(Update, update_graph_edge_transforms)
            .add_systems(Update, update_identifiers.before(update_labels))
            // .add_systems(Update, update_graph_node_transforms);
            .add_systems(Update, handle_add_graph_nodes_edges_event)
            .add_systems(Update, handle_layout_settings_event)
            .add_systems(Update, update_layout);
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
#[derive(Component)]
struct ExampleLabel {
    entity: Entity,
}
fn update_labels(
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
    mut commands: Commands,
    mut labels: Query<(Entity, &mut Style, &ExampleLabel), With<ExampleLabel>>,
    labelled: Query<&GlobalTransform>,
    graph: Res<Graph>,
    identifier_query: Query<&GraphNode, With<GraphNode>>,
    conn_query: Query<&GraphEdge, With<GraphEdge>>,
) {
    let (camera, _, camera_global_transform) = camera.single_mut();
    for (entity, mut style, label) in &mut labels {
        if let Ok(labelled_component) = labelled.get(label.entity) {
            let world_position = labelled_component.translation();

            let viewport_position =
                camera.world_to_viewport(camera_global_transform, world_position);

            match viewport_position {
                Some(viewport_position) => {
                    style.top = Val::Px(viewport_position.y);
                    style.left = Val::Px(viewport_position.x);
                }
                None => {
                    // style.top = Val::Px(0.0);
                    // style.left = Val::Px(0.0);
                }
            }

            let identifier = identifier_query.get(label.entity);
            match identifier {
                Ok(identifier) => {
                    if graph.selected_nodes.contains(&identifier.id) {
                        commands.entity(entity).insert(Visibility::Visible);
                    } else {
                        commands.entity(entity).insert(Visibility::Hidden);
                    }
                }
                Err(_) => {}
            }

            let connection = conn_query.get(label.entity);
            match connection {
                Ok(connection) => {
                    if graph.selected_edges.contains(&connection.id) {
                        commands.entity(entity).insert(Visibility::Visible);
                    } else {
                        commands.entity(entity).insert(Visibility::Hidden);
                    }
                }
                Err(_) => {}
            }
        }
    }
}

fn update_identifiers(
    mut commands: Commands,
    mut ev: EventReader<AddGraphIdentifiers>,
    configuration: Res<Configuration>,
    my_assets: ResMut<MyAssets>,
    graph: Res<Graph>,
    conn_query: Query<Entity, With<GraphEdge>>,
    identifier_query: Query<Entity, With<GraphNode>>,
    label_query: Query<Entity, With<ExampleLabel>>,
) {
    let label_text_style = TextStyle {
        ..Default::default()
    };
    for _ in ev.read() {
        for entity in conn_query.iter() {
            commands.entity(entity).despawn();
        }

        for entity in identifier_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in label_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let mut node_coordinates: HashMap<String, (Entity, f32, f32, f32)> = HashMap::new();
        for node in &graph.nodes {
            let (x, y, z) = random_point_on_sphere_surface(configuration.container_size);
            let entity = commands
                .spawn((
                    MaterialMeshBundle {
                        mesh: my_assets.identifier_mesh_handle.clone(),
                        material: my_assets.identifier_material_handle.clone(),
                        transform: Transform::from_xyz(x, y, z)
                            .with_scale(Vec3::new(0.5, 0.5, 0.5)),

                        ..Default::default()
                    },
                    GraphNode {
                        id: node.id.clone(),
                        mass: 1.0,
                    },
                    PickableBundle::default(),
                    On::<Pointer<Click>>::run(
                        |event: Listener<Pointer<Click>>,
                         mut ev: EventWriter<SelectIdentifierEvent>| {
                            info!("The pointer clicked entity {:?}", event.target);
                            ev.send(SelectIdentifierEvent(event.target));
                        },
                    ),
                ))
                .id();
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    },
                    ExampleLabel { entity },
                ))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(node.label.clone(), label_text_style.clone())
                            .with_style(Style {
                                position_type: PositionType::Absolute,
                                bottom: Val::ZERO,
                                ..default()
                            })
                            .with_no_wrap(),
                    );
                });

            node_coordinates.insert(node.id.clone(), (entity, x, y, z));
        }

        for edge in &graph.edges {
            if let Some((source, x1, y1, z1)) = node_coordinates.get(&edge.from).cloned() {
                if let Some((target, x2, y2, z2)) = node_coordinates.get(&edge.to).cloned() {
                    let transform1 = Transform::from_xyz(x1, y1, z1);
                    let transform2 = Transform::from_xyz(x2, y2, z2);

                    let mid_point = transform1.translation.lerp(transform2.translation, 0.5);
                    let distance = transform1.translation.distance(transform2.translation);
                    let rotation = Quat::from_rotation_arc(
                        Vec3::Y,
                        (transform2.translation - transform1.translation).normalize(),
                    );

                    let entity = commands
                        .spawn((
                            MaterialMeshBundle {
                                mesh: my_assets.connection_mesh_handle.clone(),
                                material: my_assets.connection_material_handle.clone(),
                                visibility: Visibility::Visible,
                                transform: Transform::from_xyz(
                                    mid_point.x,
                                    mid_point.y,
                                    mid_point.z,
                                )
                                .with_rotation(rotation)
                                .with_scale(Vec3::new(1.0, distance, 1.0)),

                                ..Default::default()
                            },
                            GraphEdge {
                                id: edge.id.clone(),
                                source,
                                target,
                                strength: 1.0,
                            },
                        ))
                        .id();
                    commands
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                ..default()
                            },
                            ExampleLabel { entity },
                        ))
                        .with_children(|parent| {
                            let label = match &edge.edge_type {
                                EdgeType::Unspecified => format!("Connection"),
                                EdgeType::Named(name) => name.clone(),
                            };
                            parent.spawn(
                                TextBundle::from_section(label, label_text_style.clone())
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        bottom: Val::ZERO,
                                        ..default()
                                    })
                                    .with_no_wrap(),
                            );
                        });
                }
            }
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
) {
    for data in ev.read() {
        for node in &data.nodes {
            if !graph.nodes.iter().any(|x| x.id == node.id) {
                graph.nodes.push(node.clone());
            }
        }
        for edge in &data.edges {
            if !graph.edges.iter().any(|x| x.id == edge.id) {
                graph.edges.push(edge.clone());
            }
        }
        graph.keys = graph.edges.iter().fold(vec![], |mut acc, edge| {
            if !acc.contains(&edge.from) {
                acc.push(edge.from.clone());
            }
            if !acc.contains(&edge.to) {
                acc.push(edge.to.clone());
            }
            acc
        });
        let edges: Vec<(usize, usize)> = graph
            .edges
            .iter()
            .map(|edge| {
                (
                    graph.keys.iter().position(|x| *x == edge.from).unwrap(),
                    graph.keys.iter().position(|x| *x == edge.to).unwrap(),
                )
            })
            .collect();

        let nodes = graph
            .keys
            .iter()
            .map(|_| forceatlas2::Node {
                pos: random_point_on_circle_surface(2.0),
                speed: forceatlas2::Vec2::new(0.0, 0.0),
                old_speed: forceatlas2::Vec2::new(0.0, 0.0),
                size: 1.0,
                mass: 1.0,
            })
            .collect::<Vec<_>>();
        graph.layout.add_nodes(&edges, &nodes, None);
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
