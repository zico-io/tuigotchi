# Gameplay Ideas

A living backlog of ideas for future features. Nothing here is committed — it's a space for capturing possibilities.

## Mechanics

### Personality system
Pets could develop personality traits based on how they're raised. A pet that's played with often becomes playful (higher happiness decay but faster happiness gain). A well-fed pet becomes lazy (lower energy, higher hunger). Personality affects combat in v2.

### Day/night cycle
Decay rates could change based on time of day. Energy decays faster during "day" (the pet is active), hunger decays slower at "night" (the pet is resting). Sleep action is more effective at night.

### Mood system
Beyond the four stats, a mood derived from stat combinations:

| Mood | Condition |
|---|---|
| Content | All stats in safe range |
| Hungry | Hunger > 60 |
| Sad | Happiness < 30 |
| Tired | Energy < 20 |
| Sick | Health < 50 |
| Critical | Multiple stats in danger |

Mood could affect the ASCII art expression and available actions.

### Mini-games
Instead of instant stat changes, actions could trigger mini-games:
- **Feed** — choose the right food (some foods give more hunger reduction)
- **Play** — simple reflex game in the terminal
- **Clean** — scrubbing animation

### Multiple pets
Manage a small party of pets. They interact with each other — playing together is more effective, but food is shared. Sets up v2 party-based combat.

## Quality of life

### Save/load
Serialize game state to a file. Resume where you left off. Essential for any real gameplay beyond demos.

### Notifications
Terminal bell or system notification when stats hit critical thresholds. Useful when the game is running in a background terminal.

### Speed controls
Adjustable tick rate — slow motion for relaxed play, fast-forward for testing or speedruns.

### Statistics screen
Track lifetime stats: total actions performed, time alive, stages reached, deaths. A history of past pets.

## Visual

### Animated transitions
Brief animation when evolving (old art → flash → new art). Death animation before tombstone appears.

### Weather/background
Terminal background effects — rain (falling characters), sunshine (bright colors), snow. Purely aesthetic.

### Custom pet names
Let the player name their pet at startup instead of hardcoded "Wobble".

## See also

- [Future Combat](../game-design/future-combat.md) — v2 combat design space
- [Roadmap](../product/roadmap.md) — what's actually scheduled
- [UX Notes](ux-notes.md) — terminal interaction patterns
