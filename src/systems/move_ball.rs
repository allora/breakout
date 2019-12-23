use crate::game_objects::{Ball,Paddle};

use amethyst::{
    core::{Time, Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage, World},
    input::{InputHandler, StringBindings},
};

/// This system is responsible for moving all the paddles according to the user
/// provided input.
#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut transforms, paddles, input, time): Self::SystemData) {
        let mut paddle_x = 0.0;
        let mut paddle_y = 0.0;
        for (_, paddle_transform) in (&paddles, &transforms).join() {
            paddle_x = paddle_transform.translation().x;
            paddle_y = paddle_transform.translation().y;
        }

        // Iterate over all balls and bounce them.
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let opt_launch = input.action_is_down("launch_ball").unwrap_or(false);

            if !ball.has_launched {
                if opt_launch {
                    println!("Launch Ball!");
                    ball.velocity = [300.0, 300.0];
                    ball.has_launched = true;
                }
                else {
                    transform.set_translation_x(paddle_x);
                    transform.set_translation_y(paddle_y + ball.radius);
                }
            }
            if ball.has_launched {
                transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
                transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());

                let ball_y = transform.translation().y;

                if ball_y < ball.radius {
                    ball.velocity = [0.0, 0.0];
                    ball.has_launched = false;
                    println!("Died!");
                }
            }

        }
    }
}