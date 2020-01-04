use crate::components::{Ball, Block};
use crate::data::{LevelInfo, PauseState, ScoreBoard};
use crate::util::point_in_rect;

use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{
        Entities, Join, Read, ReadStorage, ResourceId, System, SystemData, World, Write,
        WriteStorage,
    },
};

#[derive(SystemData)]
pub struct BlockSystemData<'s> {
    pub entities: Entities<'s>,
    pub blocks: WriteStorage<'s, Block>,
    pub balls: ReadStorage<'s, Ball>,
    pub transforms: ReadStorage<'s, Transform>,
    pub pause_state: Read<'s, PauseState>,
    pub score_board: Write<'s, ScoreBoard>,
    pub level_info: Write<'s, LevelInfo>,
}

/// This system is responsible for tracking block health
#[derive(SystemDesc)]
pub struct BlockSystem;

impl<'s> System<'s> for BlockSystem {
    type SystemData = BlockSystemData<'s>;

    fn run(&mut self, system_data: Self::SystemData) {
        let BlockSystemData {
            entities,
            mut blocks,
            balls,
            transforms,
            pause_state,
            mut score_board,
            mut level_info,
        } = system_data;

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
                    level_info.num_blocks_remaining = (level_info.num_blocks_remaining - 1).max(0);
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
