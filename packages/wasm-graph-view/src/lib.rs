mod utils;

use bevy::prelude::*;
use bevy_graph_view::GraphViewPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn show_graph(canvas_id: String) {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some(canvas_id),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(GraphViewPlugin);
    app.run();
}
