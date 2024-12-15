use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::Player::components::Player;

pub const PLAYER_SPEED: f32 = 1000.0;
pub const PLAYER_SIZE: (f32, f32) = (128.0, 16.0);

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.5, 0.8),
            custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
            ..Default::default()
        },
        transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.05, 0.0),
        ..Default::default()
    }, Player{}
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_width = PLAYER_SIZE.0 / 2.0;
        let x_min = 0.0 + half_player_width;
        let x_max = window.width() - half_player_width;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        player_transform.translation = translation;
    }
}