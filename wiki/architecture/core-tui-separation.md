# Core-TUI Separation

The game logic (`tuigotchi-core`) is completely independent of the terminal UI (`tuigotchi-tui`). This is the foundational architecture decision.

## Why it matters

1. **Testability.** Core logic is unit-tested without spinning up a terminal. Tests run in milliseconds, no mocking needed.
2. **Portability.** The same core crate could drive a web UI, a GUI, or a headless simulation. The game rules don't care about rendering.
3. **Compile times.** `tuigotchi-core` has zero external dependencies. Changes to core compile fast. TUI changes don't trigger core recompilation.
4. **V2 readiness.** Future crates (combat, items) depend on core types, not TUI types. The game model grows without touching rendering.

## What goes where

| Belongs in `core` | Belongs in `tui` |
|---|---|
| `Pet`, `PetStats`, `PetStage` | `App` (wraps `Pet` + UI state) |
| `Action` enum, `apply_action()` | Input handling (key → action mapping) |
| `tick()` function, decay rates | Event loop, tick scheduling |
| `GameEvent`, `EventBus` | Event processing → status messages |
| Stat clamping, death checks | ASCII art, color themes |
| Evolution thresholds | Gauge widgets, layout |

## The boundary

The TUI crate imports core types and calls core functions. Data flows one way:

```
User input → App (tui) → apply_action() / tick() (core) → mutated Pet → draw() (tui)
```

The core never imports from tui. The core never references `ratatui`, `crossterm`, or any rendering concept.

## The `App` struct

`App` in `crates/tui/src/app.rs` is the glue layer. It owns a `Pet` and an `EventBus` from core, plus TUI-specific state (`selected_action`, `status_message`, `running`). It translates between the core's event system and the UI's status bar.

## See also

- [Workspace Layout](workspace-layout.md) — directory structure and dependencies
- [Event System](event-system.md) — how events cross the boundary
