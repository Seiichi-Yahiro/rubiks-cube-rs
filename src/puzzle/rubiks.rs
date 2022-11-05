use bevy::prelude::{Color, Image, Mesh, Transform};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const CUBE_SIZE: f32 = 1.0;
const NUMBER_OF_SIDES: u32 = 6;
const NUMBER_OF_COLORS: u32 = NUMBER_OF_SIDES + 1;

pub struct Rubik {
    pub dimension: u32,
    pub colors: Colors,
    //size: f32,
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

impl Rubik {
    pub fn new(dimension: u32) -> Self {
        Self {
            dimension,
            colors: Colors::default(),
        }
    }

    pub fn create_texture(&self) -> Image {
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

    pub fn create_meshes(&self) -> Vec<(Mesh, Transform)> {
        if self.dimension == 1 {
            let mesh = Self::create_cube_mesh([0, 1, 2, 3, 4, 5]);
            let transform = Transform::identity();
            vec![(mesh, transform)]
        } else {
            // dimension^3 - (dimension - 2)^3
            let number_of_tiles = (6 * self.dimension * (self.dimension - 2) + 8) as usize;

            let mut meshes: Vec<(Mesh, Transform)> = Vec::with_capacity(number_of_tiles);

            let offset = CUBE_SIZE * (self.dimension + 1) as f32 / 2.0;

            let create_translation = |x, y, z| {
                Transform::from_xyz(
                    -(x as f32) * CUBE_SIZE + offset,
                    -(y as f32) * CUBE_SIZE + offset,
                    -(z as f32) * CUBE_SIZE + offset,
                )
            };

            for x in [1, self.dimension] {
                for y in 1..=self.dimension {
                    for z in 1..=self.dimension {
                        let mesh = Self::create_cube_mesh(self.get_color_map([x, y, z]));
                        let transform = create_translation(x, y, z);
                        meshes.push((mesh, transform));
                    }
                }
            }

            for y in [1, self.dimension] {
                for z in 1..=self.dimension {
                    for x in 2..self.dimension {
                        let mesh = Self::create_cube_mesh(self.get_color_map([x, y, z]));
                        let transform = create_translation(x, y, z);
                        meshes.push((mesh, transform));
                    }
                }
            }

            for z in [1, self.dimension] {
                for x in 2..self.dimension {
                    for y in 2..self.dimension {
                        let mesh = Self::create_cube_mesh(self.get_color_map([x, y, z]));
                        let transform = create_translation(x, y, z);
                        meshes.push((mesh, transform));
                    }
                }
            }

            meshes
        }
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

    fn create_cube_mesh(color_map: ColorMap) -> Mesh {
        const NUMBER_OF_VERTICES_PER_SIDE: usize = 4;
        const CAPACITY: usize = NUMBER_OF_VERTICES_PER_SIDE * NUMBER_OF_SIDES as usize;
        const HALF_SIZE: f32 = CUBE_SIZE / 2.0;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(CAPACITY);

        for axis in 0..3 {
            for x in [HALF_SIZE, -HALF_SIZE] {
                for y in [HALF_SIZE, -HALF_SIZE] {
                    for z in [HALF_SIZE, -HALF_SIZE] {
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
