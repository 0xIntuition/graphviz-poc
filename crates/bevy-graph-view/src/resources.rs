use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub selected_nodes: Vec<String>,
    pub selected_edges: Vec<String>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub image: Option<String>,
    // pub page_rank: u32,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Edge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
}

#[derive(Resource, Default, Reflect)]
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
            .register_type::<Graph>()
            .init_resource::<Configuration>()
            .register_type::<Configuration>();
    }
}
