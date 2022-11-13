use bevy::ecs::schedule::ShouldRun;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

#[derive(Debug, Copy, Clone)]
pub struct CameraPlugin {
    settings: CameraSettings,
}

impl CameraPlugin {
    pub fn new(settings: CameraSettings) -> Self {
        Self { settings }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings).add_startup_system(setup);

        #[cfg(debug_assertions)]
        app.insert_resource(FlyCamera::default())
            .add_system(cursor_grab.with_run_criteria(is_flying))
            .add_system_set(
                SystemSet::new()
                    .label("fly_camera")
                    .with_run_criteria(is_cursor_confined)
                    .with_system(fly_camera_look_around)
                    .with_system(fly_camera_movement),
            )
            .add_system(mode_switch.after("fly_camera"));
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Static,
    Flying,
}

#[cfg(debug_assertions)]
impl Default for Mode {
    fn default() -> Self {
        Self::Static
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StaticCameraSettings {
    pub pos: Vec3,
    pub looking_at: Vec3,
}

impl Default for StaticCameraSettings {
    fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 5.0),
            looking_at: Vec3::ZERO,
        }
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Default, Resource)]
struct FlyCamera {
    pitch: f32,
    yaw: f32,
}

#[cfg(debug_assertions)]
#[derive(Debug, Copy, Clone)]
pub struct FlyingCameraSettings {
    pub movement_speed: f32,
    pub sensitivity: f32,
}

#[cfg(debug_assertions)]
impl Default for FlyingCameraSettings {
    fn default() -> Self {
        Self {
            movement_speed: 2.5,
            sensitivity: 0.25,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Resource)]
pub struct CameraSettings {
    #[cfg(debug_assertions)]
    pub mode: Mode,

    pub static_settings: StaticCameraSettings,

    #[cfg(debug_assertions)]
    pub flying_settings: FlyingCameraSettings,
}

fn setup(mut commands: Commands, camera_settings: Res<CameraSettings>) {
    let transform = Transform::from_translation(camera_settings.static_settings.pos)
        .looking_at(camera_settings.static_settings.looking_at, Vec3::Y);

    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
}

#[cfg(debug_assertions)]
fn mode_switch(
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut camera_settings: ResMut<CameraSettings>,
    mut fly_camera: ResMut<FlyCamera>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        let window = windows.get_primary_mut().unwrap();
        let mut transform = query.single_mut();

        camera_settings.mode = match camera_settings.mode {
            Mode::Static => {
                window.set_cursor_visibility(false);
                window.set_cursor_grab_mode(CursorGrabMode::Confined);

                fly_camera.yaw = 0.0;
                fly_camera.pitch = 0.0;

                Mode::Flying
            }
            Mode::Flying => {
                window.set_cursor_visibility(true);
                window.set_cursor_grab_mode(CursorGrabMode::None);

                *transform = Transform::from_translation(camera_settings.static_settings.pos)
                    .looking_at(camera_settings.static_settings.looking_at, Vec3::Y);

                Mode::Static
            }
        };
    }
}

#[cfg(debug_assertions)]
fn is_flying(camera_settings: Res<CameraSettings>) -> ShouldRun {
    match camera_settings.mode {
        Mode::Static => ShouldRun::No,
        Mode::Flying => ShouldRun::Yes,
    }
}

#[cfg(debug_assertions)]
fn cursor_grab(keyboard_input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_visibility(!window.cursor_visible());

        let grab_mode = match window.cursor_grab_mode() {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined | CursorGrabMode::Locked => CursorGrabMode::None,
        };

        window.set_cursor_grab_mode(grab_mode);
    }
}

#[cfg(debug_assertions)]
fn is_cursor_confined(windows: Res<Windows>) -> ShouldRun {
    let window = windows.get_primary().unwrap();

    match window.cursor_grab_mode() {
        CursorGrabMode::Confined => ShouldRun::Yes,
        CursorGrabMode::None | CursorGrabMode::Locked => ShouldRun::No,
    }
}

#[cfg(debug_assertions)]
fn fly_camera_movement(
    camera_settings: Res<CameraSettings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    let translation = keyboard_input
        .get_pressed()
        .map(|key| match key {
            KeyCode::Space => Vec3::Y,
            KeyCode::LShift => Vec3::NEG_Y,
            KeyCode::W => transform.forward(),
            KeyCode::S => transform.back(),
            KeyCode::A => transform.left(),
            KeyCode::D => transform.right(),
            _ => Vec3::ZERO,
        })
        .fold(Vec3::ZERO, std::ops::Add::add)
        .normalize_or_zero();

    transform.translation +=
        translation * time.delta_seconds() * camera_settings.flying_settings.movement_speed;
}

#[cfg(debug_assertions)]
fn fly_camera_look_around(
    camera_settings: Res<CameraSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut fly_camera: ResMut<FlyCamera>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let sensitivity = camera_settings.flying_settings.sensitivity;
    let delta_time = time.delta_seconds();

    for event in mouse_motion_events.iter() {
        let delta = event.delta;

        fly_camera.pitch -= delta.y * delta_time * sensitivity;
        fly_camera.yaw -= delta.x * delta_time * sensitivity;
    }

    fly_camera.pitch = fly_camera.pitch.clamp(-1.5, 1.5);

    let current_rotation = Transform::from_translation(camera_settings.static_settings.pos)
        .looking_at(camera_settings.static_settings.looking_at, Vec3::Y)
        .rotation;

    let mut transform = query.single_mut();

    transform.rotation =
        Quat::from_euler(EulerRot::YXZ, fly_camera.yaw, fly_camera.pitch, 0.0) * current_rotation;
}
