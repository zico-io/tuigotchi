# Package manager configuration for `tuigotchi`

## Current repository status

At adaptation time (2026-06-01), this repository has no JavaScript/TypeScript package manifests or lockfiles.

Confirmed absent in repo tree:

- `package.json`
- `pnpm-lock.yaml`
- `yarn.lock`
- `package-lock.json`
- `bun.lock` / `bun.lockb`

Because no JS/TS package exists yet, `js-ts-dependency-upgrades/DAEMON.md` is intentionally configured with `N/A` commands and must no-op.

## Activation guidance (only after JS/TS is added)

When a JS/TS package is introduced, update `DAEMON.md` with concrete commands from the chosen package manager and remove the `N/A` values.

Suggested command shapes (replace sample package names with real repo dependencies):

- pnpm: `pnpm outdated`, `pnpm update react`, `pnpm update typescript --dev`, `pnpm install --lockfile-only`, `pnpm test`
- npm: `npm outdated`, `npm update react`, `npm update typescript --save-dev`, `npm install --package-lock-only`, `npm test`
- Yarn: `yarn outdated`, `yarn up react`, `yarn up typescript`, `yarn install`, `yarn test`
- Bun: `bun outdated`, `bun update react`, `bun update typescript`, `bun install`, `bun test`

Do not enable the daemon until the repository has both a manifest and lockfile that match the selected package manager.
