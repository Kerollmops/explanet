use std::f32::consts::FRAC_PI_2;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod planet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<SunMaterial>::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::I)),
        ))
        .add_systems(Startup, (setup_camera, setup_sun))
        .add_systems(
            Update,
            (
                look_at_the_sun,
                update_sun_settings,
                align_sun_plane_with_camera,
                planet::update_planet_on_resolution_change,
                close_on_esc,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        camera: Camera { hdr: true, ..default() },
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}

fn setup_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sunmaterials: ResMut<Assets<SunMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load Texture
    let material = sunmaterials.add(SunMaterial {
        base_texture: asset_server.load("textures/abstract-bottle-glass.png"),
        settings: SunSettings { aspect: 1.0, ..default() },
    });

    // plane
    commands.spawn((
        SunPlane,
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, ..default() })),
            material,
            ..default()
        },
    ));
}

pub fn look_at_the_sun(
    input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<SunPlane>)>,
    sun_plane: Query<&Transform, (With<SunPlane>, Without<Camera>)>,
) {
    if input.pressed(KeyCode::Z) {
        let sun_transform = sun_plane.get_single().unwrap();
        let mut camera_transform = camera.get_single_mut().unwrap();
        camera_transform.look_at(sun_transform.translation, Vec3::Z);
    }
}

pub fn align_sun_plane_with_camera(
    input: Res<Input<KeyCode>>,
    camera: Query<&Transform, (With<Camera>, Without<SunPlane>)>,
    mut sun_plane_q: Query<&mut Transform, (With<SunPlane>, Without<Camera>)>,
) {
    if input.pressed(KeyCode::A) {
        let camera_transform = camera.get_single().unwrap();
        for mut transform in sun_plane_q.iter_mut() {
            *transform = Transform::default()
                .with_translation(transform.translation)
                .with_scale(transform.scale);
            transform.look_at(camera_transform.translation, Vec3::Z);
            transform.rotate_x(-FRAC_PI_2);
        }
    }
}

pub fn update_sun_settings(mut assets: ResMut<Assets<SunMaterial>>, time: Res<Time>) {
    for (_, material) in assets.iter_mut() {
        material.settings.time = time.elapsed_seconds();
    }
}

/// The Material trait is very configurable,
/// but comes with sensible defaults for all methods.
///
/// You only need to implement functions for features that need non-default behavior.
/// See the Material api docs for details!
impl Material for SunMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sun.wgsl".into()
    }
}

#[derive(Component)]
pub struct SunPlane;

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SunMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub base_texture: Handle<Image>,
    #[uniform(2)]
    pub settings: SunSettings,
}

// This is the component that will get passed to the shader
#[derive(Debug, Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct SunSettings {
    pub time: f32,
    /// The aspect ratio of the texture to draw on.
    pub aspect: f32,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    pub _webgl2_padding: Vec3,
}
