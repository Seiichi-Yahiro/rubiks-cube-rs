use super::{GAP_SIZE, TOTAL_SIDE_LENGTH};
use crate::puzzle::Puzzle;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const NUMBER_OF_TETRAHEDRON_SIDES: u32 = 4;
const NUMBER_OF_BIPYRAMID_SIDES: u32 = 8;
const NUMBER_OF_COLORS: u32 = NUMBER_OF_TETRAHEDRON_SIDES + 1;

const NUMBER_OF_VERTICES_PER_SIDE: u32 = 3;
const HALF_TOTAL_SIDE_LENGTH: f32 = TOTAL_SIDE_LENGTH / 2.0;

const ROOT_3: f32 = 1.7320508;
const ROOT_6: f32 = 2.4494898;

const FACE_HEIGHT: f32 = HALF_TOTAL_SIDE_LENGTH * ROOT_3;
const THIRD_FACE_HEIGHT: f32 = FACE_HEIGHT / 3.0;

const HEIGHT: f32 = (TOTAL_SIDE_LENGTH / 3.0) * ROOT_6;
const HALF_HEIGHT: f32 = HEIGHT / 2.0;

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
        vec![(self.create_bipyramid_mesh(), Transform::IDENTITY)]
    }
}

impl Pyraminx {
    pub fn new(dimension: u32) -> Self {
        Self { dimension }
    }

    fn create_tetrahedron_mesh(&self) -> Mesh {
        let back = [0.0, -HALF_HEIGHT, -THIRD_FACE_HEIGHT * 2.0];
        let left = [-HALF_TOTAL_SIDE_LENGTH, -HALF_HEIGHT, THIRD_FACE_HEIGHT];
        let right = [HALF_TOTAL_SIDE_LENGTH, -HALF_HEIGHT, THIRD_FACE_HEIGHT];
        let top = [0.0, HALF_HEIGHT, 0.0];

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

        let indices: Vec<u32> = (0..NUMBER_OF_TETRAHEDRON_SIDES)
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

    fn create_bipyramid_mesh(&self) -> Mesh {
        const TOTAL_DEPTH: f32 = FACE_HEIGHT + THIRD_FACE_HEIGHT;
        const HALF_TOTAL_DEPTH: f32 = TOTAL_DEPTH / 2.0;

        let upper_back = [0.0, HALF_HEIGHT, -HALF_TOTAL_DEPTH];
        let upper_left = [
            -HALF_TOTAL_SIDE_LENGTH,
            HALF_HEIGHT,
            -HALF_TOTAL_DEPTH + FACE_HEIGHT,
        ];
        let upper_right = [
            HALF_TOTAL_SIDE_LENGTH,
            HALF_HEIGHT,
            -HALF_TOTAL_DEPTH + FACE_HEIGHT,
        ];

        let lower_front = [0.0, -HALF_HEIGHT, HALF_TOTAL_DEPTH];
        let lower_left = [
            -HALF_TOTAL_SIDE_LENGTH,
            -HALF_HEIGHT,
            HALF_TOTAL_DEPTH - FACE_HEIGHT,
        ];
        let lower_right = [
            HALF_TOTAL_SIDE_LENGTH,
            -HALF_HEIGHT,
            HALF_TOTAL_DEPTH - FACE_HEIGHT,
        ];

        #[rustfmt::skip]
        let positions: Vec<[f32; 3]> = vec![
            lower_front, lower_left, lower_right, // bottom
            upper_back, upper_left, upper_right, // top
            lower_front, upper_right, upper_left, // upper_front
            lower_right, upper_back, upper_right, // upper_right
            lower_left, upper_left, upper_back, // upper_left
            upper_back, lower_right, lower_left, // lower_back
            upper_right, lower_front, lower_right, // lower_right
            upper_left, lower_left, lower_front, // lower_left
        ];

        let normals: Vec<[f32; 3]> = {
            let upper_back = Vec3::from_array(upper_back);
            let upper_left = Vec3::from_array(upper_left);
            let upper_right = Vec3::from_array(upper_right);

            let lower_front = Vec3::from_array(lower_front);
            let lower_left = Vec3::from_array(lower_left);
            let lower_right = Vec3::from_array(lower_right);

            let top_normal = [0.0, 1.0, 0.0];
            let bottom_normal = [0.0, -1.0, 0.0];
            let upper_front_normal = (upper_right - lower_front)
                .cross(upper_left - lower_front)
                .normalize()
                .to_array();
            let upper_right_normal = (upper_back - lower_right)
                .cross(upper_right - lower_right)
                .normalize()
                .to_array();
            let upper_left_normal = (upper_left - lower_left)
                .cross(upper_back - lower_left)
                .normalize()
                .to_array();
            let lower_back_normal = (lower_right - upper_back)
                .cross(lower_left - upper_back)
                .normalize()
                .to_array();
            let lower_right_normal = (lower_front - upper_right)
                .cross(lower_right - upper_right)
                .normalize()
                .to_array();
            let lower_left_normal = (lower_left - upper_left)
                .cross(lower_front - upper_left)
                .normalize()
                .to_array();

            [
                bottom_normal,
                top_normal,
                upper_front_normal,
                upper_right_normal,
                upper_left_normal,
                lower_back_normal,
                lower_right_normal,
                lower_left_normal,
            ]
            .into_iter()
            .flat_map(|normal| [normal; NUMBER_OF_VERTICES_PER_SIDE as usize])
            .collect()
        };

        let indices: Vec<u32> = (0..NUMBER_OF_BIPYRAMID_SIDES)
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
