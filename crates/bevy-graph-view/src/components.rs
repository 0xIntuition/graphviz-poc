use bevy::prelude::*;

#[derive(Component)]
pub struct GraphNode {
    pub id: String,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct GraphEdge {
    pub id: String,
    pub source: Entity,
    pub target: Entity,
}
