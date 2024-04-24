use bevy::prelude::*;
use forceatlas2::{Layout, Settings};

#[derive(Resource)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub keys: Vec<String>,
    pub selected_nodes: Vec<String>,
    pub selected_edges: Vec<String>,
    pub layout: Layout<f32, 2>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            keys: Vec::new(),
            selected_nodes: Vec::new(),
            selected_edges: Vec::new(),
            layout: Layout::<f32, 2>::empty(
                false,
                Settings {
                    theta: 0.5,
                    ka: 0.5,
                    kr: 0.1,
                    kg: 0.9,
                    lin_log: false,
                    prevent_overlapping: None,
                    speed: 0.05,
                    strong_gravity: true,
                },
            ),
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub image: Option<String>,
    // pub page_rank: u32,
}

#[derive(Resource, Default, Clone)]
pub struct Edge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
}

#[derive(Resource, Default, Clone)]
pub enum EdgeType {
    #[default]
    Unspecified,
    Named(String),
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct Configuration {
    pub container_size: f32,
    pub animation_duration: u64,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            container_size: 4.0,
            animation_duration: 2,
        }
    }
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Graph>()
            // .register_type::<Graph>()
            .init_resource::<Configuration>()
            .register_type::<Configuration>();
    }
}
