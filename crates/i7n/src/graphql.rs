use bevy::prelude::*;
use bevy_graph_view::{events::AddGraphNodesEdges, resources::{Node, Edge, EdgeType}};
use reqwest::Client;
use serde_json::Value;

pub struct GraphQLPlugin;

impl Plugin for GraphQLPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GraphQLClient>()
            .add_systems(Startup, setup_graphql_client)
            .add_systems(Update, fetch_and_update_graph);
    }
}

#[derive(Resource)]
struct GraphQLClient(Client);

impl Default for GraphQLClient {
    fn default() -> Self {
        GraphQLClient(Client::new())
    }
}

fn setup_graphql_client(mut commands: Commands) {
    commands.insert_resource(GraphQLClient::default());
}

fn fetch_and_update_graph(
    client: Res<GraphQLClient>,
    mut ev_graph: EventWriter<AddGraphNodesEdges>,
) {
    let query = include_str!("get-signals.graphql");
    let url = "https://i7n.app/graphql";

    tokio::spawn(async move {
        let response = client.0.post(url)
            .json(&serde_json::json!({
                "query": query,
                "variables": {"after": null}
            }))
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        let signals = response["data"]["signals"]["items"].as_array().unwrap();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for signal in signals {
            let atom_id = signal["atom"]["id"].as_str().unwrap();
            let triple_id = signal["triple"]["id"].as_str().unwrap();
            let subject_id = signal["triple"]["subject"]["id"].as_str().unwrap();
            let predicate_id = signal["triple"]["predicate"]["id"].as_str().unwrap();
            let object_id = signal["triple"]["object"]["id"].as_str().unwrap();

            nodes.push(Node {
                id: atom_id.to_string(),
                label: format!("Atom {}", atom_id),
                image: None,
            });
            nodes.push(Node {
                id: triple_id.to_string(),
                label: format!("Triple {}", triple_id),
                image: None,
            });

            edges.push(Edge {
                id: format!("{}-{}", subject_id, predicate_id),
                from: subject_id.to_string(),
                to: predicate_id.to_string(),
                edge_type: EdgeType::Named("subject_predicate".to_string()),
            });
            edges.push(Edge {
                id: format!("{}-{}", predicate_id, object_id),
                from: predicate_id.to_string(),
                to: object_id.to_string(),
                edge_type: EdgeType::Named("predicate_object".to_string()),
            });
        }

        ev_graph.send(AddGraphNodesEdges { nodes, edges });
    });
}
