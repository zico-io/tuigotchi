# Workspace Layout

The project is a Cargo workspace with two crates, designed so game logic compiles and tests independently of any rendering framework.

## Structure

```
tuigotchi/
├── Cargo.toml              # Workspace root — defines members, shared version
├── crates/
│   ├── core/               # tuigotchi-core: pure game logic
│   │   └── src/
│   │       ├── lib.rs      # Public API re-exports
│   │       ├── pet.rs      # Pet, PetStats, PetStage
│   │       ├── action.rs   # Action enum, apply_action()
│   │       ├── tick.rs     # Time-based decay, aging, death
│   │       └── event.rs    # GameEvent, EventBus
│   └── tui/                # tuigotchi-tui: terminal rendering
│       └── src/
│           ├── main.rs     # Entry point, terminal setup, event loop
│           ├── app.rs      # App state, input → action mapping
│           ├── ui.rs       # ratatui widget rendering, ASCII art
│           └── theme.rs    # Color and style constants
```

## Dependency graph

```
tuigotchi-tui
  └── tuigotchi-core    (game logic)
  └── ratatui           (TUI framework)
  └── crossterm         (terminal I/O)

tuigotchi-core
  └── (no external deps)
```

`tuigotchi-core` has zero external dependencies. This is intentional — it keeps the game logic portable and fast to compile.

## Version management

Both crates share a single version defined in `workspace.package.version` in the root `Cargo.toml`. Release-please bumps this version and both crates move in lockstep.

## V2 expansion

Future crates (`crates/combat`, `crates/items`, etc.) will sit alongside `core` and `tui` as workspace members. They'll depend on `tuigotchi-core` for shared types but won't need to touch core's internals.

## See also

- [Core-TUI Separation](core-tui-separation.md) — why the split matters
- [V2 Readiness](v2-readiness.md) — decisions enabling future crates
- [Tech Stack](../product/tech-stack.md) — library choices
