use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ViewRotation::default())
            .add_startup_system(setup)
            .add_system(change_view);
    }
}

#[derive(Component)]
pub struct View;

#[derive(Debug, Default, Resource)]
struct ViewRotation {
    pitch: f32,
    yaw: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((SpatialBundle::VISIBLE_IDENTITY, View));
}

fn change_view(
    mut view_rotation: ResMut<ViewRotation>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    touches: Res<Touches>,
    mut query: Query<&mut Transform, With<View>>,
    time: Res<Time>,
) {
    let delta: Vec2;

    if let Some(touch) = touches.iter().next() {
        delta = touch.delta();
    } else if mouse_button_input.pressed(MouseButton::Left) {
        delta = mouse_motion_events
            .iter()
            .map(|mouse_motion| mouse_motion.delta)
            .fold(Vec2::ZERO, std::ops::Add::add);
    } else {
        return;
    }

    let sensitivity = 0.25;
    let delta_time = time.delta_seconds();

    view_rotation.pitch += delta.y * delta_time * sensitivity;
    view_rotation.yaw += delta.x * delta_time * sensitivity;

    view_rotation.pitch = view_rotation
        .pitch
        .clamp(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);

    while view_rotation.yaw.abs() > std::f32::consts::TAU {
        view_rotation.yaw -= std::f32::consts::TAU * view_rotation.yaw.signum();
    }

    let mut view = query.single_mut();
    view.rotation = Quat::from_euler(EulerRot::XYZ, view_rotation.pitch, view_rotation.yaw, 0.0);
}
