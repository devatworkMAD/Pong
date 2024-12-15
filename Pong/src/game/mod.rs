use bevy::prelude::{App, Plugin};
use crate::game::Player::PlayerPlugin;

mod systems;
mod Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((PlayerPlugin));
    }
}