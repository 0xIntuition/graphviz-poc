use bevy::{input::touch::TouchPhase, prelude::*, window::WindowMode};
use bevy_graph_view::GraphViewPlugin;

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(GraphViewPlugin)
    .add_systems(Update, (touch_camera,));

    // MSAA makes some Android devices panic, this is under investigation
    // https://github.com/bevyengine/bevy/issues/8229
    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();
}

fn touch_camera(
    windows: Query<&Window>,
    mut touches: EventReader<TouchInput>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    mut last_position: Local<Option<Vec2>>,
) {
    let window = windows.single();

    for touch in touches.read() {
        if touch.phase == TouchPhase::Started {
            *last_position = None;
        }
        if let Some(last_position) = *last_position {
            let mut transform = camera.single_mut();
            *transform = Transform::from_xyz(
                transform.translation.x
                    + (touch.position.x - last_position.x) / window.width() * 5.0,
                transform.translation.y,
                transform.translation.z
                    + (touch.position.y - last_position.y) / window.height() * 5.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Y);
        }
        *last_position = Some(touch.position);
    }
}
