mod assets;
mod utils;

use assets::{AssetsPlugin, MyAssets};
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, render::camera::ScalingMode};
pub struct GraphViewPlugin;

#[derive(Component)]
struct Group;

impl Plugin for GraphViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetsPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, rotate_system);
    }
}

fn rotate_system(time: Res<Time>, mut query: Query<(&Group, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.1));
    }
}

fn setup(mut commands: Commands, my_assets: ResMut<MyAssets>) {
    const NODES: usize = 1000;

    commands
        .spawn((Group, SpatialBundle::default()))
        .with_children(|parent| {
            for _ in 0..NODES {
                let (x, y, z) = utils::random_point_on_sphere_surface(4.0);
                parent.spawn((MaterialMeshBundle {
                    mesh: my_assets.identifier_mesh_handle.clone(),
                    material: my_assets.identifier_material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, z),
                        ..Default::default()
                    },
                    ..Default::default()
                },));
            }
        });

    commands.spawn((Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        projection: OrthographicProjection {
            near: -500.0,
            far: 500.0,
            scale: 12.5,
            scaling_mode: ScalingMode::FixedVertical(0.8),
            ..default()
        }
        .into(),

        transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
