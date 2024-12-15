use bevy::app::Plugin;
use bevy::prelude::*;

pub(crate) mod components;
mod systems;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        println!("build PlayerPlugin");
        app
            .add_systems(Startup, (
                spawn_player
            ))
            .add_systems(Update,(
                player_movement,
                confine_player_movement
                ));

    }
}