use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct SelectNodeEvent(pub String);

#[derive(Event, Debug)]
pub struct SelectEdgeEvent(pub String);

#[derive(Event, Debug)]
pub struct AddGraphIdentifiers;

#[derive(Event, Debug)]
pub struct SelectIdentifierEvent(pub Entity);

#[derive(Event, Debug)]
pub struct DeselectIdentifierEvent;

#[derive(Event)]
pub struct LayoutSettingsEvent {
    pub gravity: f32,
    pub attraction: f32,
    pub repulsion: f32,
    pub speed: f32,
}

#[derive(Event)]
pub struct AddGraphNodesEdges {
    pub nodes: Vec<crate::resources::Node>,
    pub edges: Vec<crate::resources::Edge>,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectNodeEvent>()
            .add_event::<SelectEdgeEvent>()
            .add_event::<LayoutSettingsEvent>()
            .add_event::<SelectIdentifierEvent>()
            .add_event::<AddGraphNodesEdges>()
            .add_event::<AddGraphIdentifiers>();
    }
}
