mod game;
mod systems;
mod utils;
mod audio;

use bevy::prelude::*;
use crate::game::{init_game, start_game_system};
use crate::systems::{paddle_system, move_balls_system, bounce_system, winner_system};
use crate::utils::camera_virtual_screen_system;


fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Simple Pong".to_string(),
            width: 800,
            height: 600,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_game)
        .add_system(start_game_system)
        .add_system(camera_virtual_screen_system)
        .add_system(paddle_system)
        .add_system(move_balls_system)
        .add_system(bounce_system)
        .add_system(winner_system)
        .run();
}
