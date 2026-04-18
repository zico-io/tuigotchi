use ratatui::style::Color;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
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
            Constraint::Min(10),   // item list + details
            Constraint::Length(3), // controls
            Constraint::Length(2), // status bar
        ])
        .split(frame.area());

    // Title
    let inv_count = app.inventory.len();
    let inv_cap = app.inventory.capacity();
    let title_block = Block::default()
        .title(format!(" Inventory ({inv_count}/{inv_cap}) "))
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    frame.render_widget(title_block, chunks[0]);

    // Split middle area into list and details
    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    draw_inventory_list(frame, mid_chunks[0], app);
    draw_inventory_details(frame, mid_chunks[1], app);

    // Controls
    let controls = Paragraph::new(" j/k: navigate  e: equip  u: unequip  d: discard  i/Esc: close")
        .block(
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

fn draw_inventory_list(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Items ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.inventory.is_empty() {
        let empty = Paragraph::new("  (empty)").style(Style::default().fg(Color::DarkGray));
        frame.render_widget(empty, inner);
        return;
    }

    let items = app.inventory.items();
    // Calculate visible range based on cursor
    let visible_height = inner.height as usize;
    let start = if app.inventory_cursor >= visible_height {
        app.inventory_cursor - visible_height + 1
    } else {
        0
    };
    let end = (start + visible_height).min(items.len());

    let lines: Vec<Line> = items[start..end]
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let idx = start + i;
            let equipped_marker = if app.inventory.is_equipped(idx) {
                "[E] "
            } else {
                "    "
            };
            let cursor = if idx == app.inventory_cursor {
                "▸ "
            } else {
                "  "
            };
            let color = shared::rarity_color(&item.rarity);
            let style = if idx == app.inventory_cursor {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };
            Line::from(vec![Span::styled(
                format!("{cursor}{equipped_marker}{}", item.name),
                style,
            )])
        })
        .collect();

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn draw_inventory_details(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Details ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items = app.inventory.items();
    if items.is_empty() || app.inventory_cursor >= items.len() {
        return;
    }

    let item = &items[app.inventory_cursor];
    let rarity_col = shared::rarity_color(&item.rarity);

    let mut lines = vec![
        Line::from(Span::styled(
            &item.name,
            Style::default().fg(rarity_col).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::raw("Rarity: "),
            Span::styled(item.rarity.label(), Style::default().fg(rarity_col)),
        ]),
        Line::from(format!("Slot:   {}", item.slot.label())),
        Line::from(""),
        Line::from(Span::styled(
            "Modifiers:",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )),
    ];

    for m in &item.modifiers {
        lines.push(Line::from(format!("  +{:.1} {}", m.value, m.stat.label())));
    }

    if app.inventory.is_equipped(app.inventory_cursor) {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "[EQUIPPED]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
