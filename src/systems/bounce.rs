use bevy::ecs::{Query, Res};
use crate::game::{Ball, Paddle, ARENA_HEIGHT, Side};
use bevy::prelude::Transform;
use crate::audio::{play_bounce_sound, Sounds};
use bevy::audio::Audio;

// TODO explore using collide method
pub fn bounce_system(
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    paddle_query: Query<(&Paddle, &Transform)>
) {
    for (mut ball, ball_transform) in ball_query.iter_mut() {
        let ball_x = ball_transform.translation.x;
        let ball_y = ball_transform.translation.y;

        // Bounce at the top or the bottom of the arena.
        if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
            || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
        {
            ball.velocity[1] = -ball.velocity[1];
            play_bounce_sound(&audio, &sounds);
        }

        for (paddle, paddle_transform) in paddle_query.iter() {
            let paddle_x = paddle_transform.translation.x - (paddle.width * 0.5);
            let paddle_y = paddle_transform.translation.y - (paddle.height * 0.5);

            // To determine whether the ball has collided with a paddle, we create a larger
            // rectangle around the current one, by subtracting the ball radius from the
            // lowest coordinates, and adding the ball radius to the highest ones. The ball
            // is then within the paddle if its center is within the larger wrapper
            // rectangle.
            if point_in_rect(
                ball_x,
                ball_y,
                paddle_x - ball.radius,
                paddle_y - ball.radius,
                paddle_x + paddle.width + ball.radius,
                paddle_y + paddle.height + ball.radius,
            ) {
                if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                    || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                {
                    ball.velocity[0] = -ball.velocity[0];
                    play_bounce_sound(&audio, &sounds);
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
