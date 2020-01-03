extern crate amethyst;

use crate::data::ScoreBoard;
use crate::states::MainMenu;
use crate::util::*;

use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    input::is_close_requested,
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiText},
};

const BUTTON_QUIT: &str = "game_quit";
const BUTTON_QUIT_TO_MENU: &str = "level_quit_to_menu";
const TEXT_LEVEL_INDEX: &str = "score_text";

#[derive(Default, Debug)]
pub struct Results {
    ui_root: Option<Entity>,
    button_quit_to_menu: Option<Entity>,
    button_quit_app: Option<Entity>,
    text_score: Option<Entity>,
}

impl SimpleState for Results {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/results.ron", ())));
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
                } else {
                    Trans::None
                }
            }

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_quit_app {
                    Trans::Quit
                } else if Some(target) == self.button_quit_to_menu {
                    Trans::Switch(Box::new(MainMenu::default()))
                } else {
                    Trans::None
                }
            }

            _ => Trans::None,
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_quit_app.is_none()
            || self.button_quit_to_menu.is_none()
            || self.text_score.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_quit_app = ui_finder.find(BUTTON_QUIT);
                self.button_quit_to_menu = ui_finder.find(BUTTON_QUIT_TO_MENU);
                self.text_score = ui_finder.find(TEXT_LEVEL_INDEX);
            });
        }

        let score_board = &world.read_resource::<ScoreBoard>();
        let mut ui_text = world.write_storage::<UiText>();
        {
            if let Some(text) = self
                .text_score
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

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("Failed to remove PauseMenu");
        }
        self.ui_root = None;
        self.button_quit_app = None;
        self.button_quit_to_menu = None;
        self.text_score = None;
    }
}
