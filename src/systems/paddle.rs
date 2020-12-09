use crate::game::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};
use bevy::core::Time;
use bevy::ecs::{Query, Res};
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use bevy::prelude::Transform;

pub fn paddle_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let movement = match paddle.side {
            Side::Left => {
                let mut dir = 0.0;
                if keyboard_input.pressed(KeyCode::W) {
                    dir += 1.0
                }
                if keyboard_input.pressed(KeyCode::S) {
                    dir -= 1.0
                }
                dir
            }
            Side::Right => {
                let mut dir = 0.0;
                if keyboard_input.pressed(KeyCode::Up) {
                    dir += 1.0
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    dir -= 1.0
                }
                dir
            }
        };

        let translation = &mut transform.translation;
        translation.y += time.delta_seconds() * movement * 120.0;
        translation.y = translation
            .y
            .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
            .max(PADDLE_HEIGHT * 0.5);
    }
}
