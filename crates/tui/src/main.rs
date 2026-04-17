mod app;
mod theme;
mod ui;

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

const TICK_RATE: Duration = Duration::from_secs(1);

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
    let mut app = App::new("Wobble");
    let mut last_tick = Instant::now();

    while app.running {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                        KeyCode::Left | KeyCode::Char('h') => app.prev_action(),
                        KeyCode::Right | KeyCode::Char('l') => app.next_action(),
                        KeyCode::Enter | KeyCode::Char(' ') => app.perform_action(),
                        _ => {}
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

    Ok(())
}
