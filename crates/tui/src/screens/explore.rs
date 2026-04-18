use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::App, theme};

use super::shared;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Min(8),    // pet display
            Constraint::Length(8), // stats
            Constraint::Length(3), // explore panel
            Constraint::Length(2), // status bar
        ])
        .split(frame.area());

    shared::draw_title(frame, chunks[0], app);
    shared::draw_pet(frame, chunks[1], app);
    shared::draw_stats(frame, chunks[2], app);
    draw_explore_panel(frame, chunks[3], app);
    shared::draw_status(frame, chunks[4], app);
}

fn draw_explore_panel(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let battle_log = app.explore_state.last_battle_log.as_deref();

    let explore_text = if let Some(log) = battle_log {
        log.to_string()
    } else {
        let dots = ".".repeat((app.explore_tick_count % 3 + 1) as usize);
        format!("Exploring{dots}")
    };

    let wins = app.explore_state.battles_won;
    let text = Line::from(vec![
        Span::styled(format!("W:{wins} "), theme::WIN_COUNT_STYLE),
        Span::styled(
            explore_text,
            Style::default()
                .fg(theme::BATTLE_LOG_COLOR)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(" Explore [Tab return to camp, q quit] ")
            .title_style(theme::TITLE_STYLE)
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(paragraph, area);
}
