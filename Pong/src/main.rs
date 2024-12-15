mod systems;
mod game;

use bevy::prelude::*;
use crate::game::GamePlugin;
use crate::systems::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera
            ))
        .add_plugins(GamePlugin)
        .run();
}