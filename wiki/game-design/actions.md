# Actions

The player interacts with their pet through four actions. Each action modifies stats by fixed amounts, clamped to [0.0, 100.0] after application.

## Action table

| Action | Hunger | Happiness | Health | Energy | Net effect |
|---|---|---|---|---|---|
| Feed | -20 | — | — | +5 | Reduces hunger, slight energy boost |
| Play | +10 | +15 | — | -10 | Fun but costly — hungry and tired |
| Clean | — | +5 | +10 | — | Health recovery, mild happiness |
| Sleep | +5 | — | — | +30 | Major energy recovery, slight hunger |

## Trade-offs

Every action has a cost:

- **Feed** is the most important — it's the only way to prevent starvation. But it only affects hunger and energy.
- **Play** is the main happiness source but increases hunger and drains energy. Playing while hungry accelerates the death spiral.
- **Clean** is the only way to restore health (besides preventing starvation). Low opportunity cost.
- **Sleep** gives the most energy but adds hunger. Sleeping while starving is dangerous.

## Controls

| Key | Effect |
|---|---|
| `←` or `h` | Select previous action |
| `→` or `l` | Select next action |
| `Enter` or `Space` | Perform selected action |
| `q` or `Esc` | Quit |

The action bar shows all four options with `▸` marking the selected one. Vi-style `h`/`l` navigation is supported alongside arrow keys.

## Implementation

Actions are defined in `crates/core/src/action.rs`:

- `Action` enum is `#[non_exhaustive]` with four variants
- `ALL_ACTIONS` constant defines the display order: Feed, Play, Clean, Sleep
- `apply_action(pet, action)` mutates stats and calls `clamp()`
- Dead pets ignore all actions (early return)

## Dead state

When `pet.alive == false`, `apply_action()` returns immediately. No stats change. The TUI still renders the action bar but actions have no effect.

## See also

- [Stats and Decay](stats-and-decay.md) — what stats mean and how they decay
- [Pet Lifecycle](pet-lifecycle.md) — death conditions
- [UX Notes](../feedback/ux-notes.md) — keybinding rationale
