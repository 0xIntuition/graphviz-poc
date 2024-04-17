use bevy::prelude::*;

#[derive(Component)]
pub struct Identifier {
    pub id: String,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Connection {
    pub id: String,
    pub from: Entity,
    pub to: Entity,
}
