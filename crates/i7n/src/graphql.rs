use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_graph_view::{
    events::AddGraphNodesEdges,
    resources::{Edge, EdgeType, Node},
};
use bevy_mod_reqwest::*;

pub struct GraphQLPlugin;

impl Plugin for GraphQLPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReqwestPlugin::default())
            .add_event::<GraphQLResponse>()
            .add_systems(Update, handle_graphql_response)
            .add_systems(Update, intuition_ui);
    }
}

pub fn intuition_ui(mut egui_contexts: EguiContexts, bevyreq: BevyReqwest) {
    let egui_context: &mut egui::Context = egui_contexts.ctx_mut();

    egui::Window::new("Intuition")
        .resizable(true)
        .show(egui_context, |ui| {
            if ui.button("Fetch claims").clicked() {
                send_graphql_request(bevyreq);
            }
        });
}

#[derive(serde::Deserialize, Debug, Event)]
struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(serde::Deserialize, Debug)]
struct GraphQLData {
    claims_from_following: Vec<Claim>,
}

#[derive(serde::Deserialize, Debug)]
struct Claim {
    account: Account,
    triple: Triple,
}

#[derive(serde::Deserialize, Debug)]
struct Account {
    id: String,
    label: String,
}

#[derive(serde::Deserialize, Debug)]
struct Triple {
    id: String,
    label: String,
    subject: Atom,
    predicate: Atom,
    object: Atom,
}

#[derive(serde::Deserialize, Debug)]
struct Atom {
    id: String,
    label: String,
}

impl From<ListenerInput<ReqResponse>> for GraphQLResponse {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        value.deserialize_json().unwrap()
    }
}

fn send_graphql_request(mut bevyreq: BevyReqwest) {
    let query = include_str!("claims-from-following.graphql");
    let url: reqwest::Url = "https://api.i7n.app/v1/graphql".try_into().unwrap();
    info!("sending graphql request to {}", url);
    let reqwest = bevyreq
        .client()
        .post(url)
        .json(&serde_json::json!({
            "query": query,
            "variables": {"address": "0x19711cd19e609febdbf607960220898268b7e24b"}
            // 0x88d0af73508452c1a453356b3fac26525aec23a2
            // 0x19711cd19e609febdbf607960220898268b7e24b
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

        for claim in &ev.data.claims_from_following {
            // Account node
            nodes.push(Node {
                id: claim.account.id.clone(),
                label: claim.account.label.clone(),
                image: None,
            });

            // Triple node
            nodes.push(Node {
                id: claim.triple.id.clone(),
                label: claim.triple.label.clone(),
                image: None,
            });

            // Object node
            nodes.push(Node {
                id: claim.triple.object.id.clone(),
                label: claim.triple.object.label.clone(),
                image: None,
            });

            // Predicate node
            nodes.push(Node {
                id: claim.triple.predicate.id.clone(),
                label: claim.triple.predicate.label.clone(),
                image: None,
            });

            // Subject node
            nodes.push(Node {
                id: claim.triple.subject.id.clone(),
                label: claim.triple.subject.label.clone(),
                image: None,
            });

            // Subject-Predicate edge
            edges.push(Edge {
                id: format!("{}-{}", claim.triple.subject.id, claim.triple.predicate.id),
                from: claim.triple.subject.id.clone(),
                to: claim.triple.predicate.id.clone(),
                edge_type: EdgeType::Named("subject_predicate".to_string()),
            });

            // Predicate-Object edge
            edges.push(Edge {
                id: format!("{}-{}", claim.triple.predicate.id, claim.triple.object.id),
                from: claim.triple.predicate.id.clone(),
                to: claim.triple.object.id.clone(),
                edge_type: EdgeType::Named("predicate_object".to_string()),
            });

            // Account-Triple edge
            edges.push(Edge {
                id: format!("{}-{}", claim.account.id, claim.triple.id),
                from: claim.account.id.clone(),
                to: claim.triple.id.clone(),
                edge_type: EdgeType::Named("account_triple".to_string()),
            });
        }

        ev_graph.send(AddGraphNodesEdges { nodes, edges });
    }
}
