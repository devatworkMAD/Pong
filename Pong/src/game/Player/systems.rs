use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::Player::components::Player;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.5, 0.8),
            custom_size: Some(Vec2::new(100.0, 100.0)), // Width and height of the box
            ..Default::default()
        },
        transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.05, 0.0), // Position of the box
        ..Default::default()
    }, Player{}
    ));
}