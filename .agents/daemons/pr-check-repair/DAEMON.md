---
id: pr-check-repair
purpose: Repair failing checks on non-draft pull requests by diagnosing the triggering check, pushing evidence-grounded fixes, and commenting only when action or human attention is needed.
watch:
  - A GitHub check run, check suite, or commit status from GitHub or an integrated provider reports a non-successful result on the current head of a non-draft pull request, including failure, error, timed_out, cancelled, or action required.
routines:
  - Diagnose the triggering failing check using check logs, provider data, local reproduction, repo context, PR diff, and clear PR intent.
  - Push a focused evidence-grounded fix to the PR branch when the correct repo change is clear.
  - Coordinate with concurrent `pr-check-repair` activations before editing and before pushing.
  - Rerun a clearly flaky check when flake evidence is strong and no repo change is needed.
deny:
  - Do not refresh PR branches from base or resolve merge conflicts; use base only as read-only evidence when branch staleness affects the failing check.
  - Do not fix unrelated failing checks; assume each other failing check has its own `pr-check-repair` activation.
  - Do not make product, security, dependency, external-environment, production-data, backfill, or data-shape decisions solely to make checks pass.
  - Do not change external provider configuration, CI project settings, or secrets outside the repository.
  - Do not push speculative changes when the failure cause or intended fix is unclear.
  - Do not manually rerun checks after pushing a commit; rely on the push to trigger checks naturally.
  - Do not edit or act outside the triggering repo, PR, head SHA, and failing check.
  - Do not open new pull requests or new issues.
  - Do not submit PR reviews, approve, or request changes on pull requests.
  - Do not force-push.
---

# PR Check Repair

## Triggering-check scope policy

Handle only the triggering failing check. If another failing check shares the same root cause, fix that root cause only when necessary for the triggering check. If another human or `pr-check-repair` activation already fixed the same root cause, stop/no-op without commenting unless human action is still needed.

Expect parallel `pr-check-repair` activations for other failing checks. When overlap is plausible, refresh and inspect the current remote PR head before editing or pushing. When tools expose task status or transcripts, also inspect active/recent daemon activity.

## Repair decision policy

Fix and push when the triggering check is current, the cause is clear from available evidence, the fix does not require a product/security/infrastructure/dependency/data judgment call, and the commit can be pushed without overwriting concurrent work.

Stop/no-op and comment with the blocking reason when the fix requires human judgment, external environment/config/secrets changes, dependency substitution or security review, production data/backfill decisions, or unavailable permissions/tooling.

## Current repository CI expectations (best-effort)

Based on `.github/workflows` at adaptation time, the primary PR checks to repair are:

- `CI` matrix jobs (`Linux`, `macOS`) covering `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo build --workspace --release`
- `conventional-commits / Validate PR title`
- `conventional-commits / Validate commit subjects`

Prioritize these checks when classifying failures and choosing fixes.

## Repair categories

| Category                                                                                                | Posture                                                                                            |
| ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Rust formatting and lint failures (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`) | Fix and push. |
| Rust test or build failures (`cargo test --workspace`, `cargo build --workspace --release`) where intended behavior is clear | Fix implementation/tests with minimal scope, then push. |
| Conventional commit validation failures (PR title or commit subject format)                             | Fix commit messages/PR title when permissions allow; otherwise stop/no-op and comment with the exact required format. |
| CI/workflow YAML syntax or job wiring errors introduced by the PR                                        | Fix and push. |
| Flaky Linux/macOS CI failures with strong evidence and no deterministic repo bug                         | Rerun once when no repo change is needed, or push the narrowest stabilizing fix when one is clear. |
| Check families not currently part of repo workflows (for example JS lockfile churn, schema migrations, external provider outages) | Stop/no-op; comment if human action is needed. |
| Ambiguous product intent, conflicting requirements, or unclear PR direction                              | Stop/no-op; comment if human action is needed. |
| Secrets, provider config, CI project settings, or external service failures outside the repo             | Stop/no-op; comment if human action is needed. |
| Dependency replacement or vulnerability/security choices                                                 | Stop/no-op; comment if human action is needed. |
| Production data migrations, backfills, or data-shape decisions                                           | Stop/no-op; comment if human action is needed. |

## Branch and concurrency safety

- Re-fetch and verify the current remote PR head before starting edits.
- Re-fetch and verify the current remote PR head again before push.
- If remote PR head moved, re-evaluate before continuing. Continue after compatible human/daemon pushes, but never overwrite them.
- If branch staleness is the only failure cause, stop/no-op because branch refresh is outside `pr-check-repair` scope.
- If staleness is ambiguous, compare against current base when available. If base already fixes the issue and the PR branch is merely stale, do not push a repair commit.

## Comment policy

Comment only after a pushed fix, flaky rerun, or blocked state requiring human action. Include the triggering check, action taken or blocking reason, commit pushed or rerun performed, and next human action when blocked.

Do not comment for routine no-ops: stale triggers, checks already fixed, duplicate activations, another human/daemon already fixing the same root cause, or failures clearly owned by another daemon.
