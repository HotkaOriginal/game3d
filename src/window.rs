use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
// use bevy::window::WindowMode::Fullscreen;
// use bevy::window::WindowMode::Fullscreen;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub struct WindowPluginHandler;

impl Plugin for WindowPluginHandler {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, )
        app
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "3D Game".into(),
                    name: Some("bevy.app".into()),
                    // It spawns a squared window
                    // resolution: (1000., 500.).into(),
                    resizable: true,
                    // mode: Fullscreen,
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: false,
                    ..default()
                }),
                ..default()
            },  ))
            .add_systems(Startup, (set_window_icon, setup_window))
            .add_systems(Update, (make_visible, exit_esc));
    }
}

fn exit_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("app_icon/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn setup_window(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    // Buttons on the top of the window (keep the close button)
    window.enabled_buttons.maximize = false;
    window.enabled_buttons.minimize = false;

    // The window is not always on top because it's commented
    // window.window_level = WindowLevel::AlwaysOnTop;
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}