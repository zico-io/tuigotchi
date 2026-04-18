use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use tuigotchi_core::{
    action::ALL_ACTIONS, combat::manual_combat::ALL_COMBAT_ACTIONS, game_state::GameMode,
    items::item::Rarity, pet::PetStage,
};

use crate::{
    app::{App, Screen},
    theme,
};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Main => draw_main(frame, app),
        Screen::Inventory => draw_inventory(frame, app),
        Screen::BossFight => draw_boss_fight(frame, app),
    }
}

fn draw_main(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title
            Constraint::Min(8),    // pet display
            Constraint::Length(8), // stats
            Constraint::Length(3), // actions / explore panel
            Constraint::Length(2), // status bar
        ])
        .split(frame.area());

    draw_title(frame, chunks[0], app);
    draw_pet(frame, chunks[1], app);
    draw_stats(frame, chunks[2], app);

    #[allow(unreachable_patterns)]
    match app.game_mode {
        GameMode::Camp => draw_actions(frame, chunks[3], app),
        GameMode::Explore => draw_explore(frame, chunks[3], app),
        GameMode::BossFight => draw_explore(frame, chunks[3], app),
        _ => draw_actions(frame, chunks[3], app),
    }

    draw_status(frame, chunks[4], app);
}

fn draw_title(frame: &mut Frame, area: Rect, app: &App) {
    #[allow(unreachable_patterns)]
    let mode_label = match app.game_mode {
        GameMode::Camp => "CAMP",
        GameMode::Explore => "EXPLORE",
        GameMode::BossFight => "BOSS FIGHT",
        _ => "???",
    };

    let needs_care = if app.pet.needs_care {
        " [NEEDS CARE]"
    } else {
        ""
    };

    let level = app.combat_profile.level();
    let inv_count = app.inventory.len();
    let inv_cap = app.inventory.capacity();
    let title = format!(
        " {} — {:?} (age: {}s) [{}] Lv.{} Bag:{}/{}{} ",
        app.pet.name,
        app.pet.stage,
        app.pet.age_seconds,
        mode_label,
        level,
        inv_count,
        inv_cap,
        needs_care,
    );
    let block = Block::default()
        .title(title)
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    frame.render_widget(block, area);
}

fn draw_pet(frame: &mut Frame, area: Rect, app: &App) {
    let art = pet_ascii(&app.pet.stage, app.pet.alive);
    let color = stage_color(&app.pet.stage);
    let paragraph = Paragraph::new(art).style(Style::default().fg(color)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(paragraph, area);
}

fn draw_stats(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Stats ")
        .title_style(theme::TITLE_STYLE)
        .borders(Borders::ALL)
        .border_style(theme::BORDER_STYLE);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let stat_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1), // XP bar
        ])
        .split(inner);

    let stats = [
        ("Hunger", app.pet.stats.hunger, theme::HUNGER_COLOR),
        ("Happy ", app.pet.stats.happiness, theme::HAPPINESS_COLOR),
        ("Health", app.pet.stats.health, theme::HEALTH_COLOR),
        ("Energy", app.pet.stats.energy, theme::ENERGY_COLOR),
    ];

    for (i, (label, value, color)) in stats.iter().enumerate() {
        let gauge = Gauge::default()
            .label(format!("{label}: {value:.0}%"))
            .ratio((*value as f64 / 100.0).clamp(0.0, 1.0))
            .gauge_style(Style::default().fg(*color));
        frame.render_widget(gauge, stat_chunks[i]);
    }

    // XP bar
    let xp = app.combat_profile.xp();
    let xp_next = app.combat_profile.xp_to_next();
    let xp_ratio = if xp_next > 0 {
        (xp as f64 / xp_next as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let xp_gauge = Gauge::default()
        .label(format!(
            "XP: {}/{} (Lv.{})",
            xp,
            xp_next,
            app.combat_profile.level()
        ))
        .ratio(xp_ratio)
        .gauge_style(Style::default().fg(Color::Magenta));
    frame.render_widget(xp_gauge, stat_chunks[4]);
}

fn draw_actions(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<Span> = ALL_ACTIONS
        .iter()
        .enumerate()
        .flat_map(|(i, action)| {
            let style = if i == app.selected_action {
                theme::SELECTED_STYLE
            } else {
                Style::default()
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
            .title(" Actions [←/→ select, Enter perform, Tab explore, i inventory, q quit] ")
            .title_style(theme::TITLE_STYLE)
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(paragraph, area);
}

fn draw_explore(frame: &mut Frame, area: Rect, app: &App) {
    let battle_log = app
        .explore_state
        .last_battle_log
        .as_deref()
        .unwrap_or("Exploring...");

    let wins = app.explore_state.battles_won;
    let text = Line::from(vec![
        Span::styled(
            format!("W:{wins} "),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            battle_log,
            Style::default()
                .fg(Color::Yellow)
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

fn draw_inventory(frame: &mut Frame, app: &App) {
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
    draw_status(frame, chunks[3], app);
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
            let color = rarity_color(&item.rarity);
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
    let rarity_col = rarity_color(&item.rarity);

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

fn draw_boss_fight(frame: &mut Frame, app: &App) {
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
            .gauge_style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .title(format!(" BOSS: {} ", encounter.boss.enemy.name))
                    .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
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
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
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
                    .fg(Color::Cyan)
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
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let pet_inner = pet_block.inner(mid_chunks[1]);
        frame.render_widget(pet_block, mid_chunks[1]);

        let pet_paragraph = Paragraph::new(pet_info);
        frame.render_widget(pet_paragraph, pet_inner);

        // Pet HP gauge rendered separately in a dedicated area
        // (We'll add it as the last line in the pet panel area)
        let gauge_area = Rect {
            x: pet_inner.x,
            y: pet_inner.y + pet_inner.height.saturating_sub(1),
            width: pet_inner.width,
            height: 1,
        };
        let pet_gauge = Gauge::default()
            .label(format!("{:.0}%", pet_hp_ratio * 100.0))
            .ratio(pet_hp_ratio)
            .gauge_style(Style::default().fg(Color::Cyan));
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
                    Style::default().fg(Color::Yellow),
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
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
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
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(paragraph, chunks[1]);
    }

    draw_status(frame, chunks[4], app);
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let msg = app
        .status_message
        .as_deref()
        .unwrap_or("Take care of your pet!");
    let paragraph = Paragraph::new(msg).style(theme::STATUS_STYLE);
    frame.render_widget(paragraph, area);
}

fn rarity_color(rarity: &Rarity) -> Color {
    match rarity {
        Rarity::Common => theme::COMMON_COLOR,
        Rarity::Uncommon => theme::UNCOMMON_COLOR,
        Rarity::Rare => theme::RARE_COLOR,
        _ => Color::White,
    }
}

fn pet_ascii(stage: &PetStage, alive: bool) -> &'static str {
    if !alive {
        return r#"
      ___
     | R |
     | I |
     | P |
     |___|
    "#;
    }

    match stage {
        PetStage::Egg => {
            r#"
       ___
      /   \
     | . . |
      \___/
    "#
        }
        PetStage::Baby => {
            r#"
      ^___^
     ( o.o )
      > ^ <
    "#
        }
        PetStage::Teen => {
            r#"
      /\_/\
     ( o.o )
      > ^ <
     /|   |\
    "#
        }
        PetStage::Adult => {
            r#"
       /\_/\
      ( ^.^ )
     />   <\
     |  |  |
     _/   \_
    "#
        }
        PetStage::Elder => {
            r#"
       /\_/\
      ( -.- )
     />   <\
     | ~~~ |
     _/   \_
    "#
        }
    }
}

#[allow(dead_code)]
fn stage_color(stage: &PetStage) -> ratatui::style::Color {
    match stage {
        PetStage::Egg => theme::EGG_COLOR,
        PetStage::Baby => theme::BABY_COLOR,
        PetStage::Teen => theme::TEEN_COLOR,
        PetStage::Adult => theme::ADULT_COLOR,
        PetStage::Elder => theme::ELDER_COLOR,
    }
}
