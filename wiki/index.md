# Tuigotchi Wiki

A structured knowledge base for the tuigotchi project — architecture decisions, game mechanics, product direction, and ideas.

Start with [schema.md](schema.md) to understand how this wiki is organized and maintained.

---

## Architecture & Patterns

- [Workspace Layout](architecture/workspace-layout.md) — crate structure, dependency graph, version management
- [Core-TUI Separation](architecture/core-tui-separation.md) — why game logic is independent of rendering
- [Event System](architecture/event-system.md) — EventBus, GameEvent variants, event flow
- [V2 Readiness](architecture/v2-readiness.md) — decisions enabling future combat, loot, and skills

## Game Design & Mechanics

- [Pet Lifecycle](game-design/pet-lifecycle.md) — five stages, evolution thresholds, death
- [Stats and Decay](game-design/stats-and-decay.md) — four stats, decay rates, starvation, balance math
- [Actions](game-design/actions.md) — Feed/Play/Clean/Sleep with stat delta table
- [ASCII Art](game-design/ascii-art.md) — art per stage, color mapping, layout
- [Future Combat](game-design/future-combat.md) — JRPG encounters, skill trees, loot (design space)

## Product & Roadmap

- [Roadmap](product/roadmap.md) — v1 phases (complete), v2 phases (planned), feature ideas
- [Release Process](product/release-process.md) — release-please, conventional commits, CI enforcement
- [Tech Stack](product/tech-stack.md) — Rust, ratatui, crossterm, development tooling

## Feedback & Ideas

- [Gameplay Ideas](feedback/gameplay-ideas.md) — personality, day/night, moods, mini-games, save/load
- [UX Notes](feedback/ux-notes.md) — layout, keybindings, status bar, known limitations
