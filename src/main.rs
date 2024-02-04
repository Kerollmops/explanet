use std::f32::consts::TAU;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use billboard::align_billboards_with_camera;
use planet::Planet;
use sun::{setup_sun, update_sun_settings, Sun, SunColor, SunMaterial};

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
        .insert_resource(SunColor { color: Color::rgb(0.592, 0.192, 0.0) })
        .add_systems(Startup, (setup_camera, setup_sun, setup_two_planets))
        .add_systems(
            Update,
            (
                update_sun_settings,
                align_billboards_with_camera,
                planet::update_planet_on_resolution_change,
                rotate_planets_around_sun,
                rotate_rotatables,
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
    commands.entity(planet_entity).insert(Rotatable { speed: 0.1 });

    Planet::with_resolution(
        &mut commands,
        meshes.as_mut(),
        materials.as_mut(),
        resolution,
        42,
        Color::ALICE_BLUE,
        Transform::from_xyz(0.2, 0.0, 10.6).with_scale(Vec3::splat(0.4)),
    );
    commands.entity(planet_entity).insert(Rotatable { speed: 0.05 });
}

fn rotate_planets_around_sun(
    time: Res<Time>,
    sun_q: Query<&Transform, With<Sun>>,
    mut planets_q: Query<&mut Transform, (With<Planet>, Without<Sun>)>,
) {
    let angle = time.elapsed_seconds() % TAU / 100.0;
    let sun = sun_q.get_single().unwrap().translation;
    for mut planet in planets_q.iter_mut() {
        let p = planet.translation;
        let x = angle.cos() * (p.x - sun.x) - angle.sin() * (p.z - sun.z) + sun.x;
        let z = angle.sin() * (p.x - sun.x) + angle.cos() * (p.z - sun.z) + sun.z;
        planet.translation = Vec3::new(x, p.y, z);
    }
}

// Define a component to designate a rotation speed to an entity.
#[derive(Component)]
pub struct Rotatable {
    pub speed: f32,
}

// This system will rotate any entity in the scene with a Rotatable component around its y-axis.
fn rotate_rotatables(mut rotatables: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, rotatable) in &mut rotatables {
        transform.rotate_y(rotatable.speed * TAU * timer.delta_seconds());
    }
}
