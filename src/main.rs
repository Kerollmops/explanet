use std::f32::consts::{FRAC_PI_2, PI};

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::render::texture::{
    ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
};
use bevy::window::close_on_esc;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod planet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<SunMaterial>::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::I)),
        ))
        .register_type::<SunColor>()
        .insert_resource(SunColor { color: Color::rgb(0.988, 0.588, 0.302) })
        .add_systems(Startup, (setup_camera, setup_sun))
        .add_systems(
            Update,
            (
                update_sun_settings,
                align_billboards_with_camera,
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
        base_texture: asset_server.load_with_settings(
            "textures/abstract-bottle-glass.png",
            |s: &mut ImageLoaderSettings| {
                s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..default()
                })
            },
        ),
        settings: SunSettings { aspect: 1.0, ..default() },
    });

    // plane
    commands.spawn((
        BillBoard,
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, ..default() })),
            material,
            transform: Transform::default(),
            ..default()
        },
    ));
}

#[derive(Debug, Component)]
pub struct BillBoard;

pub fn align_billboards_with_camera(
    camera: Query<&Transform, (With<Camera>, Without<BillBoard>)>,
    mut billboard_q: Query<&mut Transform, (With<BillBoard>, Without<Camera>)>,
) {
    let camera_transform = camera.get_single().unwrap();
    for mut transform in billboard_q.iter_mut() {
        transform.rotation = camera_transform.rotation
            * Quat::from_rotation_y(PI)
            * Quat::from_rotation_x(-FRAC_PI_2);
    }
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SunColor {
    color: Color,
}

pub fn update_sun_settings(
    time: Res<Time>,
    sun_color: Res<SunColor>,
    mut assets: ResMut<Assets<SunMaterial>>,
) {
    for (_, material) in assets.iter_mut() {
        material.settings.time = time.elapsed_seconds();
        material.settings.sun_color = Vec4::from(sun_color.color.as_rgba_f32()).truncate();
    }
}

impl Material for SunMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sun.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SunMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub base_texture: Handle<Image>,
    #[uniform(2)]
    pub settings: SunSettings,
}

#[derive(Debug, Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct SunSettings {
    pub time: f32,
    /// The aspect ratio of the texture to draw on.
    pub aspect: f32,
    pub sun_color: Vec3,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    pub _webgl2_padding: Vec3,
}

impl Default for SunSettings {
    fn default() -> Self {
        SunSettings {
            time: 0.0,
            aspect: 1.0,
            sun_color: Vec3::new(0.82, 0.35, 0.1),
            #[cfg(feature = "webgl2")]
            _webgl2_padding: Vec3::default(),
        }
    }
}
