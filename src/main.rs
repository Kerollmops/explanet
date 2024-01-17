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
        .add_systems(Update, (update_sun_settings, close_on_esc))
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
        SunSettings::default(),
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
    mut sunmaterials: ResMut<Assets<SunMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load Texture
    let material = sunmaterials.add(SunMaterial {
        base_texture: asset_server.load("textures/abstract-bottle-glass.png"),
        settings: SunSettings { aspect: 1.0, ..default() },
    });

    // plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0, ..default() })),
        material,
        ..default()
    });
}

// Change the intensity over time to show that the effect is controlled from the main world
pub fn update_sun_settings(mut settings: Query<&mut SunSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        // This will then be extracted to the render world and uploaded to the gpu automatically by the [`UniformComponentPlugin`]
        setting.time = time.elapsed_seconds();
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
