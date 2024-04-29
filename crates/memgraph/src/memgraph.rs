use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_graph_view::{
    events::AddGraphNodesEdges,
    resources::{Edge, EdgeType, Node},
};
use rsmgclient::{ConnectParams, Connection, ConnectionStatus, SSLMode, Value};

#[derive(Event)]
pub struct FetchDataEvent;

pub struct MemgraphPlugin;

impl Plugin for MemgraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FetchDataEvent>()
            .add_systems(Update, handle_fetch_data_event)
            .add_systems(Update, memgraph_ui)
            .add_systems(Startup, setup_connection);
    }
}

pub fn setup_connection(world: &mut World) {
    let connect_params = ConnectParams {
        host: Some(String::from("localhost")),
        port: 7687,
        sslmode: SSLMode::Disable,
        ..Default::default()
    };
    let connection = Connection::connect(&connect_params).unwrap();

    let status = connection.status();

    if status != ConnectionStatus::Ready {
        println!("Connection failed with status: {:?}", status);
        return;
    }
    world.insert_non_send_resource(connection);
}

pub fn memgraph_ui(mut egui_contexts: EguiContexts, mut ev: EventWriter<FetchDataEvent>) {
    let egui_context: &mut egui::Context = egui_contexts.ctx_mut();

    egui::Window::new("Memgraph")
        .resizable(true)
        .show(egui_context, |ui| {
            if ui.button("Fetch").clicked() {
                ev.send(FetchDataEvent);
            }
        });
}

fn handle_fetch_data_event(
    mut events: EventReader<FetchDataEvent>,
    mut ev_g: EventWriter<AddGraphNodesEdges>,
    mut connection: NonSendMut<Connection>,
) {
    for _ev in events.read() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let _ = connection.execute("MATCH (n)-[r]->(m) RETURN n, r, m;", None);

        while let Ok(result) = connection.fetchall() {
            for record in result {
                for value in record.values {
                    match value {
                        Value::Node(node) => {
                            nodes.push(Node {
                                id: node.id.to_string(),
                                label: node.labels.join(", "),
                                image: None,
                            });
                        }
                        Value::Relationship(edge) => {
                            edges.push(Edge {
                                id: edge.id.to_string(),
                                from: edge.start_id.to_string(),
                                to: edge.end_id.to_string(),
                                edge_type: EdgeType::Named(edge.type_),
                            });
                        }
                        value => println!("Value: {}", value),
                    }
                }
            }

            println!();
        }

        ev_g.send(AddGraphNodesEdges {
            nodes: nodes.clone(),
            edges: edges.clone(),
        });
    }
}
