use crate::components::{Ball, Block, Paddle};
use crate::config::ArenaConfig;
use crate::data::PauseState;
use crate::util::*;

use amethyst::{
    core::{math::*, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, ResourceId, System, SystemData, World, WriteStorage},
};

#[derive(SystemData)]
pub struct BounceSystemData<'s> {
    pub balls: WriteStorage<'s, Ball>,
    pub transforms: ReadStorage<'s, Transform>,
    pub paddles: ReadStorage<'s, Paddle>,
    pub blocks: ReadStorage<'s, Block>,
    pub arena_config: Read<'s, ArenaConfig>,
    pub pause_state: Read<'s, PauseState>,
}

/// This system is responsible for properly bouncing the ball off various surfaces
#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = BounceSystemData<'s>;

    fn run(&mut self, system_data: Self::SystemData) {
        let BounceSystemData {
            mut balls,
            transforms,
            paddles,
            blocks,
            arena_config,
            pause_state,
        } = system_data;

        if pause_state.paused {
            return;
        }

        // Iterate over all balls and test them for collisions
        for (ball, transform) in (&mut balls, &transforms).join() {
            let mut ball_has_bounced = false;

            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            let last_ball_pos = ball.last_position;
            let adjusted_ball_pos = {
                let cur_ball_pos = Vector2::new(ball_x, ball_y);

                let ball_move_vector = cur_ball_pos - last_ball_pos;
                let ball_move_adjusted_mag = ball_move_vector.magnitude() + ball.radius;
                let adjusted_ball_vector = ball_move_vector.normalize() * ball_move_adjusted_mag;

                last_ball_pos + adjusted_ball_vector
            };

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
                ) && ball.velocity[1] < 0.0
                {
                    ball.velocity[1] = -ball.velocity[1];
                    ball_has_bounced = true;
                    break;
                }
            }

            if ball_has_bounced {
                break;
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
                    let block_top_right = Vector2::new(
                        block_x + block.width * 0.5 + ball.radius,
                        block_y + block.height * 0.5 + ball.radius,
                    );

                    let block_top_left = Vector2::new(
                        block_x - block.width * 0.5 - ball.radius,
                        block_y + block.height * 0.5 + ball.radius,
                    );

                    let block_bottom_right = Vector2::new(
                        block_x + block.width * 0.5 + ball.radius,
                        block_y - block.height * 0.5 - ball.radius,
                    );

                    let block_bottom_left = Vector2::new(
                        block_x - block.width * 0.5 - ball.radius,
                        block_y - block.height * 0.5 - ball.radius,
                    );

                    // Test vertical parallel
                    if is_vector_parallel(
                        // top of block
                        block_top_right,
                        // bottom of block
                        block_bottom_right,
                        // last ball pos
                        last_ball_pos,
                        //cur ball pos
                        adjusted_ball_pos,
                    ) {
                        // bounce vertically
                        ball.velocity[1] = -ball.velocity[1];
                    }
                    // Test horizontal parallel
                    else if is_vector_parallel(
                        // left of block
                        block_top_left,
                        // right of block
                        block_top_right,
                        // last ball pos
                        last_ball_pos,
                        //cur ball pos
                        adjusted_ball_pos,
                    ) {
                        // bounce horizontally
                        ball.velocity[0] = -ball.velocity[0];
                    }
                    // Test top line intersection
                    else if is_line_intersected(
                            // left of block
                            block_top_left,
                            // right of block
                            block_top_right,
                            // last ball pos
                            last_ball_pos,
                            //cur ball pos
                            adjusted_ball_pos,
                        )
                        // Test bottom line intersection
                        || is_line_intersected(
                            // left of block
                            block_bottom_left,
                            // right of block
                            block_bottom_right,
                            // last ball pos
                            last_ball_pos,
                            //cur ball pos
                            adjusted_ball_pos,
                        )
                    {
                        // bounce vertically
                        ball.velocity[1] = -ball.velocity[1];
                    } else {
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
