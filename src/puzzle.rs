use crate::StandardMaterial;
use bevy::prelude::{Handle, Image, Mesh, Transform};

pub mod rubiks;

const TOTAL_SIDE_LENGTH: f32 = 1.0;
const GAP_SIZE: f32 = 0.005;

pub trait Puzzle {
    fn create_texture(&self) -> Image;
    fn create_material(&self, texture: Handle<Image>) -> StandardMaterial;
    fn create_meshes(&self) -> Vec<(Mesh, Transform)>;
}
