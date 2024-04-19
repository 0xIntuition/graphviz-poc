use bevy::prelude::*;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct MyAssets {
    pub identifier_mesh_handle: Handle<Mesh>,
    pub identifier_material_handle: Handle<StandardMaterial>,
    pub identifier_selected_material_handle: Handle<StandardMaterial>,
    pub identifier_connected_material_handle: Handle<StandardMaterial>,
    pub connection_mesh_handle: Handle<Mesh>,
    pub connection_material_handle: Handle<StandardMaterial>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyAssets>()
            .register_type::<MyAssets>()
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut my_assets: ResMut<MyAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<StandardMaterial>>,
) {
    my_assets.identifier_mesh_handle = meshes.add(Mesh::try_from(Sphere { radius: 0.02 }).unwrap());
    my_assets.identifier_connected_material_handle = color_materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.1, 0.9),
        ..Default::default()
    });
    my_assets.identifier_selected_material_handle = color_materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });
    my_assets.identifier_material_handle = color_materials.add(StandardMaterial {
        emissive: Color::rgb_linear(2300.0, 900.0, 300.0),
        // base_color: Color::RED,
        ..Default::default()
    });

    my_assets.connection_mesh_handle = meshes.add(Mesh::from(Cylinder {
        radius: 0.001,
        half_height: 0.5,
    }));
    my_assets.connection_material_handle = color_materials.add(StandardMaterial {
        emissive: Color::rgb_linear(900.0, 900.0, 900.0),
        ..Default::default()
    });
}
