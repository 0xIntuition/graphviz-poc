//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_graph_view::show_graph;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    // Create canvas element with id "viz"
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.set_attribute("id", "viz").unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();

    // Sample nodes
    let nodes = r#"[
        {"id": "1"},
        {"id": "2"},
        {"id": "3"}
    ]"#;

    // Sample edges
    let edges = r#"[
        {"id": "e1", "from": "1", "to": "2"},
        {"id": "e2", "from": "2", "to": "3"}
    ]"#;

    // Sample settings
    let settings = r#"{
        "resolution": [592,128],
        "initial_camera_location": [0, -7, 3],
        "clear_color": [0.00784313725490196,0.023529411764705882,0.09019607843137255]
    }"#;

    // Call show_graph with sample data
    show_graph("viz".to_string(), nodes.to_string(), edges.to_string(), settings.to_string());
}
