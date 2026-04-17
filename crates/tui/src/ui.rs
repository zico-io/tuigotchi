use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use tuigotchi_core::{action::ALL_ACTIONS, game_state::GameMode, pet::PetStage};

use crate::{app::App, theme};

pub fn draw(frame: &mut Frame, app: &App) {
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

    match app.game_mode {
        GameMode::Camp => draw_actions(frame, chunks[3], app),
        GameMode::Explore => draw_explore(frame, chunks[3]),
        _ => draw_actions(frame, chunks[3], app),
    }

    draw_status(frame, chunks[4], app);
}

fn draw_title(frame: &mut Frame, area: Rect, app: &App) {
    let mode_label = match app.game_mode {
        GameMode::Camp => "CAMP",
        GameMode::Explore => "EXPLORE",
        _ => "???",
    };

    let needs_care = if app.pet.needs_care {
        " [NEEDS CARE]"
    } else {
        ""
    };

    let title = format!(
        " {} — {:?} (age: {}s) [{}]{} ",
        app.pet.name, app.pet.stage, app.pet.age_seconds, mode_label, needs_care
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
            .title(" Actions [←/→ select, Enter perform, Tab explore, q quit] ")
            .title_style(theme::TITLE_STYLE)
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(paragraph, area);
}

fn draw_explore(frame: &mut Frame, area: Rect) {
    let text = Line::from(vec![Span::styled(
        "Exploring... (Tab to return to camp)",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )]);

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(" Explore [Tab return to camp, q quit] ")
            .title_style(theme::TITLE_STYLE)
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(paragraph, area);
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let msg = app
        .status_message
        .as_deref()
        .unwrap_or("Take care of your pet!");
    let paragraph = Paragraph::new(msg).style(theme::STATUS_STYLE);
    frame.render_widget(paragraph, area);
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

fn stage_color(stage: &PetStage) -> ratatui::style::Color {
    match stage {
        PetStage::Egg => theme::EGG_COLOR,
        PetStage::Baby => theme::BABY_COLOR,
        PetStage::Teen => theme::TEEN_COLOR,
        PetStage::Adult => theme::ADULT_COLOR,
        PetStage::Elder => theme::ELDER_COLOR,
    }
}
