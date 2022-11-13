use super::{GAP_SIZE, TOTAL_SIDE_LENGTH};
use crate::puzzle::Puzzle;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const NUMBER_OF_SIDES: u32 = 4;
const NUMBER_OF_COLORS: u32 = NUMBER_OF_SIDES + 1;

pub struct Pyraminx {
    pub dimension: u32,
}

impl Puzzle for Pyraminx {
    fn create_texture(&self) -> Image {
        let colors: [[f32; 4]; NUMBER_OF_COLORS as usize] = [
            crate::color::GREEN.as_rgba_f32(),
            crate::color::BLUE.as_rgba_f32(),
            crate::color::YELLOW.as_rgba_f32(),
            crate::color::RED.as_rgba_f32(),
            crate::color::GRAY.as_rgba_f32(),
        ];

        let data = colors
            .into_iter()
            .flatten()
            .map(|color| (255.0 * color) as u8)
            .collect();

        Image::new(
            Extent3d {
                width: NUMBER_OF_COLORS,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
        )
    }

    fn create_material(&self, texture: Handle<Image>) -> StandardMaterial {
        StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(texture),
            perceptual_roughness: 0.15,
            ..Default::default()
        }
    }

    fn create_meshes(&self) -> Vec<(Mesh, Transform)> {
        vec![(self.create_tetrahedron_mesh(), Transform::IDENTITY)]
    }
}

impl Pyraminx {
    pub fn new(dimension: u32) -> Self {
        Self { dimension }
    }

    fn create_tetrahedron_mesh(&self) -> Mesh {
        const NUMBER_OF_VERTICES_PER_SIDE: u32 = 3;
        const HALF_TOTAL_SIDE_LENGTH: f32 = TOTAL_SIDE_LENGTH / 2.0;

        let face_height = HALF_TOTAL_SIDE_LENGTH * 3.0f32.sqrt();
        let third_face_height = face_height / 3.0;

        let height = (TOTAL_SIDE_LENGTH / 3.0) * 6.0f32.sqrt();
        let half_height = height / 2.0;

        let back = [0.0, -half_height, -third_face_height * 2.0];
        let left = [-HALF_TOTAL_SIDE_LENGTH, -half_height, third_face_height];
        let right = [HALF_TOTAL_SIDE_LENGTH, -half_height, third_face_height];
        let top = [0.0, half_height, 0.0];

        let positions: Vec<[f32; 3]> = vec![
            back, right, left, // bottom side
            top, left, right, // front side
            top, right, back, // right side
            top, back, left, // left side
        ];

        let normals: Vec<[f32; 3]> = {
            let back = Vec3::from_array(back);
            let left = Vec3::from_array(left);
            let right = Vec3::from_array(right);
            let top = Vec3::from_array(top);

            let bottom_normal = [0.0, -1.0, 0.0];
            let front_normal = (left - top).cross(right - top).normalize().to_array();
            let right_normal = (right - top).cross(back - top).normalize().to_array();
            let left_normal = (back - top).cross(left - top).normalize().to_array();

            [bottom_normal, front_normal, right_normal, left_normal]
                .into_iter()
                .flat_map(|normal| [normal; NUMBER_OF_VERTICES_PER_SIDE as usize])
                .collect()
        };

        let indices: Vec<u32> = (0..NUMBER_OF_SIDES)
            .flat_map(|i| {
                let offset = i * NUMBER_OF_VERTICES_PER_SIDE;
                offset..offset + NUMBER_OF_VERTICES_PER_SIDE
            })
            .collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));

        mesh
    }
}
