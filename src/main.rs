use std::f32::consts::{FRAC_PI_2, PI};

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::render::texture::{
    ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
};
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
    commands
        .spawn((TransformBundle::default(), VisibilityBundle::default(), BillBoard))
        .with_children(|parent| {
            parent.spawn((
                SunRendering,
                MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, ..default() })),
                    material,
                    transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
                    ..default()
                },
            ));
        });
}

#[derive(Debug, Component)]
pub struct BillBoard;

pub fn align_billboards_with_camera(
    camera: Query<&Transform, (With<Camera>, Without<BillBoard>)>,
    mut billboard_q: Query<&mut Transform, (With<BillBoard>, Without<Camera>)>,
) {
    let camera_transform = camera.get_single().unwrap();
    for mut transform in billboard_q.iter_mut() {
        transform.rotation = camera_transform.rotation * Quat::from_rotation_y(PI);
    }
}

pub fn update_sun_settings(mut assets: ResMut<Assets<SunMaterial>>, time: Res<Time>) {
    for (_, material) in assets.iter_mut() {
        material.settings.time = time.elapsed_seconds();
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

#[derive(Component)]
pub struct SunRendering;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SunMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub base_texture: Handle<Image>,
    #[uniform(2)]
    pub settings: SunSettings,
}

#[derive(Debug, Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct SunSettings {
    pub time: f32,
    /// The aspect ratio of the texture to draw on.
    pub aspect: f32,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    pub _webgl2_padding: Vec3,
}
