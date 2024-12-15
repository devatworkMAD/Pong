use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::Ball::components::Ball;

pub const BALL_SPEED: f32 = 1000.0;
pub const BALL_SIZE: f32 = 16.0;

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
        Ball{}
    ));
}