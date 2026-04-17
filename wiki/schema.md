# Wiki Schema

This document defines how the tuigotchi wiki is structured and maintained.

## Purpose

The wiki is a persistent, compounding knowledge base for the tuigotchi project. It sits between the raw codebase and the people working on it — capturing architecture decisions, game design rationale, product direction, and ideas that don't belong in code comments or commit messages.

The codebase is the source of truth for *what the code does*. The wiki is the source of truth for *why it's that way* and *where it's going*.

## Who maintains it

The LLM writes and maintains all wiki pages. Humans direct what gets added, updated, or removed. Think of it like pair-writing: the human says "document the decay system" and the LLM does the filing, cross-referencing, and formatting.

## Page format

Every wiki page follows this structure:

```markdown
# Page Title

One or two sentences summarizing what this page covers.

## Section

Content organized under H2 and H3 headings.

## See also

- [Related page](../category/page.md) — why it's related
```

Rules:
- **H1** is the page title, used exactly once
- **Summary** immediately follows H1 — a sentence or two, no heading
- **Body** uses H2/H3 for structure
- **See also** at the bottom links to related pages
- Links use relative paths: `[text](../category/page.md)`
- Code references use `crates/core/src/file.rs` paths from repo root
- Tables for structured data (stat values, thresholds, color mappings)

## Directory structure

```
wiki/
├── index.md              # Entry point — categorized links to all pages
├── schema.md             # This file
├── architecture/         # Crate boundaries, patterns, design decisions
├── game-design/          # Mechanics, stats, actions, art, future systems
├── product/              # Roadmap, releases, tech stack
└── feedback/             # Ideas, UX observations, playtesting notes
```

## Index

`index.md` is the single entry point. Every page in the wiki must appear in the index with a one-line description. The index is organized by category. When adding a page, update the index in the same pass.

## Operations

**Add a page.** Write the markdown file in the appropriate category directory. Add it to `index.md`. Cross-link from related pages.

**Update a page.** When the codebase changes in a way that affects a wiki page, update the page to reflect the new state. Check cross-references still hold.

**Remove a page.** Delete the file, remove it from `index.md`, and remove inbound links from other pages.

**Health check.** Periodically verify: all index links resolve, no orphan pages, cross-references are accurate, stat values match the codebase.

## Automated maintenance hooks

Three Claude Code hooks in `.claude/settings.json` keep the wiki in sync with the codebase:

| Hook | Event | What it does |
|---|---|---|
| Wiki freshness check | `PostToolUse` (Write\|Edit) | When a Rust source file in `crates/` is modified, an agent checks if any wiki page contains stale values (stats, thresholds, art, events). Flags drift with a suggestion. |
| Session-end audit | `Stop` | When a session ends, audits `git diff` for source changes and cross-checks wiki pages for staleness. |
| Compaction context | `PreCompact` | Before context compaction, injects a reminder to preserve wiki-relevant context (which pages may need updates). |

The freshness check and session-end audit use Haiku for fast, low-cost checks. They only fire for crate source file changes — wiki edits, config changes, and non-Rust files are ignored.

## Conventions

- Stat values, thresholds, and rates should match the codebase exactly. If they drift, the codebase wins.
- Speculation about future systems (v2 combat, loot, skills) goes in dedicated "future" pages, clearly marked as design-space exploration, not commitments.
- Don't duplicate CLAUDE.md content. The wiki complements it — CLAUDE.md is for Claude Code context, the wiki is for humans and LLMs exploring the project.
