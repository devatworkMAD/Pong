use bevy::prelude::{App, Plugin};
use crate::game::Ball::BallPlugin;
use crate::game::Player::PlayerPlugin;

mod systems;
mod Player;
mod Ball;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PlayerPlugin,
                BallPlugin
            ));
    }
}