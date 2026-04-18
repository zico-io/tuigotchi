use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tuigotchi_core::action::ALL_ACTIONS;

use crate::{app::App, theme};

use super::shared;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Min(8),    // pet display
            Constraint::Length(8), // stats
            Constraint::Length(3), // actions
            Constraint::Length(2), // status bar
        ])
        .split(frame.area());

    shared::draw_title(frame, chunks[0], app);
    shared::draw_pet(frame, chunks[1], app);
    shared::draw_stats(frame, chunks[2], app);
    draw_actions(frame, chunks[3], app);
    shared::draw_status(frame, chunks[4], app);
}

fn draw_actions(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let items: Vec<Span> = ALL_ACTIONS
        .iter()
        .enumerate()
        .flat_map(|(i, action)| {
            let style = if i == app.selected_action {
                theme::SELECTED_STYLE
            } else {
                ratatui::style::Style::default()
            };
            let prefix = if i == app.selected_action {
                "▸ "
            } else {
                "  "
            };
            vec![Span::styled(format!("{prefix}{}  ", action.label()), style)]
        })
        .collect();

    let paragraph = Paragraph::new(Line::from(items)).block(
        Block::default()
            .title(
                " Actions [←/→ select, Enter perform, Tab explore, i inventory, s skills, q quit] ",
            )
            .title_style(theme::TITLE_STYLE)
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(paragraph, area);
}
