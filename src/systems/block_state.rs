use crate::components::Block;
use crate::config::BlockConfig;
use crate::data::PauseState;

use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, ResourceId, World, WriteStorage},
    renderer::SpriteRender,
};

#[derive(SystemData)]
pub struct BlockStateSystemData<'s> {
    pub blocks: WriteStorage<'s, Block>,
    pub renderers: WriteStorage<'s, SpriteRender>,
    pub pause_state: Read<'s, PauseState>,
    pub block_config: Read<'s, BlockConfig>,
}

/// This system is responsible for managing block damage state visuals
#[derive(SystemDesc)]
pub struct BlockStateSystem;

impl<'s> System<'s> for BlockStateSystem {
    type SystemData = BlockStateSystemData<'s>;

    fn run(&mut self, system_data: Self::SystemData) {
        let BlockStateSystemData {
            mut blocks,
            mut renderers,
            pause_state,
            block_config
        } = system_data;

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
