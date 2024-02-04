use std::f32::consts::TAU;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use billboard::align_billboards_with_camera;
use planet::Planet;
use sun::{setup_sun, update_sun_settings, SunColor, SunMaterial};

mod billboard;
mod planet;
mod sun;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<SunMaterial>::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::I)),
        ))
        .register_type::<SunColor>()
        .register_type::<Planet>()
        .insert_resource(SunColor { color: Color::rgb(0.75, 0.26, 0.03) })
        .add_systems(Startup, (setup_camera, setup_sun, setup_two_planets))
        .add_systems(
            Update,
            (
                update_sun_settings,
                align_billboards_with_camera,
                planet::update_planet_on_resolution_change,
                rotate_center_rotates,
                rotate_rotate_arounds,
                close_on_esc,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        camera: Camera { hdr: true, ..default() },
        transform: Transform::from_xyz(-6.0, 2.0, 11.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_two_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let resolution = 300;
    let planet_entity = Planet::with_resolution(
        &mut commands,
        meshes.as_mut(),
        materials.as_mut(),
        resolution,
        42,
        Color::ANTIQUE_WHITE,
        Transform::from_xyz(-1.0, 0.0, -5.0).with_scale(Vec3::splat(0.3)),
    );
    commands.entity(planet_entity).insert((
        CenterRotate { speed: 0.5, axis: Vec3::new(0.1, 0.5, 0.0) },
        RotateAround { speed: 0.05, center: Vec3::splat(0.0) },
    ));

    let planet_entity = Planet::with_resolution(
        &mut commands,
        meshes.as_mut(),
        materials.as_mut(),
        resolution,
        42,
        Color::ALICE_BLUE,
        Transform::from_xyz(0.2, 0.0, 10.6).with_scale(Vec3::splat(0.4)),
    );
    commands.entity(planet_entity).insert((
        CenterRotate { speed: 0.4, axis: Vec3::new(0.01, 0.5, 0.0) },
        RotateAround { speed: 0.1, center: Vec3::splat(0.0) },
    ));
}

// Define a component to designate a rotation speed to an entity.
#[derive(Component)]
pub struct CenterRotate {
    pub speed: f32,
    pub axis: Vec3,
}

// This system will rotate any entity in the scene with a Rotatable component around its y-axis.
fn rotate_center_rotates(mut rotates: Query<(&mut Transform, &CenterRotate)>, timer: Res<Time>) {
    for (mut transform, rotates) in &mut rotates {
        let radian = rotates.speed * TAU * timer.delta_seconds();
        transform.rotate(Quat::from_scaled_axis(rotates.axis * radian));
    }
}

#[derive(Component)]
pub struct RotateAround {
    pub speed: f32,
    pub center: Vec3,
}

fn rotate_rotate_arounds(mut rotates: Query<(&mut Transform, &RotateAround)>, timer: Res<Time>) {
    for (mut transform, rotates) in &mut rotates {
        transform.rotate_around(
            rotates.center,
            Quat::from_rotation_y(rotates.speed * TAU * timer.delta_seconds()),
        );
    }
}
