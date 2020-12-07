use bevy::ecs::{Res, Query};
use bevy::core::Time;
use crate::game::Ball;
use bevy::prelude::Transform;

pub fn move_balls_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += ball.velocity[0] * time.delta_seconds();
        translation.y += ball.velocity[1] * time.delta_seconds();
    }
}
