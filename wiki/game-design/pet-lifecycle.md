# Pet Lifecycle

A pet progresses through five life stages based on age. Each stage has distinct ASCII art, a color, and an evolution threshold.

## Stages

| Stage | Evolves at | Color | Description |
|---|---|---|---|
| Egg | 30s | White | Starting state. Dormant, waiting to hatch. |
| Baby | 120s | Cyan | Just hatched. Small and vulnerable. |
| Teen | 300s | Green | Growing up. Ears/limbs appear. |
| Adult | 600s | Yellow | Fully grown. Largest art form. |
| Elder | never | Magenta | Final stage. Sleepy expression. |

## Evolution logic

Evolution is checked at the end of each `tick()` call in `crates/core/src/tick.rs`:

1. Get the current stage's `evolution_threshold()` — returns `Some(seconds)` or `None` for Elder
2. If `pet.age_seconds >= threshold`, advance to `stage.next()`
3. Emit `GameEvent::Evolved { from, to }`
4. The TUI displays "{name} evolved to {stage}!" in the status bar

Age increments by `elapsed` seconds each tick. With a 1-second tick rate, age tracks real wall-clock time.

## Initial state

A new pet starts as:

| Field | Value |
|---|---|
| `stage` | `Egg` |
| `age_seconds` | `0` |
| `alive` | `true` |
| `hunger` | `50.0` |
| `happiness` | `50.0` |
| `health` | `100.0` |
| `energy` | `100.0` |

## Death

Death is not a stage — it's a boolean flag (`pet.alive`). A pet dies when `health <= 0.0`, which happens from starvation damage. Dead pets:

- Don't respond to actions (`apply_action` returns early)
- Don't tick (stat decay stops)
- Show the RIP tombstone ASCII art instead of their stage art

## See also

- [Stats and Decay](stats-and-decay.md) — how stats deteriorate over time
- [ASCII Art](ascii-art.md) — visual representation per stage
- [Actions](actions.md) — how the player keeps the pet alive
