use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Billboard;

pub fn align_billboards_with_camera(
    camera: Query<&Transform, (With<Camera>, Without<Billboard>)>,
    mut billboard_q: Query<&mut Transform, (With<Billboard>, Without<Camera>)>,
) {
    let camera_transform = camera.get_single().unwrap();
    for mut transform in billboard_q.iter_mut() {
        transform.rotation = camera_transform.rotation
            * Quat::from_rotation_y(PI)
            * Quat::from_rotation_x(-FRAC_PI_2);
    }
}
