# Future Combat System

Design space exploration for v2's JRPG random encounters, skill trees, and Diablo 2-style loot. Nothing here is committed — it's a thinking space for what v2 could look like.

## JRPG Random Encounters

The core idea: while caring for your pet in the overworld, random encounters interrupt with turn-based combat. Your pet's stats influence combat performance.

### Open questions

- **Encounter triggers.** Time-based? Action-based (every N actions)? Exploration-based (if we add areas)?
- **Turn structure.** Classic JRPG (select attack, execute, enemy turn)? ATB (active time battle)?
- **Stat mapping.** How do pet stats translate to combat stats? Does a hungry pet fight worse?
- **Fleeing.** Can you always flee? Energy cost?
- **Difficulty scaling.** Based on pet stage? Total age? Win streak?

### Possible combat stats

| Combat stat | Derived from | Idea |
|---|---|---|
| Attack | happiness + energy | Happy, energetic pets hit harder |
| Defense | health | Healthy pets take less damage |
| Speed | energy | Energetic pets act first |
| Morale | happiness | Low morale = chance to skip turn |

## Character Building (Skill Trees)

Pets could unlock abilities as they evolve and win fights.

### Open questions

- **Skill points.** From leveling? From evolution? From specific actions?
- **Tree shape.** Linear progression? Branching paths (pick one of two)?
- **Respec.** Can you undo skill choices?
- **Stage-locked skills.** Do certain skills only unlock at certain stages?

### Possible skill categories

- **Combat** — attack moves, defensive stances, counterattacks
- **Passive** — stat bonuses, slower decay, better healing from actions
- **Utility** — flee bonus, item find rate, XP multiplier

## Itemization (Diablo 2-Style Loot)

Random loot with affixes, rarity tiers, and equipment slots.

### Open questions

- **Drop source.** Combat only? Found while exploring? Crafted?
- **Rarity tiers.** Normal, magic, rare, unique? How many?
- **Affix system.** Prefix + suffix? How many affixes per tier?
- **Equipment slots.** What does a pet wear? Hat, collar, accessory?
- **Inventory limit.** How many items can you hold?
- **Trading / discarding.** Sell for gold? Just discard?

### Possible affix examples

```
Sturdy Collar of Vigor       (prefix: +5 defense, suffix: +10 max health)
Gleaming Hat                  (prefix: +3 happiness/tick)
Ancient Amulet of the Elder   (prefix: +XP%, suffix: evolution speed)
```

## Architecture impact

V2 features would live in new crates:

| Crate | Responsibility |
|---|---|
| `crates/combat` | Encounter generation, turn logic, damage calculation |
| `crates/items` | Item generation, affixes, inventory management |
| `crates/skills` | Skill tree definitions, point allocation, effects |

All three depend on `tuigotchi-core` for `Pet`, `PetStats`, and `GameEvent`. The TUI crate adds new render screens (combat view, inventory screen, skill tree display).

## See also

- [V2 Readiness](../architecture/v2-readiness.md) — architecture decisions supporting this
- [Pet Lifecycle](pet-lifecycle.md) — stage system that combat builds on
- [Stats and Decay](stats-and-decay.md) — base stats that map to combat stats
- [Roadmap](../product/roadmap.md) — when v2 phases are planned
