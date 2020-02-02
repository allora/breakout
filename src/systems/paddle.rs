use crate::components::Paddle;
use crate::config::ArenaConfig;
use crate::data::PauseState;

use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, ResourceId, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemData)]
pub struct PaddleSystemData<'s> {
    pub paddles: ReadStorage<'s, Paddle>,
    pub transforms: WriteStorage<'s, Transform>,
    pub time: Read<'s, Time>,
    pub input: Read<'s, InputHandler<StringBindings>>,
    pub arena_config: Read<'s, ArenaConfig>,
    pub pause_state: Read<'s, PauseState>,
}

/// This system is responsible for moving all the paddles according to the user
/// provided input.
#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = PaddleSystemData<'s>;

    fn run(&mut self, system_data: Self::SystemData) {
        let PaddleSystemData {
            paddles,
            mut transforms,
            time,
            input,
            arena_config,
            pause_state,
        } = system_data;

        if pause_state.paused {
            return;
        }

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
