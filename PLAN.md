# Tuigotchi — Rust Project Scaffold Plan

## Context

Scaffolding a new Rust TUI virtual pet game ("tuigotchi") from an empty repo. V1 is classic Tamagotchi mechanics. V2 (future) adds JRPG combat, skill trees, and Diablo 2-style loot. The architecture must support v2 without requiring rewrites — clean crate boundaries, trait-based extensibility, and a game-logic core decoupled from rendering.

## Workspace Structure

```
tuigotchi/
├── Cargo.toml                  # workspace root
├── CLAUDE.md                   # Claude Code project context
├── .gitignore
├── .rustfmt.toml
├── .clippy.toml
├── bacon.toml                  # bacon (cargo-watch alternative) config
├── .github/
│   └── workflows/
│       └── ci.yml              # build + test + clippy + fmt (Linux + macOS)
├── crates/
│   ├── core/                   # tuigotchi-core: game logic, no TUI deps
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pet.rs          # Pet struct, stats, state machine
│   │       ├── action.rs       # Feed, play, clean, etc.
│   │       ├── tick.rs         # Time-based stat decay & aging
│   │       └── event.rs        # Game events (trait-based, v2 extensible)
│   └── tui/                    # tuigotchi-tui: rendering & input
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs         # Entry point, terminal setup, event loop
│           ├── app.rs          # App state, input handling
│           ├── ui.rs           # ratatui widget rendering
│           └── theme.rs        # Colors, styles (centralized)
└── README.md
```

## Phases

### Phase 0: Scaffold & Tooling
- Workspace root files (Cargo.toml, .gitignore, .rustfmt.toml, .clippy.toml, bacon.toml)
- CLAUDE.md with project context
- GitHub Actions CI pipeline
- Empty crate shells that compile

### Phase 1: Core Game Logic (`crates/core`)
- Pet struct with stats (hunger, happiness, health, energy)
- PetStage enum (Egg → Baby → Teen → Adult → Elder)
- Action enum (Feed, Play, Clean, Sleep) with `#[non_exhaustive]`
- Tick system for time-based stat decay and stage evolution
- GameEvent trait + EventBus for extensibility
- Unit tests

### Phase 2: TUI Rendering (`crates/tui`)
- Terminal init/restore with panic hook
- Sync event loop (crossterm poll with timeout)
- App state machine
- Pet ASCII art, stat bars, action menu
- Theme/style constants

## V2-readiness decisions (no implementation now)
- **`GameEvent` trait** in core: v2 combat/loot events implement this same trait
- **`PetStats` is a struct, not hardcoded fields**: easy to add XP, mana, etc.
- **Workspace crates**: v2 adds `crates/combat`, `crates/items` without touching core or tui internals
- **Action enum is non-exhaustive**: `#[non_exhaustive]` so adding Attack, UseItem, etc. doesn't break matches
- **Event loop in tui is a simple state machine**: v2 can add game states (Overworld, Combat, Inventory) as enum variants

## Verification
1. `cargo build` — compiles with no errors
2. `cargo test` — core unit tests pass
3. `cargo clippy -- -D warnings` — no warnings
4. `cargo fmt --check` — formatted
5. `cargo run -p tuigotchi-tui` — launches TUI, shows pet with stats, responds to key input
6. `bacon clippy` — bacon runs and watches for changes
