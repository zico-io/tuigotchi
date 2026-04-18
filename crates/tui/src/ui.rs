use tuigotchi_core::game_state::GameMode;

use crate::app::{App, Screen};
use crate::screens;

pub fn draw(frame: &mut ratatui::Frame, app: &App) {
    match app.screen {
        Screen::Main =>
        {
            #[allow(unreachable_patterns)]
            match app.game_mode {
                GameMode::Camp => screens::camp::draw(frame, app),
                GameMode::Explore | GameMode::BossFight => screens::explore::draw(frame, app),
                _ => screens::camp::draw(frame, app),
            }
        }
        Screen::Inventory => screens::inventory::draw(frame, app),
        Screen::BossFight => screens::boss_fight::draw(frame, app),
    }
}
