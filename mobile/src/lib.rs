use bevy::{
    input::touch::TouchPhase,
    log::{Level, LogPlugin},
    prelude::*,
    window::{PrimaryWindow, WindowMode},
    winit::WinitWindows,
};
use bevy_graph_view::GraphViewPlugin;
use objc::{
    msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use raw_window_handle::HasWindowHandle;
use raw_window_handle::RawWindowHandle;
use std::os::raw::c_void;

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: Level::ERROR,
                ..default()
            }),
    )
    .add_plugins(GraphViewPlugin)
    .add_systems(Update, (touch_camera, button_handler))
    .add_systems(PostStartup, (setup_scene,));

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

fn setup_scene(mut commands: Commands) {
    // Test ui
    commands
        .spawn(ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),
                right: Val::Px(50.0),
                bottom: Val::Px(50.0),
                ..default()
            },
            ..default()
        })
        .with_children(|b| {
            b.spawn(
                TextBundle::from_section(
                    "Test Button",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::BLACK,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}

fn button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    windows: NonSend<WinitWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::BLUE.into();
                let entity = window_query.single();
                let raw_window = windows.get_window(entity).unwrap();
                unsafe {
                    if let Ok(window_handle) = raw_window.window_handle() {
                        println!("Window handle: {:?}", window_handle);
                        match window_handle.as_raw() {
                            RawWindowHandle::UiKit(handle) => {
                                println!("UiKit handle: {:?}", handle);
                                if let Some(controller) = handle.ui_view_controller {
                                    let ui_view_controller: *mut Object =
                                        controller.as_ptr() as *mut Object;
                                    if let Some(custom_vc_class) =
                                        Class::get("HelloWorldViewController")
                                    {
                                        let hello_world_vc: *mut Object =
                                            msg_send![custom_vc_class, new];
                                        let () = msg_send![ui_view_controller, presentViewController: hello_world_vc animated: true completion: std::ptr::null_mut::<c_void>()];
                                    } else {
                                        println!("Failed to get class");
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                };
            }
            Interaction::Hovered => {
                *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}
