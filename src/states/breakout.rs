use crate::components::*;
use crate::config::{ArenaConfig, BallConfig, BlockConfig, LevelsConfig, PaddleConfig};
use crate::data::{LevelInfo, PauseState, ScoreBoard};
use crate::states::{PauseMenu, Results};
use crate::util::delete_hierarchy;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Entity, Join},
    ecs::world::EntitiesRes,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{UiCreator, UiFinder, UiText},
};

const TEXT_SCORE: &str = "score_text";
const TEXT_LIVES: &str = "lives_text";

#[derive(Default)]
pub struct Breakout {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    level_index: usize,
    ui_root: Option<Entity>,
    score_text: Option<Entity>,
    lives_text: Option<Entity>,
}

impl Breakout {
    pub fn new(index: usize) -> Self {
        Breakout {
            sprite_sheet_handle: None,
            level_index: index,
            ui_root: None,
            score_text: None,
            lives_text: None,
        }
    }
}

impl SimpleState for Breakout {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/hud.ron", ())));

        world.register::<BreakoutRemovalTag>();

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        // Set initial pause bool
        let pause_state = PauseState { paused: false };

        // Set initial score
        let score_board = ScoreBoard { current_score: 0 };

        // Init block count
        let mut level_info = LevelInfo {
            num_blocks_remaining: 0,
            num_lives_remaining: 3,
        };

        world.insert(pause_state);
        world.insert(score_board);

        initialise_level(
            world,
            self.sprite_sheet_handle.clone().unwrap(),
            self.level_index,
            &mut level_info.num_blocks_remaining,
        );

        world.insert(level_info);

        initialise_ball(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_paddle(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.lives_text.is_none()
        || self.score_text.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.lives_text = ui_finder.find(TEXT_LIVES);
                self.score_text = ui_finder.find(TEXT_SCORE);
            });
        }

        let level_info = &world.read_resource::<LevelInfo>();
        {
            let mut ui_text = world.write_storage::<UiText>();
            {
                if let Some(text) = self
                    .lives_text
                    .and_then(|entity: Entity| ui_text.get_mut(entity))
                {
                    let lives_string = "LIVES: ";

                    text.text = format!(
                        "{}{}",
                        lives_string,
                        level_info.num_lives_remaining.to_string()
                    );
                }
            }
        }

        let score_board = &world.read_resource::<ScoreBoard>();
        {
            let mut ui_text = world.write_storage::<UiText>();
            {
                if let Some(text) = self
                    .score_text
                    .and_then(|entity: Entity| ui_text.get_mut(entity))
                {
                    let score_string = "SCORE: ";

                    text.text = format!(
                        "{}{}",
                        score_string,
                        (score_board.current_score * 100).to_string()
                    );
                }
            }
        }

        if level_info.num_lives_remaining == 0 || level_info.num_blocks_remaining == 0 {
            return Trans::Switch(Box::new(Results::default()));
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Push(Box::new(PauseMenu::default()))
                } else {
                    Trans::None
                }
            }

            _ => Trans::None,
        }
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let pause_state = data.world.try_fetch_mut::<PauseState>();
        if let Some(mut pause_resource) = pause_state {
            pause_resource.paused = true;
        };
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let pause_state = data.world.try_fetch_mut::<PauseState>();
        if let Some(mut pause_resource) = pause_state {
            pause_resource.paused = false;
        };
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // Delete UI
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("Failed to remove MainMenu");
        }
        self.ui_root = None;
        self.score_text = None;
        self.lives_text = None;

        // Delete gameplay entities
        let entities = data.world.read_resource::<EntitiesRes>();
        let removal_tags = data.world.read_storage::<BreakoutRemovalTag>();

        let deletions_successful = (&entities, &removal_tags)
            .join()
            .map(|(entity, _)| entities.delete(entity))
            .all(|x| x.is_ok());

        if !deletions_successful {
            println!("Failed to delete level");
        }
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
        .with(BreakoutRemovalTag)
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
        SpriteSheetFormat(texture_handle),   // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialises the paddle
fn initialise_paddle(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    // Load configs
    let (paddle_height, paddle_width, paddle_velocity) = {
        let config = world.read_resource::<PaddleConfig>();
        (config.height, config.width, config.velocity)
    };

    let (arena_width, arena_paddle_pos) = {
        let config = world.read_resource::<ArenaConfig>();
        (config.width, config.paddlepos)
    };

    // Correctly position the paddle.
    let y = arena_paddle_pos;
    transform.set_translation_xyz((paddle_width * 0.5) + (arena_width * 0.5), y, 0.1);

    // Assign the sprite for the paddle
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // paddle is the first sprite in the sprite sheet
    };

    // Create a paddle entity.
    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle {
            velocity: paddle_velocity,
            width: paddle_width,
            height: paddle_height,
        })
        .with(transform)
        .with(BreakoutRemovalTag)
        .build();
}

/// Initialises the ball
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

    // Correctly position the ball.
    let y = arena_paddle_pos + ball_radius;
    transform.set_translation_xyz(ball_radius + (arena_width * 0.5), y, 0.0);

    // Assign the sprites for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite in the sprite sheet
    };

    // Create a ball entity.
    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: ball_radius,
            has_launched: false,
            velocity: Vector2::new(0.0, 0.0),
            last_position: Vector2::new(0.0, 0.0),
        })
        .with(transform)
        .with(BreakoutRemovalTag)
        .build();
}

/// Initialises a brick
fn initialise_level(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    level_index: usize,
    block_count: &mut i32,
) {
    // Load configs
    let block_positions = {
        let config = world.read_resource::<LevelsConfig>();
        config.layout.to_vec()
    };

    let (block_width, block_height, block_damage_states) = {
        let config = world.read_resource::<BlockConfig>();
        (config.width, config.height, config.damage_states.to_vec())
    };

    let (_, arena_height) = {
        let config = world.read_resource::<ArenaConfig>();
        (config.width, config.height)
    };

    // Assign the sprites for the block
    let mut sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2, // Block is the third sprite
    };

    // Create a block entities.
    for y_pos in 0..block_positions[level_index].len() {
        for x_pos in 0..block_positions[level_index][y_pos].len() {
            if block_positions[level_index][y_pos][x_pos] == 0 {
                continue;
            }

            let mut transform = Transform::default();

            let x = block_width * x_pos as f32;
            let y = block_height * y_pos as f32;

            let hits = block_positions[level_index][y_pos][x_pos];

            transform.set_translation_xyz(
                (block_width * 0.5) + x,
                (arena_height - y) - (block_height * 0.5),
                0.1,
            );

            let block = Block {
                width: block_width,
                height: block_height,
                max_hits: hits,
                cur_hits: 0,
                cur_damage_state: (hits - 1) as usize,
            };

            sprite_render.sprite_number = block_damage_states[block.cur_damage_state].0;

            // increment block counter
            *block_count += 1;

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(block)
                .with(transform)
                .with(BreakoutRemovalTag)
                .build();
        }
    }
}
