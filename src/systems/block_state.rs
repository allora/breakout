use crate::breakout::PauseState;
use crate::config::BlockConfig;
use crate::game_objects::Block;

use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteStorage},
    renderer::SpriteRender,
};

/// This system is responsible for managing block damage state visuals
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

            // change block sprite index based on damage thresholds
            if (block.max_hits - block.cur_hits) < damage_threshold
                && block.cur_hits != block.max_hits
            {
                block.cur_damage_state = (block.cur_damage_state - 1).max(0);

                let (sprite_index, _) = block_config.damage_states[block.cur_damage_state];
                renderer.sprite_number = sprite_index;
            }
        }
    }
}
