use bevy::prelude::*;
use bevy_third_person_camera::camera::Zoom;
use bevy_third_person_camera::ThirdPersonCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            aim_speed: 25.,
            zoom_enabled: true,
            zoom_sensitivity: 5.0,
            mouse_sensitivity: 10.,
            zoom: Zoom::new(5., 20.),
            ..default()
        },
    );
    commands.spawn(camera);
}