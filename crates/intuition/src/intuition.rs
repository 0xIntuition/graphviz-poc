use bevy::{
    log::{self},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_graph_view::{
    events::AddGraphNodesEdges,
    resources::{Edge, EdgeType, Node},
};
use bevy_mod_reqwest::*;
use egui_extras::install_image_loaders;
use reqwest::Url;

pub fn intuition_ui(mut egui_contexts: EguiContexts, bevyreq: BevyReqwest, bevyreq2: BevyReqwest) {
    let egui_context: &mut egui::Context = egui_contexts.ctx_mut();

    egui::Window::new("Intuition")
        .resizable(true)
        .show(egui_context, |ui| {
            if ui.button("Fetch identities").clicked() {
                send_identities_request(bevyreq);
            }
            if ui.button("Fetch claims").clicked() {
                send_claims_request(bevyreq2);
            }
        });
}

pub struct IntuitionPlugin;

impl Plugin for IntuitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReqwestPlugin::default())
            .add_event::<IdentitiesResponse>()
            .add_event::<ClaimsResponse>()
            .add_systems(Update, handle_identities_response)
            .add_systems(Update, handle_claims_response)
            .add_systems(Startup, configure_egui)
            .add_systems(Update, intuition_ui);
    }
}

pub fn configure_egui(mut egui_contexts: EguiContexts) {
    let egui_context: &mut egui::Context = egui_contexts.ctx_mut();
    install_image_loaders(egui_context);
}

#[derive(serde::Deserialize, Debug, Event)]
pub struct IdentitiesResponse {
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub data: Vec<IntuitionIdentity>,
}
#[derive(serde::Deserialize, Debug, Reflect, Clone)]
pub struct IntuitionIdentity {
    pub id: String,
    pub identity_id: String,
    pub identity_hash: String,
    pub display_name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_reference: Option<String>,
    pub creator: Creator,
    pub status: String,
    pub created_at: String,
    pub num_positions: u32,
    pub updated_at: String,
    pub vault_id: String,
    pub assets_sum: String,
    pub conviction_sum: String,
    pub conviction_price: String,
}
#[derive(serde::Deserialize, Debug, Reflect, Clone)]
pub struct Creator {
    pub id: String,
    pub wallet: String,
    pub total: u32,
}

#[derive(serde::Deserialize, Debug, Event)]
pub struct ClaimsResponse {
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub data: Vec<IntuitionClaim>,
}

#[derive(serde::Deserialize, Debug, Reflect, Clone)]
pub struct IntuitionClaim {
    pub claim_id: String,
    pub vault_id: String,
    pub counter_vault_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub creator: Creator,
    pub subject: IntuitionIdentity,
    pub object: IntuitionIdentity,
    pub predicate: IntuitionIdentity,
    pub status: String,
    pub user_conviction_for: String,
    pub user_conviction_against: String,
    pub num_positions: u32,
    pub for_num_positions: u32,
    pub for_assets_sum: String,
    pub assets_sum: String,
    pub for_conviction_sum: String,
    pub for_conviction_price: String,
    pub against_num_positions: u32,
    pub against_assets_sum: String,
    pub against_conviction_sum: String,
    pub against_conviction_price: String,
}

impl From<ListenerInput<ReqResponse>> for IdentitiesResponse {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        let s = value.deserialize_json().unwrap();
        s
    }
}
impl From<ListenerInput<ReqResponse>> for ClaimsResponse {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        let s = value.deserialize_json().unwrap();
        s
    }
}
pub fn send_identities_request(mut bevyreq: BevyReqwest) {
    log::info!("Sending identities request");
    let url: Url = "https://dev.api.intuition.systems/identities"
        .try_into()
        .unwrap();
    let reqwest = bevyreq
        .client()
        .get(url)
        // .header("x-api-key", INTUITION_API_KEY)
        .build()
        .unwrap();
    bevyreq.send(reqwest, On::send_event::<IdentitiesResponse>());
}

pub fn send_claims_request(mut bevyreq: BevyReqwest) {
    log::info!("Sending claims request");
    let url: Url = "https://dev.api.intuition.systems/claims"
        .try_into()
        .unwrap();
    let reqwest = bevyreq
        .client()
        .get(url)
        // .header("x-api-key", INTUITION_API_KEY)
        .build()
        .unwrap();
    bevyreq.send(reqwest, On::send_event::<ClaimsResponse>());
}
fn handle_identities_response(
    mut events: EventReader<IdentitiesResponse>,
    mut ev_g: EventWriter<AddGraphNodesEdges>,
) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for ev in events.read() {
        log::info!("got respoonse: {ev:?}");
        for identity in ev.data.iter() {
            nodes.push(Node {
                id: identity.identity_id.clone(),
                label: identity.display_name.clone(),
                image: identity.image.clone(),
                ..Default::default()
            });
            nodes.push(Node {
                id: identity.creator.wallet.clone(),
                label: identity.creator.wallet.clone(),
                ..Default::default()
            });
            edges.push(Edge {
                id: identity.id.clone(),
                from: identity.creator.wallet.clone(),
                to: identity.identity_id.clone(),
                edge_type: EdgeType::Named("created".to_string()),
            });
        }
        ev_g.send(AddGraphNodesEdges {
            nodes: nodes.clone(),
            edges: edges.clone(),
        });
    }
}

fn handle_claims_response(
    mut events: EventReader<ClaimsResponse>,
    mut ev_g: EventWriter<AddGraphNodesEdges>,
) {
    let mut edges = Vec::new();
    for ev in events.read() {
        log::info!("got respoonse: {ev:?}");
        for claim in ev.data.iter() {
            edges.push(Edge {
                id: claim.claim_id.clone(),
                from: claim.subject.identity_id.clone(),
                to: claim.object.identity_id.clone(),
                edge_type: EdgeType::Named(claim.predicate.display_name.clone()),
            });
        }
        ev_g.send(AddGraphNodesEdges {
            nodes: Vec::new(),
            edges: edges.clone(),
        });
    }
}
