# Tech Stack

Technology choices and the reasoning behind them.

## Language: Rust

Chosen for:
- **Performance** — zero-cost abstractions, no GC pauses during rendering
- **Safety** — ownership model prevents data races, use-after-free
- **Ecosystem** — strong TUI library ecosystem (ratatui)
- **Learning** — project uses intermediate Rust idioms as a learning exercise

## TUI framework: ratatui

[ratatui](https://ratatui.rs/) is a Rust library for building terminal UIs. It provides:
- Layout system with constraints
- Built-in widgets (Paragraph, Gauge, Block, etc.)
- Pluggable backends (we use crossterm)
- Immediate-mode rendering (redraw every frame)

**Why ratatui over alternatives:**
- Active community, well-documented
- Clean widget API
- Flexible layout system
- Good `crossterm` integration

## Terminal I/O: crossterm

[crossterm](https://github.com/crossterm-rs/crossterm) handles terminal raw mode, alternate screen, and key events. It's cross-platform (Linux, macOS, Windows).

Used for:
- `enable_raw_mode()` / `disable_raw_mode()`
- `EnterAlternateScreen` / `LeaveAlternateScreen`
- `event::poll()` and `event::read()` for keyboard input

## Development tools

| Tool | Purpose | Command |
|---|---|---|
| `cargo clippy` | Linting with `-D warnings` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `cargo fmt` | Code formatting | `cargo fmt --all --check` |
| `bacon` | File-watching build runner | `bacon clippy` |
| `release-please` | Automated versioning/changelog | GitHub Action (CI only) |

## CI

GitHub Actions pipeline runs on every PR:
- Build (Linux + macOS)
- Test
- Clippy (deny warnings)
- Format check
- Conventional commit validation

## See also

- [Workspace Layout](../architecture/workspace-layout.md) — how the crates are organized
- [Release Process](release-process.md) — how CI and releases work
