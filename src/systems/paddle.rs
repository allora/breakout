use crate::game_objects::Paddle;
use crate::config::ArenaConfig;

use amethyst::{
    core::{Time, Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage, World},
    input::{InputHandler, StringBindings},
};

/// This system is responsible for moving all the paddles according to the user
/// provided input.
#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (paddles, mut transforms, time, input, arena_config): Self::SystemData) {
        // Iterate over all paddles and move them according to the input the user
        // provided.
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let opt_movement = input.axis_value("paddle");

            if let Some(movement) = opt_movement {
                let arena_width = arena_config.width;
                let scaled_move = paddle.velocity * time.delta_seconds() * movement as f32;

                transform.prepend_translation_x(scaled_move);

                // We make sure the paddle remains in the arena.
                let paddle_x = transform.translation().x;
                transform.set_translation_x(
                    paddle_x
                        .max(paddle.width * 0.5)
                        .min(arena_width - paddle.width * 0.5),
                );
            }
        }
    }
}