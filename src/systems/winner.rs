use crate::audio::{play_score_sound, Sounds};
use crate::game::{Ball, ScoreBoard, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};
use bevy::audio::Audio;
use bevy::ecs::{Query, Res, ResMut};
use bevy::prelude::Transform;
use bevy::ui::widget::Text;

pub fn winner_system(
    audio: Res<Audio>,
    sounds: Res<Sounds>,

    mut scoreboard: ResMut<ScoreBoard>,
    score_text: Res<ScoreText>,
    mut text_query: Query<&mut Text>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
) {
    for (mut ball, mut transform) in ball_query.iter_mut() {
        let ball_x = transform.translation.x;

        let did_hit = if ball_x <= ball.radius {
            // Right player scored on the left side.
            // We top the score at 999 to avoid text overlap.
            scoreboard.score_right = (scoreboard.score_right + 1).min(999);
            if let Ok(mut text) = text_query.get_mut(score_text.p2_score) {
                text.value = scoreboard.score_right.to_string();
            }

            true
        } else if ball_x >= ARENA_WIDTH - ball.radius {
            // Left player scored on the right side.
            // We top the score at 999 to avoid text overlap.
            scoreboard.score_left = (scoreboard.score_left + 1).min(999);
            if let Ok(mut text) = text_query.get_mut(score_text.p1_score) {
                text.value = scoreboard.score_left.to_string();
            }

            true
        } else {
            false
        };

        if did_hit {
            ball.velocity[0] = -ball.velocity[0]; // Reverse Direction

            let translation = &mut transform.translation;
            translation.x = ARENA_WIDTH / 2.0; // Reset Position
            translation.y = ARENA_HEIGHT / 2.0; // Reset Position

            play_score_sound(&audio, &sounds);

            println!(
                "Score: | {:^3} | {:^3} |",
                scoreboard.score_left, scoreboard.score_right
            );
        }
    }
}
