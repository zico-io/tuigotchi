use ratatui::style::{Color, Modifier, Style};

// Pet stage colors
pub const EGG_COLOR: Color = Color::White;
pub const BABY_COLOR: Color = Color::Cyan;
pub const TEEN_COLOR: Color = Color::Green;
pub const ADULT_COLOR: Color = Color::Yellow;
pub const ELDER_COLOR: Color = Color::Magenta;

// Stat bar colors
pub const HUNGER_COLOR: Color = Color::Red;
pub const HAPPINESS_COLOR: Color = Color::Yellow;
pub const HEALTH_COLOR: Color = Color::Green;
pub const ENERGY_COLOR: Color = Color::Blue;

// UI chrome
pub const BORDER_STYLE: Style = Style::new().fg(Color::DarkGray);
pub const TITLE_STYLE: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
pub const SELECTED_STYLE: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
pub const STATUS_STYLE: Style = Style::new()
    .fg(Color::DarkGray)
    .add_modifier(Modifier::ITALIC);

// Rarity colors
pub const COMMON_COLOR: Color = Color::White;
pub const UNCOMMON_COLOR: Color = Color::Green;
pub const RARE_COLOR: Color = Color::Blue;

// Combat colors
pub const BOSS_COLOR: Color = Color::Red;
pub const BOSS_BORDER: Style = Style::new().fg(Color::Red);
pub const PET_COMBAT_COLOR: Color = Color::Cyan;

// Level-up highlight
#[allow(dead_code)]
pub const LEVEL_UP_STYLE: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);

// Battle log
pub const BATTLE_LOG_COLOR: Color = Color::Yellow;

// Explore
pub const WIN_COUNT_STYLE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
