# Release Process

Versioning and changelog are fully automated with release-please.

## How it works

1. **Commit to main** using conventional commit format
2. **Release-please** (GitHub Action) opens or updates a release PR
3. The release PR bumps `workspace.package.version` in root `Cargo.toml`, updates `Cargo.lock`, and appends to `CHANGELOG.md`
4. **Merge the release PR** → tags `vX.Y.Z` and publishes a GitHub Release

Both crates share a single version. They move in lockstep.

## Conventional commits

Every commit subject must follow [Conventional Commits](https://www.conventionalcommits.org/):

| Prefix | Meaning | Version bump | In CHANGELOG? |
|---|---|---|---|
| `feat:` | New feature | Patch (pre-1.0) / Minor (post-1.0) | Yes |
| `fix:` | Bug fix | Patch | Yes |
| `feat!:` | Breaking change | Minor (pre-1.0) / Major (post-1.0) | Yes |
| `docs:` | Documentation | None | Yes |
| `refactor:` | Code restructuring | None | Yes |
| `perf:` | Performance | None | Yes |
| `chore:` | Maintenance | None | No |
| `test:` | Tests | None | No |
| `ci:` | CI changes | None | No |
| `build:` | Build system | None | No |
| `style:` | Formatting | None | No |

## Enforcement

Commit format is enforced at two layers:

1. **Local hook** — `.githooks/commit-msg` validates the commit subject. Enable per clone:
   ```bash
   git config core.hooksPath .githooks
   ```
2. **CI** — `.github/workflows/conventional-commits.yml` validates every PR title and commit subject

Do not bypass the local hook with `--no-verify`. CI is the backstop.

## Config files

| File | Purpose | Who edits |
|---|---|---|
| `release-please-config.json` | Release-please configuration | release-please (mostly) |
| `.release-please-manifest.json` | Current released version | release-please only |

## See also

- [Roadmap](roadmap.md) — what's shipping when
- [Tech Stack](tech-stack.md) — tooling choices
