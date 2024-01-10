use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use planet::{update_planet_on_resolution_change, Planet, Resolution, Seeded};

mod planet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            InfiniteGridPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::I)),
        ))
        .register_type::<Resolution>()
        .register_type::<Seeded>()
        .add_systems(Startup, (setup_single_planet, setup_camera_and_light, setup_grid))
        .add_systems(Update, (update_planet_on_resolution_change, close_on_esc))
        .run();
}

fn setup_camera_and_light(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight { intensity: 1500.0, shadows_enabled: true, ..default() },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn setup_grid(mut commands: Commands) {
    commands.spawn(InfiniteGridBundle::default());
}

fn setup_single_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let resolution = 100;
    let color = Color::ANTIQUE_WHITE;
    Planet::with_resolution(
        &mut commands,
        meshes.as_mut(),
        materials.as_mut(),
        resolution,
        42,
        color,
    );
}
