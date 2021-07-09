use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
    assets::AssetStorage,
    audio::{output::Output, Source},
};

use crate::pong::{Ball, ScoreText, ScoreBoard, ARENA_WIDTH, ARENA_HEIGHT};
use crate::audio::{play_score_sound, Sounds};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem{

    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>
    );

    fn run(&mut self, (
        mut balls, 
        mut transforms, 
        mut ui_text, 
        mut score_board, 
        score_text,
        storage,
        sounds,
        audio_output,
    ): Self::SystemData) {  
        for (ball, local_transform) in (&mut balls, &mut transforms).join() {
            let ball_x = local_transform.translation().x;

            let did_hit = if ball_x < 0.0 {
                score_board.score_right = (score_board.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = score_board.score_right.to_string();
                }
                true
            } else if ball_x > ARENA_WIDTH {
                score_board.score_left = (score_board.score_left + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = score_board.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                local_transform.set_translation_x(ARENA_WIDTH * 0.5);
                local_transform.set_translation_y(ARENA_HEIGHT * 0.5);
                
                play_score_sound(&*sounds, &storage, audio_output.as_deref());

                println!(
                    "Score: | {:^3} | {:^3} |", 
                    score_board.score_left, score_board.score_right
                );
            }
        }
    }

}
