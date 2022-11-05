mod camera;
mod color;
mod puzzle;

use crate::camera::{CameraPlugin, CameraSettings};
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin::new(CameraSettings::default()))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let rubik = puzzle::rubiks::Rubik::new(3);

    let texture = images.add(rubik.create_texture());

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(texture),
        perceptual_roughness: 0.15,
        unlit: false,
        ..default()
    });

    for (mesh, transform) in rubik.create_meshes() {
        commands
            .spawn_bundle(PbrBundle {
                transform,
                mesh: meshes.add(mesh),
                material: material.clone(),
                ..default()
            })
            .insert(Wireframe);
    }

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50_000.0,
            shadows_enabled: false,
            ..default()
        },
        ..default()
    });
}
