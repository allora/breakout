use crate::game_objects::{Ball, Paddle, Block};
use crate::config::ArenaConfig;
use crate::breakout::PauseState;
use crate::util::*;

use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage, World},
};

/// This system is responsible for properly bouncing the ball off various surfaces
#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Block>,
        Read<'s, ArenaConfig>,
        Read<'s, PauseState>,
    );

    fn run(&mut self, (mut balls, transforms, paddles, blocks, arena_config, pause_state): Self::SystemData) {
        if pause_state.paused {
            return;
        }

        // Iterate over all balls and test them for collisions
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            let (arena_width, arena_height) = (arena_config.width, arena_config.height);

            // Bounce at the top of the arena.
            if ball_y >= arena_height - ball.radius && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            }

            // Boiunce off the sides
            if (ball_x >= arena_width - ball.radius && ball.velocity[0] > 0.0)
                || (ball_x <= ball.radius && ball.velocity[0] < 0.0)
            {
                ball.velocity[0] = -ball.velocity[0];
            }

            // Bounce at the paddle.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x;
                let paddle_y = paddle_transform.translation().y;

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - (paddle.width * 0.5) - ball.radius,
                    paddle_y - (paddle.height * 0.5) - ball.radius,
                    paddle_x + (paddle.width * 0.5) + ball.radius,
                    paddle_y + (paddle.height * 0.5) + ball.radius,
                ) {
                    if ball.velocity[1] < 0.0
                    {
                        ball.velocity[1] = -ball.velocity[1];
                    }
                }
            }

            // bounce off a block
            for (block, block_transform) in (&blocks, &transforms).join() {
                let block_x = block_transform.translation().x;
                let block_y = block_transform.translation().y;

                // TODO: This doesnt cover the case where the ball has moved too far in a single
                // frame thus completely missing the block. This also is not super accurate due
                // to block adjecencies, sometimes the wrong block is bounced off of
                if point_in_rect(
                    block_x,
                    block_y,
                    ball_x - (block.width * 0.5) - ball.radius,
                    ball_y - (block.height * 0.5) - ball.radius,
                    ball_x + (block.width * 0.5) + ball.radius,
                    ball_y + (block.height * 0.5) + ball.radius,
                ) {
                    // Test vertical parallel
                    if is_vector_parallel(
                        // top of block
                        block_x,
                        block_y + block.height * 0.5,
                        // bottom of block
                        block_x,
                        block_y - block.height * 0.5,
                        // last ball pos
                        ball_x - ball.velocity[0],
                        ball_y - ball.velocity[1],
                        //cur ball pos
                        ball_x,
                        ball_y,
                    ) {
                        // bounce vertically
                        ball.velocity[1] = -ball.velocity[1];
                    }

                    // Test horizontal parallel
                    else if is_vector_parallel(
                        // left of block
                        block_x - block.width * 0.5,
                        block_y,
                        // right of block
                        block_x + block.width * 0.5,
                        block_y,
                        // last ball pos
                        ball_x - ball.velocity[0],
                        ball_y - ball.velocity[1],
                        //cur ball pos
                        ball_x,
                        ball_y,
                    ) {
                        // bounce horizontally
                        ball.velocity[0] = -ball.velocity[0];
                    }

                    // Test top line intersection
                    else if is_line_intersected(
                        // left of block
                        block_x - block.width * 0.5 - ball.radius,
                        block_y + block.height * 0.5 + ball.radius,
                        // right of block
                        block_x + block.width * 0.5 + ball.radius,
                        block_y + block.height * 0.5 + ball.radius,
                        // last ball pos
                        ball_x - ball.velocity[0],
                        ball_y - ball.velocity[1],
                        //cur ball pos
                        ball_x,
                        ball_y,
                    ) {
                        // bounce vertically
                        ball.velocity[1] = -ball.velocity[1];
                    }

                    // Test bottom line intersection
                    else if is_line_intersected(
                        // left of block
                        block_x - block.width * 0.5 - ball.radius,
                        block_y - block.height * 0.5 - ball.radius,
                        // right of block
                        block_x + block.width * 0.5 + ball.radius,
                        block_y - block.height * 0.5 - ball.radius,
                        // last ball pos
                        ball_x - ball.velocity[0],
                        ball_y - ball.velocity[1],
                        //cur ball pos
                        ball_x,
                        ball_y,
                    ) {
                        // bounce vertically
                        ball.velocity[1] = -ball.velocity[1];
                    } else
                    {
                        // We dont have to test the rest. If we didnt cross top or bottom bounds,
                        // we crossed the sides and thus bounce horizontally
                        ball.velocity[0] = -ball.velocity[0];
                    }

                    // The ball bounced off a block already, dont need to test other blocks
                    break;
                }
            }
        }
    }
}

