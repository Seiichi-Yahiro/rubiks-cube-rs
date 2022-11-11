use super::{GAP_SIZE, TOTAL_SIDE_LENGTH};
use crate::puzzle::Puzzle;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const PHYSICAL_SIDE_LENGTH: f32 = 0.057;
const SCALE_FACTOR: f32 = TOTAL_SIDE_LENGTH / PHYSICAL_SIDE_LENGTH;

const GAP_SPACE: f32 = 2.0 * GAP_SIZE;

// opposite sides add up to 0.038
// all middle pieces are 0.019
// total side length 0.019 + 0.038 = 0.057
const RIGHT_THICKNESS: f32 = 0.025 * SCALE_FACTOR;
const LEFT_THICKNESS: f32 = 0.013 * SCALE_FACTOR;
const TOP_THICKNESS: f32 = 0.029 * SCALE_FACTOR;
const BOTTOM_THICKNESS: f32 = 0.009 * SCALE_FACTOR;
const FRONT_THICKNESS: f32 = 0.021 * SCALE_FACTOR;
const BACK_THICKNESS: f32 = 0.017 * SCALE_FACTOR;
const MIDDLE_THICKNESS: f32 = 0.019 * SCALE_FACTOR - GAP_SPACE; // make middle piece smaller to accommodate for gaps

const X_THICKNESS: [f32; 3] = [RIGHT_THICKNESS, MIDDLE_THICKNESS, LEFT_THICKNESS];
const Y_THICKNESS: [f32; 3] = [TOP_THICKNESS, MIDDLE_THICKNESS, BOTTOM_THICKNESS];
const Z_THICKNESS: [f32; 3] = [FRONT_THICKNESS, MIDDLE_THICKNESS, BACK_THICKNESS];

const NUMBER_OF_SIDES: u32 = 6;
const NUMBER_OF_COLORS: u32 = 1;

pub struct Mirror;

impl Puzzle for Mirror {
    fn create_texture(&self) -> Image {
        let data = crate::color::GOLD
            .as_rgba_f32()
            .into_iter()
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
            perceptual_roughness: 0.05,
            metallic: 0.75,
            ..default()
        }
    }

    fn create_meshes(&self) -> Vec<(Mesh, Transform)> {
        // capacity = dimension^3 - (dimension - 2)^3
        let mut meshes: Vec<(Mesh, Transform)> = Vec::with_capacity(26);

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x & y & z == 1 {
                        continue;
                    }

                    let side_lengths = Vec3::new(X_THICKNESS[x], Y_THICKNESS[y], Z_THICKNESS[z]);
                    let mesh = self.create_mesh(side_lengths);
                    let transform =
                        Transform::from_translation(self.get_tile_translation([x, y, z]));

                    meshes.push((mesh, transform));
                }
            }
        }

        meshes
    }
}

impl Mirror {
    pub fn new() -> Self {
        Self
    }

    fn get_tile_translation(&self, [x, y, z]: [usize; 3]) -> Vec3 {
        const HALF_MIDDLE_THICKNESS: f32 = MIDDLE_THICKNESS / 2.0;

        let index_vec = Vec3::new(x as f32, y as f32, z as f32) - 1.0;
        let middle_offset = Vec3::splat(HALF_MIDDLE_THICKNESS + GAP_SIZE);
        let side_offset = Vec3::new(X_THICKNESS[x], Y_THICKNESS[y], Z_THICKNESS[z]) / 2.0;

        let offset = 0.5
            - (Vec3::new(RIGHT_THICKNESS, TOP_THICKNESS, FRONT_THICKNESS)
                + HALF_MIDDLE_THICKNESS
                + GAP_SIZE);

        -(index_vec * (side_offset + middle_offset) - offset)
    }

    pub fn create_mesh(&self, side_lengths: Vec3) -> Mesh {
        const NUMBER_OF_VERTICES_PER_SIDE: usize = 4;
        const CAPACITY: usize = NUMBER_OF_VERTICES_PER_SIDE * NUMBER_OF_SIDES as usize;

        let half_side_lengths = side_lengths / 2.0;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(CAPACITY);

        // right left
        for x in [half_side_lengths.x, -half_side_lengths.x] {
            for y in [half_side_lengths.y, -half_side_lengths.y] {
                for z in [half_side_lengths.z, -half_side_lengths.z] {
                    positions.push([x, y, z]);
                }
            }
        }

        // top bottom
        for y in [half_side_lengths.y, -half_side_lengths.y] {
            for z in [half_side_lengths.z, -half_side_lengths.z] {
                for x in [half_side_lengths.x, -half_side_lengths.x] {
                    positions.push([x, y, z]);
                }
            }
        }

        // front back
        for z in [half_side_lengths.z, -half_side_lengths.z] {
            for x in [half_side_lengths.x, -half_side_lengths.x] {
                for y in [half_side_lengths.y, -half_side_lengths.y] {
                    positions.push([x, y, z]);
                }
            }
        }

        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(CAPACITY);

        for axis in 0..3 {
            for sign in [1.0, -1.0] {
                let mut normal = [sign, 0.0, 0.0];
                normal.rotate_right(axis);

                for _ in 0..NUMBER_OF_VERTICES_PER_SIDE {
                    normals.push(normal);
                }
            }
        }

        let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0]; CAPACITY];

        let indices: Vec<u32> = vec![
            0, 2, 1, 1, 2, 3, // right
            4, 5, 6, 6, 5, 7, // left
            8, 10, 9, 9, 10, 11, // top
            12, 13, 14, 14, 13, 15, // bottom
            16, 18, 17, 17, 18, 19, // front
            20, 21, 22, 22, 21, 23, // back
        ];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));

        mesh
    }
}
