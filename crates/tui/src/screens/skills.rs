use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tuigotchi_core::skills::skill::{self, SkillCategory, SkillEffect, ALL_SKILL_IDS};

use crate::{app::App, theme};

use super::shared;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Min(10),   // skill columns + details
            Constraint::Length(3), // controls
            Constraint::Length(2), // status bar
        ])
        .split(frame.area());

    // Title
    let pts = app.skill_tree.available_points();
    let title_block = Block::default()
        .title(format!(" Skill Tree  ({pts} points available) "))
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    frame.render_widget(title_block, chunks[0]);

    // Split middle into three category columns + detail panel
    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    draw_category_column(frame, mid_chunks[0], app, SkillCategory::Combat);
    draw_category_column(frame, mid_chunks[1], app, SkillCategory::Survival);
    draw_category_column(frame, mid_chunks[2], app, SkillCategory::Fortune);
    draw_skill_details(frame, mid_chunks[3], app);

    // Controls
    let controls = Paragraph::new(" j/k: navigate  Enter: allocate point  s/Esc: close").block(
        Block::default()
            .title(" Controls ")
            .title_style(theme::TITLE_STYLE)
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(controls, chunks[2]);

    // Status
    shared::draw_status(frame, chunks[3], app);
}

fn category_color(cat: SkillCategory) -> Color {
    #[allow(unreachable_patterns)]
    match cat {
        SkillCategory::Combat => theme::COMBAT_SKILL_COLOR,
        SkillCategory::Survival => theme::SURVIVAL_SKILL_COLOR,
        SkillCategory::Fortune => theme::FORTUNE_SKILL_COLOR,
        _ => Color::White,
    }
}

fn draw_category_column(frame: &mut Frame, area: Rect, app: &App, category: SkillCategory) {
    let color = category_color(category);
    let block = Block::default()
        .title(format!(" {} ", category.label()))
        .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let all_defs = skill::all_skills();
    let cat_skills: Vec<_> = all_defs.iter().filter(|d| d.category == category).collect();

    let lines: Vec<Line> = cat_skills
        .iter()
        .map(|def| {
            let global_idx = ALL_SKILL_IDS
                .iter()
                .position(|id| *id == def.id)
                .unwrap_or(0);
            let rank = app.skill_tree.rank(def.id);
            let is_selected = global_idx == app.skill_cursor;

            // Check prerequisites
            let prereqs_met = def
                .prerequisites
                .iter()
                .all(|p| app.skill_tree.rank(*p) > 0);

            let cursor = if is_selected { ">" } else { " " };

            let style = if !prereqs_met {
                Style::default().fg(theme::LOCKED_SKILL_COLOR)
            } else if is_selected {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else if rank > 0 {
                Style::default().fg(color)
            } else {
                Style::default().fg(Color::White)
            };

            Line::from(Span::styled(
                format!("{} {} {}/{}", cursor, def.name, rank, def.max_rank),
                style,
            ))
        })
        .collect();

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn draw_skill_details(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Details ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.skill_cursor >= ALL_SKILL_IDS.len() {
        return;
    }

    let id = ALL_SKILL_IDS[app.skill_cursor];
    let Some(def) = skill::skill_def(id) else {
        return;
    };

    let rank = app.skill_tree.rank(id);
    let cat_color = category_color(def.category);

    let mut lines = vec![
        Line::from(Span::styled(
            def.name,
            Style::default().fg(cat_color).add_modifier(Modifier::BOLD),
        )),
        Line::from(def.description),
        Line::from(format!("Rank: {}/{}", rank, def.max_rank)),
        Line::from(""),
    ];

    // Effects per rank
    lines.push(Line::from(Span::styled(
        "Per rank:",
        Style::default().add_modifier(Modifier::UNDERLINED),
    )));
    for effect in &def.effects_per_rank {
        let desc = format_effect(effect);
        lines.push(Line::from(format!("  {desc}")));
    }

    // Prerequisites
    if !def.prerequisites.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Requires:",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )));
        for prereq_id in &def.prerequisites {
            let met = app.skill_tree.rank(*prereq_id) > 0;
            let name = skill::skill_def(*prereq_id)
                .map(|d| d.name.to_string())
                .unwrap_or_else(|| format!("{:?}", prereq_id));
            let style = if met {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(theme::LOCKED_SKILL_COLOR)
            };
            let check = if met { "[x]" } else { "[ ]" };
            lines.push(Line::from(Span::styled(format!("  {check} {name}"), style)));
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

#[allow(unreachable_patterns)]
fn format_effect(effect: &SkillEffect) -> String {
    match effect {
        SkillEffect::CombatStatBonus { stat, value } => {
            format!("+{value:.0} {}", stat.to_uppercase())
        }
        SkillEffect::DecayRateModifier { stat, multiplier } => {
            let pct = ((1.0 - multiplier) * 100.0).round();
            format!("{stat} decays {pct:.0}% slower")
        }
        SkillEffect::LootChanceBonus { bonus } => {
            format!("+{:.0}% loot chance", bonus * 100.0)
        }
        SkillEffect::XpBonus { multiplier } => {
            let pct = ((multiplier - 1.0) * 100.0).round();
            format!("+{pct:.0}% XP")
        }
        _ => "???".into(),
    }
}
