# Tuigotchi

A terminal-based virtual pet game built with Rust + Ratatui.

## Project Structure

Cargo workspace with two crates:

- **`crates/core`** (`tuigotchi-core`) — Pure game logic. No TUI deps. This is where pet stats, actions, tick/decay, and the event system live.
- **`crates/tui`** (`tuigotchi-tui`) — Terminal rendering with ratatui + crossterm. Sync event loop, input handling, UI widgets.

## Commands

```bash
cargo build --workspace          # Build everything
cargo test --workspace           # Run all tests
cargo run -p tuigotchi-tui       # Launch the game
cargo clippy --workspace --all-targets -- -D warnings  # Lint
cargo fmt --all --check          # Format check
bacon clippy                     # Watch mode (requires: cargo install bacon)
```

## Conventions

- All game logic goes in `crates/core` — keep it free of TUI dependencies
- Stats are f32 in [0.0, 100.0], always clamped after mutation
- `Action` and `GameEvent` enums are `#[non_exhaustive]` for forward compatibility
- Match arms on non-exhaustive enums must include a wildcard `_ => {}` branch
- Intermediate Rust idioms: use `impl Into<String>`, `Option`/`Result` chaining, derive macros

## Releases

Versioning and changelog are automated with [release-please](https://github.com/googleapis/release-please).

**Commit format**: [Conventional Commits](https://www.conventionalcommits.org/). Common types:
- `feat: …` — new feature (bumps patch pre-1.0, minor post-1.0)
- `fix: …` — bug fix (bumps patch)
- `feat!: …` or footer `BREAKING CHANGE: …` — breaking change (bumps minor pre-1.0, major post-1.0)
- `docs:`, `refactor:`, `perf:` — appear in CHANGELOG
- `chore:`, `test:`, `ci:`, `build:`, `style:` — hidden from CHANGELOG

**Flow**: merge commits to `main` → the `release-please` workflow opens or updates a release PR that bumps `workspace.package.version` in `Cargo.toml`, refreshes `Cargo.lock`, and appends to `CHANGELOG.md`. Merging that PR tags `vX.Y.Z` and publishes a GitHub Release. Both crates share one version.

Config lives in `release-please-config.json`; the current released version in `.release-please-manifest.json` — release-please owns both.

**Commit format is enforced** at two layers:
- Local `commit-msg` hook in `.githooks/commit-msg`. Enable once per clone:
  ```bash
  git config core.hooksPath .githooks
  ```
- CI workflow `.github/workflows/conventional-commits.yml` validates every PR title and every commit subject in the PR. Agents and humans should not bypass the local hook with `--no-verify`; CI is the backstop either way.

## Wiki

A structured knowledge base lives in `wiki/`. See `wiki/schema.md` for conventions.
- `wiki/index.md` — start here
- LLM-maintained: add/update pages through conversation
- Uses relative markdown links for cross-references
- Maintenance hooks in `.claude/settings.json` auto-detect wiki drift when source files change

## V2 Roadmap (not yet implemented)

Future phases will add:
- JRPG random encounters (combat system)
- Character building (skill trees & stats)
- Itemization (Diablo 2-style random loot)

These will land as new workspace crates (`crates/combat`, `crates/items`, etc.) consuming `tuigotchi-core` traits.
