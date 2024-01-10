use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use enum_iterator::{all, Sequence};

#[derive(Component)]
pub struct Planet;

impl Planet {
    pub fn with_resolution(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        resolution: u32,
        color: Color,
    ) {
        // We refer to this material in each of the faces mesh.
        let material = materials.add(color.into());
        let mut commands = commands.spawn((Planet, SpatialBundle::default(), material.clone()));

        for face in all::<Face>() {
            commands.with_children(|commands| {
                commands.spawn((
                    face,
                    PbrBundle {
                        mesh: meshes.add(create_face_mesh(resolution, face.orientation())),
                        material: material.clone_weak(),
                        ..default()
                    },
                ));
            });
        }
    }
}

#[derive(Debug, Component, Clone, Copy, Sequence)]
pub enum Face {
    Top,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Face {
    fn orientation(&self) -> Vec3 {
        match self {
            Face::Top => Vec3::Y,
            Face::Down => Vec3::NEG_Y,
            Face::Left => Vec3::NEG_X,
            Face::Right => Vec3::X,
            Face::Front => Vec3::Z,
            Face::Back => Vec3::NEG_Z,
        }
    }
}

fn create_face_mesh(resolution: u32, local_up: Vec3) -> Mesh {
    let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
    let axis_b = local_up.cross(axis_a);

    let mut vertices = vec![Vec3::ZERO; (resolution * resolution) as usize];
    let mut triangles = vec![0u32; ((resolution - 1) * (resolution - 1) * 2 * 3) as usize];

    let mut tri_index = 0;
    for y in 0..resolution {
        for x in 0..resolution {
            let i = x + y * resolution;
            let percent = Vec2::new(x as f32, y as f32) / (resolution - 1) as f32;
            let point_on_unit_cube =
                local_up + (percent.x - 0.5) * 2.0 * axis_a + (percent.y - 0.5) * 2.0 * axis_b;
            let point_on_unit_sphere = point_on_unit_cube.normalize();
            vertices[i as usize] = point_on_unit_sphere;

            if x != resolution - 1 && y != resolution - 1 {
                triangles[tri_index] = i;
                triangles[tri_index + 1] = i + resolution + 1;
                triangles[tri_index + 2] = i + resolution;

                triangles[tri_index + 3] = i;
                triangles[tri_index + 4] = i + 1;
                triangles[tri_index + 5] = i + resolution + 1;
                tri_index += 6;
            }
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList)
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_indices(Some(Indices::U32(triangles)))
        .with_duplicated_vertices()
        .with_computed_flat_normals()
}
