use bevy::color::Color;
use bevy::log::tracing_subscriber::fmt::time;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::Ball::components::Ball;
use crate::game::Player::components::Player;

pub const BALL_SPEED: f32 = 1000.0;
pub const BALL_SIZE: f32 = 16.0;
const GRAVITY: f32 = -500.0;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::hsl(1.0, 1.0, 1.0))),
        Transform::from_xyz(window.width() * 0.5, window.height() - BALL_SIZE, 0.0),
        Ball
    ));
}

#[derive(Resource)]
pub struct BallSpeed {
    x: f32,
    y: f32
}

pub fn setup_BallSpeed(mut commands: Commands) {
    commands.insert_resource(BallSpeed{x: 0.0, y: 0.0});
}

pub fn move_ball(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    mut ball_speed: ResMut<BallSpeed>,
    time: Res<Time>
){
    if let Ok(mut transform) = ball_query.get_single_mut() {
        transform.translation.x += ball_speed.x * time.delta_secs();
        transform.translation.y += ball_speed.y * time.delta_secs();
    }
}

pub fn gravity(
    mut ball_speed: ResMut<BallSpeed>,
    time: Res<Time>
){
    ball_speed.y += GRAVITY * time.delta_secs();
}

pub const PLAYER_SIZE: (f32, f32) = (128.0, 16.0);

pub fn hit_by_player(
    mut player_query: Query<&Transform, (With<Player>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform), With<Ball>>,
    mut ball_speed: ResMut<BallSpeed>
) {
    for player_transform in player_query.iter_mut() {
        for (mut ball_transform) in ball_query.iter_mut() {
            if is_colliding(player_transform, &ball_transform) {
                ball_speed.y = ball_speed.y.abs();


                let player_x = player_transform.translation.x;
                let ball_x = ball_transform.translation.x;
                let offset = (ball_x - player_x) / (PLAYER_SIZE.0 / 2.0);
                ball_speed.x = offset * 300.0;
            }
        }
    }
}

fn is_colliding(player_transform: &Transform, ball_transform: &Transform) -> bool {
    let ball_pos = ball_transform.translation;
    let player_pos = player_transform.translation;

    let player_half_width = PLAYER_SIZE.0 / 2.0;
    let player_half_height = PLAYER_SIZE.1 / 2.0;

    ball_pos.x >= player_pos.x - player_half_width &&
        ball_pos.x <= player_pos.x + player_half_width &&
        ball_pos.y - BALL_SIZE <= player_pos.y + player_half_height &&
        ball_pos.y + BALL_SIZE >= player_pos.y - player_half_height
}

pub fn confine_ball(
    mut ball_query: Query<(&mut Transform), With<Ball>>,
    mut ball_speed: ResMut<BallSpeed>,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();
    let half_ball_size = BALL_SIZE / 2.0;

    if let Ok(mut transform) = ball_query.get_single_mut() {
        if  transform.translation.x  >= window.width() - half_ball_size{
            ball_speed.x = -ball_speed.x.abs();
        }
        if  transform.translation.x  <= half_ball_size{
            ball_speed.x = ball_speed.x.abs();
        }
    }
}