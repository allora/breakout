extern crate amethyst;

use crate::breakout::Breakout;
use crate::config::LevelsConfig;
use std::iter;
use std::cmp;

use amethyst::{
    prelude::*,
    input::{VirtualKeyCode, is_key_down, is_close_requested},
    ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiText},

    core::transform::ParentHierarchy,
    ecs::{
        error::WrongGeneration,
        prelude::{Entity, World, WorldExt},
    },
};

const BUTTON_START: &str = "start";
const BUTTON_LEVEL_UP: &str = "level_up";
const BUTTON_LEVEL_DN: &str = "level_down";
const BUTTON_LEVEL_INDEX: &str = "level_index_text";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_level_up: Option<Entity>,
    button_level_down: Option<Entity>,
    text_level_index: Option<Entity>,
    level_index: usize,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        self.level_index = 0;

        self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/main_menu.ron", ())));
    }

    fn handle_event(&mut self, state_data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    return Trans::Switch(Box::new(Breakout::new(self.level_index)));
                }

                let old_index = self.level_index;

                if Some(target) == self.button_level_down {
                    if self.level_index > 0 {
                        self.level_index = cmp::max(self.level_index - 1, 0);
                    }
                }

                if Some(target) == self.button_level_up {
                    let levels_config = &world.read_resource::<LevelsConfig>().layout;
                    self.level_index = cmp::min(self.level_index + 1, levels_config.len() - 1);
                }

                if old_index != self.level_index {
                    let mut ui_text = world.write_storage::<UiText>();
                    {
                        if let Some(text) = self.text_level_index.and_then(|entity| ui_text.get_mut(entity)) {
                            text.text = self.level_index.to_string();
                        }
                    }
                }

                Trans::None
            }

            _ => Trans::None
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_start.is_none()
            || self.button_level_up.is_none()
            || self.button_level_down.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_level_up = ui_finder.find(BUTTON_LEVEL_UP);
                self.button_level_down = ui_finder.find(BUTTON_LEVEL_DN);
                self.text_level_index = ui_finder.find(BUTTON_LEVEL_INDEX);
            });
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("Failed to remove MainMenu");
        }
        self.ui_root = None;
        self.button_start = None;
        self.button_level_up = None;
        self.button_level_down = None;
        self.text_level_index = None;
        self.level_index = 0;
    }
}

/// delete the specified root entity and all of its descendents as specified
/// by the Parent component and maintained by the ParentHierarchy resource
// from https://github.com/amethyst/evoli src/utils/hierarchy_util.rs
pub fn delete_hierarchy(root: Entity, world: &mut World) -> Result<(), WrongGeneration> {
    let entities = {
        iter::once(root)
            .chain(
                world
                    .read_resource::<ParentHierarchy>()
                    .all_children_iter(root),
            )
            .collect::<Vec<Entity>>()
    };
    world.delete_entities(&entities)
}