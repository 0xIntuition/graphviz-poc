use bevy::prelude::*;

#[derive(Component)]
pub struct GraphNode {
    pub id: String,
    pub mass: f32,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct GraphEdge {
    pub id: String,
    pub source: Entity,
    pub target: Entity,
    pub strength: f32,
}
