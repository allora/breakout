use crate::systems::{PaddleSystem, MoveBallSystem, BounceSystem, BlockSystem};

use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

pub struct BreakoutBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for BreakoutBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(PaddleSystem, "paddle_system", &["input_system"]);
        builder.add(MoveBallSystem, "ball_system", &["input_system"]);
        builder.add(
            BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        );
        builder.add(
            BlockSystem,
            "block_system",
            &["ball_system"],
        );
        
        Ok(())
    }
}