use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod planet;
mod sun;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<SunMaterial>::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::I)),
        ))
        .add_systems(Startup, (setup_camera, setup_sun))
        .add_systems(Update, (update_sun_settings, align_sun_plane_with_camera, close_on_esc))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
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

pub fn align_sun_plane_with_camera(
    camera: Query<&Transform, (With<Camera>, Without<SunPlane>)>,
    mut sun_plane_q: Query<&mut Transform, (With<SunPlane>, Without<Camera>)>,
) {
    let camera_transform = camera.get_single().unwrap();
    for mut transform in sun_plane_q.iter_mut() {
        transform.rotation = camera_transform.rotation;
    }
}

// Change the intensity over time to show that the effect is controlled from the main world
pub fn update_sun_settings(mut assets: ResMut<Assets<SunMaterial>>, time: Res<Time>) {
    for (_, material) in assets.iter_mut() {
        // This will then be extracted to the render world and uploaded to the gpu automatically by the [`UniformComponentPlugin`]
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
