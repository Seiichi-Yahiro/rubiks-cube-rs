mod camera;
mod color;
mod puzzle;
mod view;

use crate::camera::{CameraPlugin, CameraSettings};
use crate::puzzle::Puzzle;
use crate::view::{View, ViewPlugin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin::new(CameraSettings::default()))
        .add_plugin(ViewPlugin)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    view_query: Query<Entity, With<View>>,
) {
    let puzzle: Box<dyn Puzzle> = Box::new(puzzle::rubiks::Rubik::new(3));

    let texture = images.add(puzzle.create_texture());
    let material = materials.add(puzzle.create_material(texture));

    let view_entity = view_query.single();

    commands.entity(view_entity).add_children(|builder| {
        for (mesh, transform) in puzzle.create_meshes() {
            builder.spawn_bundle(PbrBundle {
                transform,
                mesh: meshes.add(mesh),
                material: material.clone(),
                ..default()
            });
        }
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
