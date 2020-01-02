use crate::components::{Ball, Block};
use crate::data::{PauseState, ScoreBoard};
use crate::util::point_in_rect;

use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{
        Entities, Join, Read, ReadStorage, System, SystemData, World, Write, WriteStorage,
    },
};

/// This system is responsible for tracking block health
#[derive(SystemDesc)]
pub struct BlockSystem;

impl<'s> System<'s> for BlockSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Block>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        Read<'s, PauseState>,
        Write<'s, ScoreBoard>,
    );

    fn run(
        &mut self,
        (entities, mut blocks, balls, transforms, pause_state, mut score_board): Self::SystemData,
    ) {
        if pause_state.paused {
            return;
        }

        // Iterate over all balls and blocks and see if a block loses a hit
        for (ball, transform) in (&balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            for (e, block, block_transform) in (&entities, &mut blocks, &transforms).join() {
                if block.cur_hits >= block.max_hits {
                    score_board.current_score += block.max_hits;
                    entities.delete(e).expect("entity deleted");
                } else {
                    let block_x = block_transform.translation().x;
                    let block_y = block_transform.translation().y;

                    // TODO: This is not super accurate due to block adjecencies, sometimes
                    // the additional blocks' health are decrimented. Might also need to figure
                    // out which block to hit based on velocity direction

                    // To determine whether the ball has collided with a block, we create a larger
                    // rectangle around the current one, by subtracting the ball radius from the
                    // lowest coordinates, and adding the ball radius to the highest ones. The ball
                    // is then within the block if its center is within the larger wrapper
                    // rectangle.
                    if point_in_rect(
                        block_x,
                        block_y,
                        ball_x - (block.width * 0.5) - ball.radius,
                        ball_y - (block.height * 0.5) - ball.radius,
                        ball_x + (block.width * 0.5) + ball.radius,
                        ball_y + (block.height * 0.5) + ball.radius,
                    ) {
                        block.cur_hits += 1;
                    }
                }
            }
        }
    }
}
