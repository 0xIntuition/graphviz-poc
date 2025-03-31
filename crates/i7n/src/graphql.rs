use bevy::prelude::*;
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContexts,
};
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_graph_view::{
    events::AddGraphNodesEdges,
    resources::{Edge, EdgeType, Node},
};
use bevy_mod_reqwest::*;
use primitive_types::U256;

pub struct GraphQLPlugin;

impl Plugin for GraphQLPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReqwestPlugin::default())
            .add_event::<GraphQLResponse>()
            .add_event::<AccountsResponse>()
            .init_resource::<GraphQLData>()
            .init_resource::<AddressInput>()
            .add_systems(Update, handle_graphql_response)
            .add_systems(Update, handle_accounts_response)
            .add_systems(Update, update_graph_data)
            .add_systems(Update, intuition_ui)
            .add_systems(Startup, refresh_accounts);
    }
}
// setup system
fn refresh_accounts(mut bevyreq: BevyReqwest) {
    fetch_accounts(bevyreq);
}

impl Default for GraphQLData {
    fn default() -> Self {
        Self {
            claims_from_following: Vec::new(),
        }
    }
}

#[derive(Resource)]
pub struct AddressInput {
    address: String,
    accounts: Vec<Account>,
}

impl Default for AddressInput {
    fn default() -> Self {
        Self {
            address: "0x19711cd19e609febdbf607960220898268b7e24b".to_string(),
            accounts: Vec::new(),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub claims_aggregate: Option<ClaimsAggregate>,
    #[serde(default)]
    pub signals_aggregate: Option<SignalsAggregate>,
    #[serde(default)]
    pub following: Option<ClaimsAggregate>,
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
pub struct ClaimsAggregate {
    pub aggregate: Aggregate,
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
pub struct SignalsAggregate {
    pub aggregate: Aggregate,
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
pub struct Aggregate {
    pub count: i32,
}

fn update_graph_data(
    mut events: EventReader<GraphQLResponse>,
    mut graph_data: ResMut<GraphQLData>,
) {
    for ev in events.read() {
        *graph_data = ev.data.clone();
    }
}

fn fetch_accounts(mut bevyreq: BevyReqwest) {
    let query = include_str!("get-accounts.graphql");
    let url: reqwest::Url = "https://prod.base.intuition-api.com/v1/graphql"
        .try_into()
        .unwrap();
    info!("sending graphql request to {} for accounts", url);
    let reqwest = bevyreq
        .client()
        .post(url)
        .json(&serde_json::json!({
            "query": query,
            "variables": {"offset": 0}
        }))
        .build()
        .unwrap();

    bevyreq.send(reqwest, On::send_event::<AccountsResponse>());
}

pub fn intuition_ui(
    mut egui_contexts: EguiContexts,
    bevyreq: BevyReqwest,
    bevyreq2: BevyReqwest,
    graph_data: Res<GraphQLData>,
    mut address_input: ResMut<AddressInput>,
) {
    let egui_context: &mut egui::Context = egui_contexts.ctx_mut();

    egui::Window::new("Intuition")
        .resizable(true)
        .show(egui_context, |ui| {
            let mut should_fetch_claims = false;
            let mut should_refresh_accounts = false;

            ui.horizontal(|ui| {
                ui.label("Account:");
                let accounts = address_input.accounts.clone();
                // find the account with the id that matches the address
                let selected_account = accounts
                    .iter()
                    .find(|account| account.id == address_input.address);
                
                let selected_text = match &selected_account {
                    Some(account) => account.label.clone(),
                    None => "Fetching accounts...".to_string(),
                };
                
                egui::ComboBox::from_label("")
                    .selected_text(&selected_text)
                    .show_ui(ui, |ui| {
                        for account in accounts {
                            let display_text = format!(
                                "{} ({:?} following, {:?} signals, {:?} claims)",
                                account.label,
                                account.following.as_ref().map_or(0, |s| s.aggregate.count),
                                account.signals_aggregate.as_ref().map_or(0, |s| s.aggregate.count),
                                account.claims_aggregate.as_ref().map_or(0, |s| s.aggregate.count),
                            );
                            ui.selectable_value(
                                &mut address_input.address,
                                account.id.clone(),
                                display_text,
                            );
                        }
                    });

                if ui.button("Fetch following claims").clicked() {
                    should_fetch_claims = true;
                }
            });



            let mut code = String::new();
            for claim in &graph_data.claims_from_following {
                let shares = U256::from_dec_str(&claim.shares).unwrap();
                let shares_str = format!("{:.5} ETH", shares.as_u128() as f64 / 1e18);
                code.push_str(&format!(
                    "{} \n/{}/  *{}*\n${} {}$\n\n",
                    claim.triple.subject.label,
                    claim.triple.predicate.label,
                    claim.triple.object.label,
                    claim.account.label,
                    shares_str,
                ));
            }

            ui.columns(1, |columns| {
                ScrollArea::vertical().show(&mut columns[0], |ui| {
                    crate::easy_mark::easy_mark(ui, &code);
                })
            });

            if should_fetch_claims && !address_input.address.is_empty() {
                send_graphql_request(bevyreq, address_input.address.clone());
            }

            if should_refresh_accounts {
                fetch_accounts(bevyreq2);
            }
        });
}

#[derive(serde::Deserialize, Debug, Event)]
struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(serde::Deserialize, Debug, Resource, Clone)]
pub struct GraphQLData {
    claims_from_following: Vec<Claim>,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct Claim {
    account: Account,
    triple: Triple,
    shares: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct Triple {
    id: String,
    subject: Atom,
    predicate: Atom,
    object: Atom,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct Atom {
    id: String,
    label: String,
}

impl From<ListenerInput<ReqResponse>> for GraphQLResponse {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        value.deserialize_json().unwrap()
    }
}

fn send_graphql_request(mut bevyreq: BevyReqwest, address: String) {
    let query = include_str!("claims-from-following.graphql");
    let url: reqwest::Url = "https://prod.base.intuition-api.com/v1/graphql"
        .try_into()
        .unwrap();
    info!("sending graphql request to {} for address {}", url, address);
    let reqwest = bevyreq
        .client()
        .post(url)
        .json(&serde_json::json!({
            "query": query,
            "variables": {"address": address}
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
                label: claim.triple.id.clone(),
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

#[derive(serde::Deserialize, Debug, Event)]
struct AccountsResponse {
    data: AccountsData,
}

#[derive(serde::Deserialize, Debug, Clone)]
struct AccountsData {
    accounts: Vec<Account>,
}

impl From<ListenerInput<ReqResponse>> for AccountsResponse {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        value.deserialize_json().unwrap()
    }
}

fn handle_accounts_response(
    mut events: EventReader<AccountsResponse>,
    mut address_input: ResMut<AddressInput>,
) {
    for ev in events.read() {
        address_input.accounts = ev.data.accounts.clone();
    }
}
