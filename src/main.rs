mod game_objects;
mod breakout;
mod bundle;
mod config;
mod systems;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::bundle::BreakoutBundle;
use crate::config::BreakoutConfig;

use std::time::Duration;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use crate::breakout::Breakout;

    // Config setup
    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let assets_dir = app_root.join("assets");

    let breakout_config_path = config_dir.join("breakout.ron");
    let breakout_config = BreakoutConfig::load(&breakout_config_path);

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    // Game Data setup
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(BreakoutBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::build(assets_dir, Breakout::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .with_resource(breakout_config.arena)
        .with_resource(breakout_config.ball)
        .with_resource(breakout_config.block)
        .with_resource(breakout_config.paddle)
        .build(game_data)?;
        
    game.run();

    Ok(())
}
