use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::render::texture::{
    ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
};
use bevy_inspector_egui::prelude::*;

use crate::billboard::Billboard;

#[derive(Component)]
pub struct Sun;

pub fn setup_sun(
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

    commands
        .spawn((
            Billboard,
            Sun,
            // sun material plane
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 3.0, ..default() })),
                material,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 111_000.0,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            });
        });
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SunColor {
    pub color: Color,
}

pub fn update_sun_settings(
    time: Res<Time>,
    sun_color: Res<SunColor>,
    mut assets: ResMut<Assets<SunMaterial>>,
    mut point_light_q: Query<&mut PointLight>,
) {
    for (_, material) in assets.iter_mut() {
        material.settings.time = time.elapsed_seconds_wrapped();
        material.settings.sun_color = Vec4::from(sun_color.color.as_rgba_f32()).truncate();
    }

    for mut point_light in point_light_q.iter_mut() {
        point_light.color = sun_color.color;
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
            sun_color: Vec3::new(0.54, 0.16, 0.0),
            #[cfg(feature = "webgl2")]
            _webgl2_padding: Vec3::default(),
        }
    }
}
