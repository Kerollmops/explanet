use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use sun::{update_settings, PostProcessPlugin, PostProcessSettings, SunPostProcessData};

mod planet;
mod sun;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PostProcessPlugin,
            ExtractResourcePlugin::<SunPostProcessData>::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::I)),
        ))
        .add_systems(Startup, (setup_camera, setup_sun, load_sun_resources))
        .add_systems(Update, (update_settings, close_on_esc))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        PostProcessSettings::default(),
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}

fn setup_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, ..default() })),
        material: materials.add(Color::rgb(0.9, 0.8, 0.7).into()),
        ..default()
    });
}

fn load_sun_resources(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(SunPostProcessData {
        image: server.load("images/abstract-bottle-glass.png"),
    });
}
