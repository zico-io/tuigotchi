# Tuigotchi Linear label taxonomy (provisional)

This taxonomy is provisional as of 2026-06-01.

Linear workspace labels could not be verified in this run because Linear credentials were unavailable, so this file intentionally uses a conservative proposal-only policy.

## Taxonomy version

- `provisional-2026-06-01`

## Required label families

No label families are enforced for automatic mutation until a human confirms the actual Linear label set.

## Candidate label families (reference only)

These families are safe suggestion buckets for repair proposals, not auto-apply rules:

- `type/*`: what kind of work this is
- `area/*`: product or system area
- `source/*`: where the work originated

## Candidate labels for proposals (unverified in Linear)

Derived from repository-visible GitHub label conventions; use only in repair proposals until Linear labels are confirmed:

- `bug`
- `enhancement`
- `documentation`
- `question`
- `help wanted`
- `good first issue`

## Automation policy

- Auto-add allowlist: `none` (no labels may be auto-added yet).
- Auto-remove policy: `never` (the daemon may not remove labels automatically).
- Conflict handling: post a repair proposal comment with recommended labels and rationale.

## Deprecated labels (never auto-apply)

- `duplicate`
- `invalid`
- `wontfix`

## Human input required to leave provisional mode

1. Confirm the real Linear label names and required families.
2. Define which labels the daemon may add automatically.
3. Confirm whether any labels may ever be removed automatically (or keep proposal-only repairs).
