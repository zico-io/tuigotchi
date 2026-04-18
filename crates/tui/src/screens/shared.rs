use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use tuigotchi_core::{game_state::GameMode, items::item::Rarity, pet::PetStage};

use crate::{app::App, theme};

pub(super) fn draw_title(frame: &mut Frame, area: Rect, app: &App) {
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

pub(super) fn draw_pet(frame: &mut Frame, area: Rect, app: &App) {
    let art = pet_ascii(&app.pet.stage, app.pet.alive);
    let color = stage_color(&app.pet.stage);
    let paragraph = Paragraph::new(art).style(Style::default().fg(color)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::BORDER_STYLE),
    );
    frame.render_widget(paragraph, area);
}

pub(super) fn draw_stats(frame: &mut Frame, area: Rect, app: &App) {
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

pub(super) fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let msg = app
        .status_message
        .as_deref()
        .unwrap_or("Take care of your pet!");
    let paragraph = Paragraph::new(msg).style(theme::STATUS_STYLE);
    frame.render_widget(paragraph, area);
}

pub(super) fn rarity_color(rarity: &Rarity) -> Color {
    match rarity {
        Rarity::Common => theme::COMMON_COLOR,
        Rarity::Uncommon => theme::UNCOMMON_COLOR,
        Rarity::Rare => theme::RARE_COLOR,
        _ => Color::White,
    }
}

pub(super) fn pet_ascii(stage: &PetStage, alive: bool) -> &'static str {
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
pub(super) fn stage_color(stage: &PetStage) -> Color {
    match stage {
        PetStage::Egg => theme::EGG_COLOR,
        PetStage::Baby => theme::BABY_COLOR,
        PetStage::Teen => theme::TEEN_COLOR,
        PetStage::Adult => theme::ADULT_COLOR,
        PetStage::Elder => theme::ELDER_COLOR,
    }
}
