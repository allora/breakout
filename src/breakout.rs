use crate::{game_objects::*};
use crate::config::{PaddleConfig, ArenaConfig, BallConfig};

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{transform::Transform},
    ecs::prelude::World,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Default)]
pub struct Breakout {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Breakout {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialise_paddle(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_ball(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();

    let (arena_height, arena_width) = {
        let config = world.read_resource::<ArenaConfig>();
        (config.height, config.width)
    };

    transform.set_translation_xyz(arena_width * 0.5, arena_height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(arena_width, arena_height))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/breakout_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/breakout_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddle(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    // Load configs
    let (
        paddle_height,
        paddle_width,
        paddle_velocity,
    ) = {
        let config = world.read_resource::<PaddleConfig>();
        (
            config.height,
            config.width,
            config.velocity,
        )
    };

    let (arena_width, arena_paddle_pos) = {
        let config = world.read_resource::<ArenaConfig>();
        (config.width, config.paddlepos)
    };

    // Correctly position the paddles.
    let y = arena_paddle_pos;
    transform.set_translation_xyz((paddle_width * 0.5) + (arena_width * 0.5), y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // Create a paddle entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle {
            velocity: paddle_velocity,
            width: paddle_width,
            height: paddle_height,
        })
        .with(transform)
        .build();
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    // Load configs
    let ball_radius = {
        let config = world.read_resource::<BallConfig>();
        config.radius
    };

    let (arena_width, arena_paddle_pos) = {
        let config = world.read_resource::<ArenaConfig>();
        (config.width, config.paddlepos)
    };

    // Correctly position the paddles.
    let y = arena_paddle_pos + ball_radius;
    transform.set_translation_xyz(ball_radius + (arena_width * 0.5), y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1, // paddle is the first sprite in the sprite_sheet
    };

    // Create a paddle entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Ball {
            radius: ball_radius,
            has_launched: false,
            velocity: [0.0, 0.0],
        })
        .with(transform)
        .build();
}