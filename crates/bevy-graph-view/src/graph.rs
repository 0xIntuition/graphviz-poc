use crate::assets::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::utils::normalize;
use crate::utils::random_point_on_sphere_surface;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::PickableBundle;
use forceatlas2::{Layout, Settings};
use graph::page_rank::page_rank;
use graph::page_rank::PageRankConfig;
use graph::prelude::DirectedCsrGraph;
use graph::prelude::GraphBuilder;
use std::collections::HashMap;

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_labels.after(update_identifiers))
            .add_systems(Update, select_node)
            .add_systems(Update, select_edge)
            .add_systems(Update, update_connections_transforms)
            .add_systems(Update, update_identifiers.before(update_labels))
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
    identifier_query: Query<&Identifier, With<Identifier>>,
    conn_query: Query<&Connection, With<Connection>>,
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
    conn_query: Query<Entity, With<Connection>>,
    identifier_query: Query<Entity, With<Identifier>>,
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
                    Identifier {
                        id: node.id.clone(),
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
            if let Some((from, x1, y1, z1)) = node_coordinates.get(&edge.from).cloned() {
                if let Some((to, x2, y2, z2)) = node_coordinates.get(&edge.to).cloned() {
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
                            Connection {
                                id: edge.id.clone(),
                                from,
                                to,
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

fn update_layout(
    mut layout: Local<Option<Layout<f32, 3>>>,
    mut keys: Local<Vec<String>>,
    mut layout_type: Local<LayoutType>,
    mut cloned_ranks: Local<Vec<f32>>,
    configuration: Res<Configuration>,
    mut commands: Commands,
    mut ev: EventReader<LayoutEvent>,
    identifier_query: Query<(Entity, &Transform, &Identifier), With<Identifier>>,
    graph: Res<Graph>,
) {
    for settings in ev.read() {
        *layout_type = settings.layout_type;
        *keys = graph.edges.iter().fold(vec![], |mut acc, edge| {
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
                    keys.iter().position(|x| *x == edge.from).unwrap(),
                    keys.iter().position(|x| *x == edge.to).unwrap(),
                )
            })
            .collect();

        let directed_graph: DirectedCsrGraph<usize> =
            GraphBuilder::new().edges(edges.clone()).build();

        let (ranks, _, _) = page_rank(
            &directed_graph,
            PageRankConfig {
                max_iterations: 20,
                tolerance: 1.0E-4f64,
                damping_factor: 0.85,
            },
        );
        *cloned_ranks = ranks.clone();

        normalize(&mut cloned_ranks);

        // println!("nodes {:?}", keys);
        // println!("edges {:?}", edges);
        // println!("ranks {:?}", ranks);
        // println!("normal {:?}", cloned_ranks);
        let edge_count = edges.len();
        let weights = Some((0..edge_count).map(|_| 1.0).collect::<Vec<_>>());
        // let weights = Some(cloned_ranks.clone());

        let masses_sizes = cloned_ranks
            .iter()
            .map(|rank| (1.0, 1.0))
            .collect::<Vec<_>>();
        *layout = Some(Layout::<f32, 3>::from_graph_with_masses(
            edges,
            masses_sizes,
            weights,
            Settings {
                theta: 0.5,
                ka: settings.atlas_settings.ka,
                kr: settings.atlas_settings.kr,
                kg: settings.atlas_settings.kg,

                lin_log: false,
                prevent_overlapping: None,
                speed: settings.speed,
                strong_gravity: true,
            },
        ));
    }

    match *layout {
        Some(ref mut layout) => {
            for _ in 0..5 {
                layout.iteration();
            }
            let max_distance = layout
                .nodes
                .iter()
                .map(|node| {
                    (node.pos.x().powi(2) + node.pos.y().powi(2) + node.pos.z().powi(2)).sqrt()
                })
                .fold(0.0, f32::max);
            let scale = configuration.container_size / max_distance;

            for (index, node) in layout.nodes.iter().enumerate() {
                let x = node.pos.x() * scale;
                let y = node.pos.y() * scale;
                let z = match *layout_type {
                    LayoutType::Flat => 0.0,
                    LayoutType::Sphere => node.pos.z() * scale,
                };

                let id = keys[index].clone();
                if let Some((entity, transform, _)) = identifier_query
                    .iter()
                    .find(|(_, _, identifier)| identifier.id == id)
                {
                    commands.entity(entity).insert(
                        Transform::from_xyz(x, y, z).with_scale(Vec3::ONE),
                        // .with_scale(Vec3::ONE * cloned_ranks[index] * 1.1 + 0.5),
                    );
                }
            }
        }
        None => {}
    }
}

fn update_connections_transforms(
    mut conn_query: Query<(&mut Transform, &Connection), (With<Connection>, Without<Identifier>)>,
    id_query: Query<&Transform, (With<Identifier>, Without<Connection>)>,
) {
    for (mut transform, connection) in conn_query.iter_mut() {
        if let Ok(from_transform) = id_query.get(connection.from) {
            if let Ok(to_transform) = id_query.get(connection.to) {
                let mid_point = from_transform
                    .translation
                    .lerp(to_transform.translation, 0.5);
                let distance = from_transform
                    .translation
                    .distance(to_transform.translation);
                let rotation = Quat::from_rotation_arc(
                    Vec3::Y,
                    (to_transform.translation - from_transform.translation).normalize(),
                );

                *transform = Transform::from_xyz(mid_point.x, mid_point.y, mid_point.z)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(1.0, distance, 1.0));
            }
        }
    }
}
