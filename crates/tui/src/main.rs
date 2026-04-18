mod app;
mod screens;
mod theme;
mod ui;

use std::{
    io,
    path::PathBuf,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tuigotchi_core::save;

use app::{App, Screen};

const TICK_RATE: Duration = Duration::from_secs(1);

fn save_path() -> PathBuf {
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tuigotchi");
    dir.join("save.json")
}

fn main() -> io::Result<()> {
    // Install panic hook that restores terminal before printing panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));

    setup_terminal()?;
    let result = run();
    restore_terminal()?;
    result
}

fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn run() -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let path = save_path();
    let mut app = match save::load(&path) {
        Ok(data) => App::from_save(data, path),
        Err(_) => App::new("Wobble", path),
    };

    let mut last_tick = Instant::now();

    while app.running {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.screen {
                        Screen::Inventory => match key.code {
                            KeyCode::Char('i') | KeyCode::Esc => app.toggle_inventory(),
                            KeyCode::Char('j') | KeyCode::Down => app.inventory_next(),
                            KeyCode::Char('k') | KeyCode::Up => app.inventory_prev(),
                            KeyCode::Char('e') | KeyCode::Enter => app.inventory_equip(),
                            KeyCode::Char('u') => app.inventory_unequip(),
                            KeyCode::Char('d') => app.inventory_discard(),
                            KeyCode::Char('q') => app.quit(),
                            _ => {}
                        },
                        Screen::BossFight => match key.code {
                            KeyCode::Left | KeyCode::Char('h') => app.boss_prev_action(),
                            KeyCode::Right | KeyCode::Char('l') => app.boss_next_action(),
                            KeyCode::Enter | KeyCode::Char(' ') => app.boss_perform_action(),
                            KeyCode::Char('q') => app.quit(),
                            _ => {}
                        },
                        Screen::Skills => match key.code {
                            KeyCode::Char('s') | KeyCode::Esc => app.toggle_skills(),
                            KeyCode::Char('j') | KeyCode::Down => app.skill_next(),
                            KeyCode::Char('k') | KeyCode::Up => app.skill_prev(),
                            KeyCode::Enter | KeyCode::Char(' ') => app.skill_allocate(),
                            KeyCode::Char('q') => app.quit(),
                            _ => {}
                        },
                        Screen::Main => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                            KeyCode::Left | KeyCode::Char('h') => app.prev_action(),
                            KeyCode::Right | KeyCode::Char('l') => app.next_action(),
                            KeyCode::Enter | KeyCode::Char(' ') => app.perform_action(),
                            KeyCode::Tab => app.toggle_mode(),
                            KeyCode::Char('i') => app.toggle_inventory(),
                            KeyCode::Char('s') => app.toggle_skills(),
                            _ => {}
                        },
                    }
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            let elapsed = last_tick.elapsed().as_secs().max(1);
            app.tick(elapsed);
            last_tick = Instant::now();
        }
    }

    // Save on quit
    if let Err(e) = app.save() {
        eprintln!("Warning: failed to save game: {e}");
    }

    Ok(())
}
