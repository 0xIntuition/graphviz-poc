use bevy::prelude::*;
use forceatlas2::Settings;
use graph::page_rank::PageRankConfig;

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
pub struct LayoutEvent {
    pub atlas_settings: Settings<f32>,
    pub speed: f32,
    pub layout_type: LayoutType,
    pub page_rank_config: PageRankConfig,
}

pub struct LayoutSettings {
    pub gravity: f32,
    pub attraction: f32,
    pub repulsion: f32,
    pub speed: f32,
    pub layout_type: LayoutType,
}

#[derive(PartialEq, Copy, Clone, Default)]
pub enum LayoutType {
    Flat, // 2D
    #[default]
    Sphere, // 3D
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            gravity: 0.1,
            attraction: 1.0,
            repulsion: 0.01,
            speed: 0.1,
            layout_type: LayoutType::Sphere,
        }
    }
}

pub struct PageRankSettings {
    pub iterations: usize,
    pub tolerance: f64,
    pub damping: f32,
}

impl Default for PageRankSettings {
    fn default() -> Self {
        Self {
            iterations: 20,
            tolerance: 1.0E-4f64,
            damping: 0.8500f32,
        }
    }
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectNodeEvent>()
            .add_event::<SelectEdgeEvent>()
            .add_event::<LayoutEvent>()
            .add_event::<SelectIdentifierEvent>()
            .add_event::<AddGraphIdentifiers>();
    }
}
