# Stats and Decay

The pet has four stats, each an `f32` clamped to `[0.0, 100.0]`. Stats decay every tick, and the player must take actions to counteract decay.

## Stats

| Stat | Default | Bar color | Direction | Notes |
|---|---|---|---|---|
| Hunger | 50.0 | Red | Rises over time | 0 = full, 100 = starving |
| Happiness | 50.0 | Yellow | Falls over time | Higher is better |
| Health | 100.0 | Green | Stable unless starving | Damaged by starvation only |
| Energy | 100.0 | Blue | Falls over time | Spent by playing |

**Hunger is inverted** — it rises toward 100 (starving), while the other three stats fall toward 0. This means "low hunger" is good, "high hunger" is bad.

## Decay rates

Per-second rates applied every tick (defined in `crates/core/src/tick.rs`):

| Stat | Rate | Effect per tick |
|---|---|---|
| Hunger | +0.5/s | Increases toward starvation |
| Happiness | -0.3/s | Decreases toward sadness |
| Energy | -0.2/s | Decreases toward exhaustion |
| Health | 0/s normally | No passive decay |
| Health | -1.0/s when starving | Decays when hunger >= 100.0 |

## Starvation

When `hunger >= 100.0`, the pet is starving. Health decays at 1.0 per second. This is the only way health decreases in v1. The death spiral:

```
hunger rises → hits 100 → health starts dropping → hits 0 → pet dies
```

## Clamping

All stats are clamped to `[0.0, 100.0]` after every mutation — both after actions and after tick decay. The `PetStats::clamp()` method handles this.

## Warning thresholds

The event system emits `StatWarning` events when stats hit critical levels:

| Stat | Threshold | Meaning |
|---|---|---|
| Hunger | >= 80.0 | Pet is very hungry |
| Happiness | <= 20.0 | Pet is very unhappy |
| Energy | <= 10.0 | Pet is exhausted |

Health has no warning — starvation itself is the warning.

## Balance notes

With no player intervention, starting from defaults:
- **Hunger hits 100** at ~100 seconds (50 + 0.5/s × 100s = 100)
- **Health depleted** ~100 seconds after starvation starts (100 - 1.0/s × 100s = 0)
- **Total survival time** from birth: ~200 seconds (~3.3 minutes) with no actions

A single Feed action reduces hunger by 20, buying ~40 more seconds before hunger climbs back. The player needs to feed roughly every 40 seconds to prevent starvation.

## See also

- [Actions](actions.md) — how the player modifies stats
- [Pet Lifecycle](pet-lifecycle.md) — stages and death
- [Event System](../architecture/event-system.md) — stat warning events
