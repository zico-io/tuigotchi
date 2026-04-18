use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use tuigotchi_core::combat::manual_combat::ALL_COMBAT_ACTIONS;

use crate::{app::App, theme};

use super::shared;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // boss HP
            Constraint::Min(8),    // boss ASCII art + pet HP
            Constraint::Length(7), // battle log
            Constraint::Length(3), // action selector
            Constraint::Length(2), // status bar
        ])
        .split(frame.area());

    if let Some(ref encounter) = app.boss_encounter {
        // Boss HP bar
        let boss_hp_ratio = if encounter.boss_max_hp > 0.0 {
            (encounter.boss_hp as f64 / encounter.boss_max_hp as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let boss_hp_gauge = Gauge::default()
            .label(format!(
                "{} HP: {:.0}/{:.0}",
                encounter.boss.enemy.name, encounter.boss_hp, encounter.boss_max_hp,
            ))
            .ratio(boss_hp_ratio)
            .gauge_style(Style::default().fg(theme::BOSS_COLOR))
            .block(
                Block::default()
                    .title(format!(" BOSS: {} ", encounter.boss.enemy.name))
                    .title_style(
                        Style::default()
                            .fg(theme::BOSS_COLOR)
                            .add_modifier(Modifier::BOLD),
                    )
                    .borders(Borders::ALL)
                    .border_style(theme::BOSS_BORDER),
            );
        frame.render_widget(boss_hp_gauge, chunks[0]);

        // Boss art + Pet HP
        let mid_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Boss ASCII art
        let boss_art = r#"
    /\  /\
   ( @  @ )
    > <> <
   /|    |\
  / |    | \"#;
        let boss_paragraph = Paragraph::new(boss_art)
            .style(Style::default().fg(theme::BOSS_COLOR))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme::BOSS_BORDER),
            );
        frame.render_widget(boss_paragraph, mid_chunks[0]);

        // Pet HP
        let pet_hp_ratio = if encounter.pet_max_hp > 0.0 {
            (encounter.pet_hp as f64 / encounter.pet_max_hp as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let pet_info = vec![
            Line::from(Span::styled(
                app.pet.name.to_string(),
                Style::default()
                    .fg(theme::PET_COMBAT_COLOR)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!(
                "HP: {:.0}/{:.0}",
                encounter.pet_hp, encounter.pet_max_hp,
            )),
            Line::from(format!(
                "ATK: {:.0}  DEF: {:.0}",
                encounter.pet_stats.attack, encounter.pet_stats.defense,
            )),
            Line::from(format!("Turn: {}", encounter.turn)),
        ];
        let pet_block = Block::default()
            .title(" Your Pet ")
            .title_style(
                Style::default()
                    .fg(theme::PET_COMBAT_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::PET_COMBAT_COLOR));
        let pet_inner = pet_block.inner(mid_chunks[1]);
        frame.render_widget(pet_block, mid_chunks[1]);

        let pet_paragraph = Paragraph::new(pet_info);
        frame.render_widget(pet_paragraph, pet_inner);

        // Pet HP gauge
        let gauge_area = Rect {
            x: pet_inner.x,
            y: pet_inner.y + pet_inner.height.saturating_sub(1),
            width: pet_inner.width,
            height: 1,
        };
        let pet_gauge = Gauge::default()
            .label(format!("{:.0}%", pet_hp_ratio * 100.0))
            .ratio(pet_hp_ratio)
            .gauge_style(Style::default().fg(theme::PET_COMBAT_COLOR));
        frame.render_widget(pet_gauge, gauge_area);

        // Battle log (last 5 entries)
        let log_lines: Vec<Line> = encounter
            .log
            .iter()
            .rev()
            .take(5)
            .rev()
            .map(|entry| {
                Line::from(Span::styled(
                    format!("  {entry}"),
                    Style::default().fg(theme::BATTLE_LOG_COLOR),
                ))
            })
            .collect();

        let log_paragraph = Paragraph::new(log_lines).block(
            Block::default()
                .title(" Battle Log ")
                .title_style(theme::TITLE_STYLE)
                .borders(Borders::ALL)
                .border_style(theme::BORDER_STYLE),
        );
        frame.render_widget(log_paragraph, chunks[2]);

        // Action selector
        let items: Vec<Span> = ALL_COMBAT_ACTIONS
            .iter()
            .enumerate()
            .flat_map(|(i, action)| {
                let style = if i == app.boss_action_cursor {
                    theme::SELECTED_STYLE
                } else {
                    Style::default()
                };
                let prefix = if i == app.boss_action_cursor {
                    "\u{25b8} "
                } else {
                    "  "
                };
                vec![Span::styled(format!("{prefix}{}  ", action.label()), style)]
            })
            .collect();

        let action_paragraph = Paragraph::new(Line::from(items)).block(
            Block::default()
                .title(" Actions [\u{2190}/\u{2192} select, Enter perform, q quit] ")
                .title_style(theme::TITLE_STYLE)
                .borders(Borders::ALL)
                .border_style(theme::BORDER_STYLE),
        );
        frame.render_widget(action_paragraph, chunks[3]);
    } else {
        // No encounter, shouldn't happen but handle gracefully
        let paragraph = Paragraph::new("No boss encounter active...")
            .style(Style::default().fg(ratatui::style::Color::DarkGray));
        frame.render_widget(paragraph, chunks[1]);
    }

    shared::draw_status(frame, chunks[4], app);
}
