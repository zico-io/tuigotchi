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

## V2 — Idle/Incremental JRPG (Planned)

The v2 rework transforms tuigotchi into an idle/incremental second-monitor game with two core states: **Explore** (auto-battling) and **Camp** (pet care, inventory, skills). See [Idle Loop](../game-design/idle-loop.md) for full design.

### Phase 3: Save/Load + Idle Infrastructure
- Save/load with JSON persistence
- Decay rebalancing for ~8-hour AFK window
- Offline time simulation on resume
- `needs_care` gate (stops resources, doesn't kill)

### Phase 4: Game State Machine
- Explore/Camp mode toggle
- Auto-transition to Camp when stats critical
- Mode-aware tick dispatch

### Phase 5: Combat System
- New `crates/combat` workspace member
- Auto-battle engine with simple enemies (Dragon Quest style)
- Pet stats → combat stats mapping
- XP accumulation and leveling

### Phase 6: Loot & Inventory
- New `crates/items` workspace member
- Simple stat-boost items from battle drops
- Inventory with equip/unequip/discard
- Equipment modifies combat stats

### Phase 7: Boss Encounters
- Manual turn-based boss fights (Attack/Defend/Flee)
- Only source of death — defeat forces Camp
- Guaranteed rare drops on victory

### Phase 8: TUI Overhaul
- Split into per-screen modules (Camp, Explore, Boss Fight, Inventory)
- Visual polish and rarity colors

### Phase 9: Skills
- New `crates/skills` workspace member
- Skill tree with points from leveling
- Three categories: Combat, Survival, Fortune
- Modifies decay rates, combat stats, and loot chances

## Future ideas (not yet scheduled)

- Diablo 2-style affix system (full random loot)
- Multiple pets
- Day/night cycle affecting decay rates
- Achievements
- Areas/zones with different enemy pools
- Network features (pet battles?)

## See also

- [V2 Readiness](../architecture/v2-readiness.md) — architecture supporting future phases
- [Future Combat](../game-design/future-combat.md) — combat design space
- [Release Process](release-process.md) — how versions ship
