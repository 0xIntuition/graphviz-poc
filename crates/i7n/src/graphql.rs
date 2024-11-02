use bevy::prelude::*;
use bevy_graph_view::{events::AddGraphNodesEdges, resources::{Node, Edge, EdgeType}};
use bevy_mod_reqwest::*;
use bevy_eventlistener::callbacks::ListenerInput;

pub struct GraphQLPlugin;

impl Plugin for GraphQLPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReqwestPlugin::default())
            .add_event::<GraphQLResponse>()
            .add_systems(Update, handle_graphql_response)
            .add_systems(Startup, send_graphql_request);
    }
}

#[derive(serde::Deserialize, Debug, Event)]
struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(serde::Deserialize, Debug)]
struct GraphQLData {
    signals: Vec<Signal>,
}

#[derive(serde::Deserialize, Debug)]
struct Signal {
    atom: Option<Atom>,
    triple: Option<Triple>,
}

#[derive(serde::Deserialize, Debug)]
struct Atom {
    id: String,
}

#[derive(serde::Deserialize, Debug)]
struct Triple {
    id: String,
    subject: Entity,
    predicate: Entity,
    object: Entity,
}

#[derive(serde::Deserialize, Debug)]
struct Entity {
    id: String,
}

impl From<ListenerInput<ReqResponse>> for GraphQLResponse {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        value.deserialize_json().unwrap()
    }
}

fn send_graphql_request(mut bevyreq: BevyReqwest) {
    let query = include_str!("get-signals.graphql");
    let url: reqwest::Url = "https://api.i7n.app/v1/graphql".try_into().unwrap();
    info!("sending graphql request to {}", url);
    let reqwest = bevyreq
        .client()
        .post(url)
        .json(&serde_json::json!({
            "query": query,
            "variables": {}
        }))
        .build()
        .unwrap();
    
    bevyreq.send(reqwest, On::send_event::<GraphQLResponse>());
}

fn handle_graphql_response(
    mut events: EventReader<GraphQLResponse>,
    mut ev_graph: EventWriter<AddGraphNodesEdges>,
) {
    for ev in events.read() {
    info!("got graphql response");
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for signal in &ev.data.signals {
            // Check if atom is present
            if let Some(ref atom) = signal.atom {
                nodes.push(Node {
                    id: atom.id.clone(),
                    label: format!("Atom {}", atom.id),
                    image: None,
                });
            }

            // Check if triple is present
            if let Some(ref triple) = signal.triple {
                nodes.push(Node {
                    id: triple.id.clone(),
                    label: format!("Triple {}", triple.id),
                    image: None,
                });

                edges.push(Edge {
                    id: format!("{}-{}", triple.subject.id, triple.predicate.id),
                    from: triple.subject.id.clone(),
                    to: triple.predicate.id.clone(),
                    edge_type: EdgeType::Named("subject_predicate".to_string()),
                });
                edges.push(Edge {
                    id: format!("{}-{}", triple.predicate.id, triple.object.id),
                    from: triple.predicate.id.clone(),
                    to: triple.object.id.clone(),
                    edge_type: EdgeType::Named("predicate_object".to_string()),
                });
            }
        }

        ev_graph.send(AddGraphNodesEdges { nodes, edges });
    }
}
