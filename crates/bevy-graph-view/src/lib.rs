mod assets;
mod components;
pub mod events;
mod graph;
pub mod resources;
mod utils;

use assets::AssetsPlugin;
use bevy::prelude::*;
use events::EventsPlugin;
use graph::GraphPlugin;
use resources::ResourcesPlugin;

pub struct GraphViewPlugin;

impl Plugin for GraphViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetsPlugin)
            .add_plugins(ResourcesPlugin)
            .add_plugins(EventsPlugin)
            .add_plugins(GraphPlugin);
    }
}
