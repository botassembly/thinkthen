# 0383: Windows stage 1: the Node addon ships for Windows x86-64

Status: ready. Deferred to 0.3 under Ian's 2026-10-05 scope ruling; the ticket stays open.

Milestone: 0.2

Depends on: 0498
Depends on: 0501

## Outcome

- The release workflow builds a `win32-x64` addon. `npm-assemble` packs five addons into the one npm package.
- On Windows the package loads its addon and answers from a loopback backend.
- `libraries/typescript/README.md` names Windows. Linux and macOS addons are unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0) did not build the addon. `loader.js` refuses `win32`, and its `SHIPPED` list names four platforms. `package.json` limits `os` and `files`. `check.sh` pins the `win32` refusal. The Node pin covers Linux x64 only. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 100 to 250 lines and 1 slice, with medium Linux and macOS risk: one npm package carries every platform's addon, so a packing mistake breaks every platform. No experiment preceded this ticket.
- Keeps: each existing platform's addon name and bytes in the package. The refusal sentence for a platform the package does not ship.
- Changes: `loader.js` adds `win32-x64` to `SHIPPED`. `package.json` adds Windows to `os` and the addon to `files`. A Windows Node pin. `npm-assemble` takes five addons. The `check.sh` refusal case becomes a load case on the runner.
  Claim `libraries/typescript/**`, `sdlc/scripts/npm-assemble` and `.github/workflows/windows.yml`. Windows execution is owed to the first authorized candidate; do not close until it passes.
- Proof: the load case passes on `windows-2025` and counts loopback requests. An `npm pack` listing holds all five addons, and the Linux and macOS TypeScript checks stay green.
- Defers: the main unknown, whether the addon needs the MSVC runtime beside it. The builder checks on a clean runner and records the answer in the README.

## Surface assessment amendment

The 2026-10-09 ruling supersedes the old 0.3 deferral prose; this is 0.2 Windows qualification. Consume the final npm artifact and inventory from 0498, 0501 and the relevant 0517 package slice, including declared native runtime dependencies. Derive target inventory instead of retaining a second literal addon count. Install with no user library-path override or source-tree fallback and count the real call. Independent language-family completion is not a prerequisite. Windows execution still waits for the first authorized candidate; this ticket stays open until that native proof passes.
