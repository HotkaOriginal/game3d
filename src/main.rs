mod window;
mod world;
mod player;
mod camera;

use window::WindowPluginHandler;
use world::WorldPlugin;
use player::PlayerPlugin;
use camera::CameraPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
fn main() {
    App::new()
        .add_plugins((WindowPluginHandler, WorldPlugin, PlayerPlugin, CameraPlugin, WorldInspectorPlugin::new(), ThirdPersonCameraPlugin))
        .run();
}