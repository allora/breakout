use crate::game_objects::{Block};
use crate::breakout::PauseState;
use crate::config::BlockConfig;

use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    renderer::SpriteRender,
    ecs::prelude::{Join, System, Read, SystemData, WriteStorage, World},
};

/// This system is responsible for moving all the paddles according to the user
/// provided input.
#[derive(SystemDesc)]
pub struct BlockStateSystem;

impl<'s> System<'s> for BlockStateSystem {
    type SystemData = (
        WriteStorage<'s, Block>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, PauseState>,
        Read<'s, BlockConfig>,
    );

    fn run(&mut self, (mut blocks, mut renderers, pause_state, block_config): Self::SystemData) {
        if pause_state.paused {
            return;
        }

        for (block, renderer) in (&mut blocks, &mut renderers).join() {
            let (_, damage_threshold) = block_config.damage_states[block.cur_damage_state];
            
            if (block.max_hits - block.cur_hits) < damage_threshold 
                && block.cur_hits != block.max_hits {
                block.cur_damage_state = (block.cur_damage_state - 1).max(0);

                let (sprite_index, _) = block_config.damage_states[block.cur_damage_state];
                renderer.sprite_number = sprite_index;
            } 
        }
    }
}