use crate::components::{Ball, Paddle};
use crate::data::{LevelInfo, PauseState};

use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{
        Join, Read, ReadStorage, ResourceId, System, SystemData, World, Write, WriteStorage,
    },
    input::{InputHandler, StringBindings},
};

#[derive(SystemData)]
pub struct MoveBallSystemData<'s> {
    pub balls: WriteStorage<'s, Ball>,
    pub transforms: WriteStorage<'s, Transform>,
    pub paddles: ReadStorage<'s, Paddle>,
    pub input: Read<'s, InputHandler<StringBindings>>,
    pub time: Read<'s, Time>,
    pub pause_state: Read<'s, PauseState>,
    pub level_info: Write<'s, LevelInfo>,
}

/// This system is responsible for moving all the balls
#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = MoveBallSystemData<'s>;

    fn run(&mut self, system_data: Self::SystemData) {
        let MoveBallSystemData {
            mut balls,
            mut transforms,
            paddles,
            input,
            time,
            pause_state,
            mut level_info,
        } = system_data;

        if pause_state.paused {
            return;
        }

        let mut paddle_x = 0.0;
        let mut paddle_y = 0.0;
        for (_, paddle_transform) in (&paddles, &transforms).join() {
            paddle_x = paddle_transform.translation().x;
            paddle_y = paddle_transform.translation().y;
        }

        // Iterate over all balls and move them according to their velocity.
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let opt_launch = input.action_is_down("launch_ball").unwrap_or(false);

            ball.last_position.x = transform.translation().x;
            ball.last_position.y = transform.translation().y;

            if !ball.has_launched {
                if opt_launch {
                    println!("Launch Ball!");
                    ball.velocity.x = 300.0;
                    ball.velocity.y = 300.0;
                    ball.has_launched = true;
                } else {
                    transform.set_translation_x(paddle_x);
                    transform.set_translation_y(paddle_y + ball.radius);
                }
            }

            if ball.has_launched {
                transform.prepend_translation_x(ball.velocity.x * time.delta_seconds());
                transform.prepend_translation_y(ball.velocity.y * time.delta_seconds());

                let ball_y = transform.translation().y;

                if ball_y < ball.radius {
                    level_info.num_lives_remaining = (level_info.num_lives_remaining - 1).max(0);
                    ball.velocity.x = 0.0;
                    ball.velocity.y = 0.0;
                    ball.has_launched = false;
                    println!("Died!");
                }
            }
        }
    }
}
