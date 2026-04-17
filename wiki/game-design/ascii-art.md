# ASCII Art

Each life stage has a unique ASCII art representation rendered in the pet display area. Art is defined in `crates/tui/src/ui.rs` in the `pet_ascii()` function.

## Art by stage

### Egg (White)
```
   ___
  /   \
 | . . |
  \___/
```

### Baby (Cyan)
```
  ^___^
 ( o.o )
  > ^ <
```

### Teen (Green)
```
  /\_/\
 ( o.o )
  > ^ <
 /|   |\
```

### Adult (Yellow)
```
   /\_/\
  ( ^.^ )
 />   <\
 |  |  |
 _/   \_
```

### Elder (Magenta)
```
   /\_/\
  ( -.- )
 />   <\
 | ~~~ |
 _/   \_
```

### Dead (RIP)
```
  ___
 | R |
 | I |
 | P |
 |___|
```

## Design progression

The art tells a visual story:
- **Egg** — simple oval with dots (eyes visible through shell)
- **Baby** — tiny face, flat-top head (`^___^`)
- **Teen** — same face, but ears appear (`/\_/\`) and legs (`/|   |\`)
- **Adult** — larger body, happy expression (`^.^`), arms and legs
- **Elder** — same body as adult, sleepy expression (`-.-`), wavy detail (`~~~`)
- **Dead** — tombstone, no creature

## Color mapping

Colors are defined in `crates/tui/src/theme.rs`:

| Stage | Color constant | Ratatui value |
|---|---|---|
| Egg | `EGG_COLOR` | `Color::White` |
| Baby | `BABY_COLOR` | `Color::Cyan` |
| Teen | `TEEN_COLOR` | `Color::Green` |
| Adult | `ADULT_COLOR` | `Color::Yellow` |
| Elder | `ELDER_COLOR` | `Color::Magenta` |

The entire ASCII art block is rendered in the stage color. No per-character coloring in v1.

## Layout

The pet display area uses `Constraint::Min(8)` — it gets at least 8 rows and expands to fill available space. Art is rendered as a `Paragraph` widget inside a bordered block.

## See also

- [Pet Lifecycle](pet-lifecycle.md) — stage transitions and thresholds
- [Tech Stack](../product/tech-stack.md) — ratatui rendering framework
