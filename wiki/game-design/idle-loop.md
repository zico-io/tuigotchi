# Idle/Incremental Gameplay Loop

V2 transforms tuigotchi from a real-time Tamagotchi into an idle/incremental second-monitor game where the player checks back periodically to engage with RPG mechanics.

## Two-State Core Loop

The game alternates between two states:

### Explore

- Constant auto-battling (Dragon Quest-style simple turn-based encounters)
- Pet fights automatically — player cannot intervene except for boss encounters
- XP and loot accumulate passively
- Stats still decay while exploring
- Boss encounters queue every ~50 battles and require manual resolution
- Auto-transitions to Camp when stats become critical

### Camp (Rest)

- Pet care: Feed, Play, Clean, Sleep (existing v1 actions)
- Manage inventory: equip, unequip, discard items
- Allocate skill points
- Toggle back to Explore when ready (blocked if `needs_care` is true)

## Idle Pacing

| Design parameter | Value | Rationale |
|---|---|---|
| Safe AFK window | ~8 hours | Covers a workday or sleep |
| Neglect consequence | Stops gaining resources | Pet does NOT die from neglect |
| Death source | Boss fights only | Defeat forces return to Camp |
| Check-in depth | 30 seconds to several minutes | Quick feed+loot or deep equip+skill+boss |
| Auto-battle rate | 1 per tick (continuous) | Resources accumulate steadily |
| Boss frequency | Every ~50 auto-battles | Periodic engagement hook |

## Offline Progression

When the player closes and reopens the game:
1. Load save file, compute elapsed time since last session
2. Simulate stat decay analytically (multiply rates by elapsed time)
3. Simulate auto-battles in batch (battles * avg XP/loot per battle)
4. Skip boss encounters — queue them for player return
5. Show "Welcome back" summary of what happened

## Needs-Care Gate

When stats become critical (hunger >= 90, happiness <= 10, or energy <= 10):
- Pet enters `needs_care` state
- Auto-battling stops — no more resource gains
- Player must return to Camp and restore stats
- `needs_care` clears when hunger < 70, happiness > 30, energy > 20
- This is the Tamagotchi-style guilt hook that drives periodic check-ins

## Mode Transitions

```
┌──────────┐  Tab / manual   ┌───────────┐
│          │ ──────────────→  │           │
│   Camp   │                  │  Explore  │
│          │ ←──────────────  │           │
└──────────┘  Tab / forced    └───────────┘
      ↑                            │
      │         Boss queued        │
      │    ┌────────────────┐      │
      └─── │  Boss Fight    │ ←────┘
           │  (manual)      │
           └────────────────┘
```

- **Camp → Explore**: manual toggle (blocked if `needs_care`)
- **Explore → Camp**: manual toggle OR auto-transition when stats critical
- **Explore → Boss Fight**: auto-transition when boss encounter queues
- **Boss Fight → Camp**: on defeat (forced, sets `needs_care`)
- **Boss Fight → Explore**: on victory or successful flee

## See also

- [Stats and Decay](stats-and-decay.md) — base stats that drive the idle pacing
- [Future Combat](future-combat.md) — detailed combat design space
- [Roadmap](../product/roadmap.md) — phased implementation plan
