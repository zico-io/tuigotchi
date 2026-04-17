# UX Notes

Observations and rationale about the terminal user experience.

## Layout

The screen is divided into five vertical sections:

```
┌─────────────────────────────────┐
│ Title (name, stage, age)     3h │
├─────────────────────────────────┤
│                                 │
│ Pet ASCII art                8h+│
│                                 │
├─────────────────────────────────┤
│ Hunger:  ████████░░ 80%         │
│ Happy :  ██████░░░░ 60%      8h │
│ Health:  ██████████ 100%        │
│ Energy:  ████░░░░░░ 40%        │
├─────────────────────────────────┤
│ ▸ Feed  Play  Clean  Sleep   3h │
├─────────────────────────────────┤
│ Take care of your pet!       2h │
└─────────────────────────────────┘
```

The pet display area is the only flexible section (`Min(8)`) — it expands to fill available terminal height. All other sections have fixed heights.

## Keybindings

| Key | Action | Rationale |
|---|---|---|
| `←` / `h` | Previous action | Arrow + vi-style |
| `→` / `l` | Next action | Arrow + vi-style |
| `Enter` / `Space` | Perform action | Both common "confirm" keys |
| `q` / `Esc` | Quit | Both common "exit" keys |

Vi-style `h`/`l` navigation is included because terminal users often prefer it. Both styles are always available — no mode switching.

## Status bar

The bottom line shows contextual messages:
- Default: "Take care of your pet!"
- After action: "You fed your pet!" / "You played with your pet!" etc.
- On warning: "Warning: hunger is critical!"
- On evolution: "Wobble evolved to Teen!"
- On death: "Wobble has died..."

Messages are overwritten each tick or action. There's no message queue — the most recent event wins. This is simple but can cause warnings to flash briefly before being replaced.

## Terminal handling

- **Raw mode** — keys are read immediately without Enter
- **Alternate screen** — game renders in a separate buffer, terminal content is restored on exit
- **Panic hook** — if the program panics, the terminal is restored before the panic message prints. Without this, a panic would leave the terminal in raw mode.

## Known limitations

- **No resize handling.** If the terminal is too small, the layout breaks. No minimum size check.
- **No mouse support.** Actions are keyboard-only.
- **Message flashing.** Stat warnings fire every tick when active, but can be overwritten by other events in the same tick.
- **Hardcoded pet name.** The pet is always named "Wobble". No prompt at startup.
- **No color theme switching.** Colors are constants, not configurable.

## See also

- [Actions](../game-design/actions.md) — what each action does
- [ASCII Art](../game-design/ascii-art.md) — visual per stage
- [Gameplay Ideas](gameplay-ideas.md) — UX improvement ideas
- [Tech Stack](../product/tech-stack.md) — ratatui and crossterm
