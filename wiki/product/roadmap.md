# Roadmap

Tuigotchi is built in phases. V1 is a classic Tamagotchi. V2 adds JRPG depth.

## V1 — Classic Virtual Pet (Complete)

### Phase 0: Scaffold & Tooling ✓
- Cargo workspace with two crates
- CI pipeline (build, test, clippy, fmt on Linux + macOS)
- Conventional commit enforcement (local hook + CI)
- Release-please automation
- Development tooling (bacon, rustfmt, clippy configs)

### Phase 1: Core Game Logic ✓
- Pet with four stats (hunger, happiness, health, energy)
- Five life stages (Egg → Baby → Teen → Adult → Elder)
- Four actions (Feed, Play, Clean, Sleep)
- Per-second tick decay with starvation and death
- Event system (StatWarning, Evolved, Died)
- Unit tests for decay, death, and evolution

### Phase 2: Terminal UI ✓
- ratatui rendering with five-section layout
- ASCII art per stage with color theming
- Stat gauge bars
- Action selection with keyboard navigation
- Status bar with event messages
- Panic hook for clean terminal restoration

## V2 — JRPG Expansion (Planned)

### Phase 3: Combat System
- Random encounter triggers
- Turn-based JRPG battle mechanics
- Pet stats → combat stats mapping
- New `crates/combat` workspace member

### Phase 4: Character Building
- Skill trees unlocked through evolution and combat
- Skill point allocation
- Passive and active abilities
- New `crates/skills` workspace member

### Phase 5: Itemization
- Diablo 2-style random loot generation
- Affix system (prefix + suffix)
- Rarity tiers
- Equipment slots and inventory
- New `crates/items` workspace member

## Feature ideas (not yet scheduled)

- Save/load game state (serialization)
- Multiple pets
- Day/night cycle affecting decay rates
- Achievements
- Sound effects (terminal bell or audio)
- Network features (pet battles?)

## See also

- [V2 Readiness](../architecture/v2-readiness.md) — architecture supporting future phases
- [Future Combat](../game-design/future-combat.md) — combat design space
- [Release Process](release-process.md) — how versions ship
