use super::{GAP_SIZE, TOTAL_SIDE_LENGTH};
use crate::puzzle::Puzzle;
use crate::StandardMaterial;
use bevy::prelude::{Color, Handle, Image, Mesh, Transform, Vec3};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const NUMBER_OF_SIDES: u32 = 6;
const NUMBER_OF_COLORS: u32 = NUMBER_OF_SIDES + 1;

pub struct Rubik {
    pub dimension: u32,
    pub colors: Colors,
}

pub struct Colors {
    right: Color,
    left: Color,
    top: Color,
    bottom: Color,
    front: Color,
    back: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            right: crate::color::RED,
            left: crate::color::ORANGE,
            top: crate::color::YELLOW,
            bottom: crate::color::WHITE,
            front: crate::color::BLUE,
            back: crate::color::GREEN,
        }
    }
}

type ColorMap = [u32; NUMBER_OF_SIDES as usize];

impl Puzzle for Rubik {
    fn create_texture(&self) -> Image {
        let data = [
            self.colors.right.as_rgba_f32(),
            self.colors.left.as_rgba_f32(),
            self.colors.top.as_rgba_f32(),
            self.colors.bottom.as_rgba_f32(),
            self.colors.front.as_rgba_f32(),
            self.colors.back.as_rgba_f32(),
            crate::color::GRAY.as_rgba_f32(),
        ]
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
        if self.dimension == 1 {
            let mesh = self.create_cube_mesh([0, 1, 2, 3, 4, 5]);
            let transform = Transform::IDENTITY;
            vec![(mesh, transform)]
        } else {
            // dimension^3 - (dimension - 2)^3
            let number_of_tiles = (6 * self.dimension * (self.dimension - 2) + 8) as usize;

            let mut meshes: Vec<(Mesh, Transform)> = Vec::with_capacity(number_of_tiles);

            for x in [1, self.dimension] {
                for y in 1..=self.dimension {
                    for z in 1..=self.dimension {
                        let mesh = self.create_cube_mesh(self.get_color_map([x, y, z]));
                        let transform =
                            Transform::from_translation(self.get_tile_translation([x, y, z]));
                        meshes.push((mesh, transform));
                    }
                }
            }

            for y in [1, self.dimension] {
                for z in 1..=self.dimension {
                    for x in 2..self.dimension {
                        let mesh = self.create_cube_mesh(self.get_color_map([x, y, z]));
                        let transform =
                            Transform::from_translation(self.get_tile_translation([x, y, z]));
                        meshes.push((mesh, transform));
                    }
                }
            }

            for z in [1, self.dimension] {
                for x in 2..self.dimension {
                    for y in 2..self.dimension {
                        let mesh = self.create_cube_mesh(self.get_color_map([x, y, z]));
                        let transform =
                            Transform::from_translation(self.get_tile_translation([x, y, z]));
                        meshes.push((mesh, transform));
                    }
                }
            }

            meshes
        }
    }
}

impl Rubik {
    pub fn new(dimension: u32) -> Self {
        Self {
            dimension,
            colors: Colors::default(),
        }
    }

    fn get_cube_side_length(&self) -> f32 {
        let gap_space = (self.dimension - 1) as f32 * GAP_SIZE;
        let remaining_space = TOTAL_SIDE_LENGTH - gap_space;
        remaining_space / self.dimension as f32
    }

    fn get_tile_translation(&self, [x, y, z]: [u32; 3]) -> Vec3 {
        let cube_side_length = self.get_cube_side_length();
        let offset = (TOTAL_SIDE_LENGTH + cube_side_length) / 2.0;
        let vec = Vec3::new(x as f32, y as f32, z as f32);

        -(vec * (cube_side_length + GAP_SIZE) - GAP_SIZE - offset)
    }

    fn get_color_map(&self, [x, y, z]: [u32; 3]) -> ColorMap {
        let mut color_map: ColorMap = [6; NUMBER_OF_SIDES as usize];

        match x {
            1 => {
                color_map[0] = 0;
            }
            _ if x == self.dimension => {
                color_map[1] = 1;
            }
            _ => {}
        }

        match y {
            1 => {
                color_map[2] = 2;
            }
            _ if y == self.dimension => {
                color_map[3] = 3;
            }
            _ => {}
        }

        match z {
            1 => {
                color_map[4] = 4;
            }
            _ if z == self.dimension => {
                color_map[5] = 5;
            }
            _ => {}
        }

        color_map
    }

    fn create_cube_mesh(&self, color_map: ColorMap) -> Mesh {
        const NUMBER_OF_VERTICES_PER_SIDE: usize = 4;
        const CAPACITY: usize = NUMBER_OF_VERTICES_PER_SIDE * NUMBER_OF_SIDES as usize;
        let half_cube_side_length = self.get_cube_side_length() / 2.0;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(CAPACITY);

        for axis in 0..3 {
            for x in [half_cube_side_length, -half_cube_side_length] {
                for y in [half_cube_side_length, -half_cube_side_length] {
                    for z in [half_cube_side_length, -half_cube_side_length] {
                        let mut position = [x, y, z];
                        position.rotate_right(axis);
                        positions.push(position);
                    }
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

        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(CAPACITY);

        let color_size = 1.0 / NUMBER_OF_COLORS as f32;
        let offset = color_size / 2.0;

        for color_id in color_map {
            for _ in 0..NUMBER_OF_VERTICES_PER_SIDE {
                uvs.push([color_id as f32 * color_size + offset, offset]);
            }
        }

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
