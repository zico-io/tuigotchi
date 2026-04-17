# Event System

The event system decouples game state changes from their presentation. Core logic emits events; the TUI layer decides how to display them.

## GameEvent

Defined in `crates/core/src/event.rs`:

```rust
#[non_exhaustive]
pub enum GameEvent {
    StatWarning(&'static str),       // a stat hit a critical threshold
    Evolved { from: PetStage, to: PetStage },  // stage transition
    Died,                             // health reached zero
}
```

The `#[non_exhaustive]` attribute means match arms must include a wildcard `_ => {}` branch. This lets v2 add new event types (combat results, loot drops, skill unlocks) without breaking existing match statements.

## EventBus

A simple `Vec<GameEvent>` wrapper with three methods:

| Method | Purpose |
|---|---|
| `push(event)` | Queue an event during tick processing |
| `drain() -> Vec<GameEvent>` | Take all events, emptying the bus |
| `is_empty() -> bool` | Check if any events are pending |

Events are collected during `tick()` and drained by `App::process_events()` after each tick completes. This batch-drain pattern means multiple events can fire in a single tick (e.g., a stat warning and an evolution in the same second).

## Event flow

```
tick() runs
  ├── stat decay happens
  ├── hunger >= 80? → push(StatWarning("hunger"))
  ├── health <= 0?  → push(Died), return early
  ├── age >= threshold? → push(Evolved { from, to })
  └── tick returns

App::process_events() runs
  ├── drain() all events
  ├── StatWarning(stat) → status_message = "Warning: {stat} is critical!"
  ├── Evolved { to }    → status_message = "{name} evolved to {to}!"
  ├── Died              → status_message = "{name} has died..."
  └── _ => {}           (wildcard for future events)
```

## Warning thresholds

| Stat | Condition | Message |
|---|---|---|
| Hunger | `>= 80.0` | "Warning: hunger is critical!" |
| Happiness | `<= 20.0` | "Warning: happiness is critical!" |
| Energy | `<= 10.0` | "Warning: energy is critical!" |

## V2 extensibility

New event variants can be added to `GameEvent` without modifying existing code. A combat system would add `CombatResult { ... }`, a loot system would add `ItemDropped { ... }`, etc. The TUI's wildcard arm handles unknown events gracefully until explicit handling is added.

## See also

- [Stats and Decay](../game-design/stats-and-decay.md) — what triggers stat warnings
- [Core-TUI Separation](core-tui-separation.md) — how events cross the boundary
- [V2 Readiness](v2-readiness.md) — non-exhaustive pattern rationale
