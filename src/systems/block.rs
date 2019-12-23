use crate::game_objects::{Ball, Block};

use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, Entities, ReadStorage, System, SystemData, WriteStorage, World},
};

/// This system is responsible for moving all the paddles according to the user
/// provided input.
#[derive(SystemDesc)]
pub struct BlockSystem;

impl<'s> System<'s> for BlockSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Block>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut blocks, balls, transforms): Self::SystemData) {
        // Iterate over all balls and blocks and see if a block loses a hit
        for (ball, transform) in (&balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            for (e, block, block_transform) in (&entities, &mut blocks, &transforms).join() {
                if block.hits == 0 {
                    entities.delete(e);
                }
                else {
                    let block_x = block_transform.translation().x;
                    let block_y = block_transform.translation().y;

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
                        block.hits = block.hits - 1;
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}