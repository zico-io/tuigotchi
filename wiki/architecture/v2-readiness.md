# V2 Readiness

Decisions made in v1 to ensure v2 features (JRPG combat, skill trees, Diablo 2-style loot) can be added without rewriting the core.

## Non-exhaustive enums

Both `Action` and `GameEvent` use `#[non_exhaustive]`:

```rust
#[non_exhaustive]
pub enum Action { Feed, Play, Clean, Sleep }

#[non_exhaustive]
pub enum GameEvent { StatWarning(&'static str), Evolved { .. }, Died }
```

**Why:** Adding `Attack`, `UseItem`, `CombatResult`, `ItemDropped` etc. won't break any existing match statement — they all have `_ => {}` arms already.

## PetStats as a struct

Stats are a plain struct with named fields, not a fixed enum or array:

```rust
pub struct PetStats {
    pub hunger: f32,
    pub happiness: f32,
    pub health: f32,
    pub energy: f32,
}
```

**Why:** V2 can add `xp: f32`, `mana: f32`, `defense: f32` etc. directly to the struct. The `clamp()` method extends naturally.

## Workspace crates

The workspace pattern means v2 systems become new crates:

```
crates/
├── core/       # existing — shared types
├── tui/        # existing — rendering
├── combat/     # v2 — JRPG encounter system
├── items/      # v2 — loot generation and inventory
└── skills/     # v2 — skill trees and leveling
```

Each new crate depends on `tuigotchi-core` for `Pet`, `PetStats`, `Action`, `GameEvent`, but doesn't touch `core` internals or `tui`.

## Time-based tick

`tick()` accepts `elapsed: u64` seconds, not a fixed frame rate:

```rust
pub fn tick(pet: &mut Pet, elapsed: u64, events: &mut EventBus)
```

**Why:** V2 can have different tick rates for combat vs overworld. Pause/resume is trivial — just stop calling tick.

## Event loop as state machine

The TUI event loop is a simple `while app.running` loop. V2 can introduce game states:

```rust
enum GameState { Overworld, Combat, Inventory, Menu }
```

The loop structure supports this — different states render different screens and handle different inputs.

## What's NOT v2-ready (and that's fine)

- **No trait abstraction for tick systems.** V2 will likely need a `Tickable` trait, but adding it now would be speculative.
- **No ECS.** The pet is a single struct. If combat needs entity management, that's a v2 architecture decision.
- **No serialization.** Save/load will be needed for v2 but isn't worth designing without knowing the full data model.

## See also

- [Workspace Layout](workspace-layout.md) — current crate structure
- [Event System](event-system.md) — non-exhaustive GameEvent
- [Future Combat](../game-design/future-combat.md) — what v2 combat might look like
- [Roadmap](../product/roadmap.md) — phased delivery plan
