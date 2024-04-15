use std::time::Duration;
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, setup_scene_once_loaded, keyboard_animation_control));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("character/Animations.glb#Animation1"),
        asset_server.load("character/Animations.glb#Animation0"),
    ]));

    // Character
    let player = (
        SceneBundle {
            scene: asset_server.load("character/Animations.glb#Scene0"),
            transform: Transform::from_xyz(2.5, 0.5, 2.5),
            ..default()
        },
        Player,
        Name::new("Player"),
        ThirdPersonCameraTarget,
        Speed(2.5),);

    commands.spawn(player);
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn player_movement(
    time: Res<Time>,
    key: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = cam_q.get_single().unwrap_or_else(|e| Err(format!("Error retrieving camera: {}", e)).unwrap());

        let mut direction = Vec3::ZERO;

        // forward
        if key.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        // back
        if key.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        // left
        if key.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        // right
        if key.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to face direction he is currently moving
        /*
        if direction.length_squared() > 0.0 {
            let direc = (direction.z, direction.y, direction.x);
            player_transform.look_to(direc.into(), Vec3::Y);
        }

         */
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    for mut player in &mut animation_players {
    /*
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed + 0.1);
        }

     */

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::KeyA) ||
            keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::KeyD){
            *current_animation = 1;
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }
        else {
            *current_animation = 0;
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }
        /*

        if keyboard_input.just_pressed(KeyCode::Digit1) {
            player.set_repeat(RepeatAnimation::Count(1));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit3) {
            player.set_repeat(RepeatAnimation::Count(3));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit5) {
            player.set_repeat(RepeatAnimation::Count(5));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::KeyL) {
            player.set_repeat(RepeatAnimation::Forever);
        }

         */
    }
}