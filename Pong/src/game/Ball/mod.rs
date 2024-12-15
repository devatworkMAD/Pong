use bevy::app::Plugin;
use bevy::prelude::*;

mod components;
mod systems;
use systems::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        println!("build BallPlugin");
        app
            .add_plugins(#[cfg(not(target_arch = "wasm32"))]
                         Wireframe2dPlugin)
            .add_systems(Startup, (
                spawn_ball,
                setup_BallSpeed
            ))
            .add_systems(Update,(
                move_ball,
                gravity,
                hit_by_player,
                confine_ball
                ));
    }
}