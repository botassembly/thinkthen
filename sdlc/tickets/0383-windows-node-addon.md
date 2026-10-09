# 0383: Ship the Node addon for Windows x86-64

Status: OPEN.

Milestone: 0.2

Depends on: 0498
Depends on: 0501

Reviews: revision 4d59e7c6a, accept

## Outcome

The final npm package carries a `win32-x64` addon beside the existing platform addons. On Windows a caller installs the package and makes a real call with no library-path setting. Linux and macOS installs behave as before. The TypeScript README names Windows as supported.

## Evidence

- Starts from: Windows stage 0 (0373) did not build the addon. `loader.js` refuses `win32`, its `SHIPPED` list names four platforms, `package.json` limits `os` and `files`, and `check.sh` pins the `win32` refusal. One npm package carries every platform's addon, so a packing mistake breaks every platform. The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) brings Windows into 0.2; the [0521 assessment](../records/0521-surface-contract-assessment.md) replaced the earlier deferral.
- Keeps: Each existing platform's addon name and bytes. The refusal sentence for a platform the package does not ship.
- Changes: The release workflow builds the `win32-x64` addon. `loader.js` and `package.json` accept it. Add a Windows Node pin. Derive the addon list from the 0501 inventory, keeping no second literal addon count. The `check.sh` refusal case becomes a load case on the runner. Declare any native runtime dependency in the package design from 0517. Claim `libraries/typescript/**` package files, `sdlc/scripts/npm-assemble` and `.github/workflows/windows.yml`, coordinating with 0498 and 0501.
- Proof: Install the final npm artifact from 0498 and 0501 on `windows-2025` with no user library-path override or source-tree fallback, make one call and count its loopback requests. The packed listing holds every inventoried addon. Linux and macOS TypeScript checks stay green. Windows execution is owed to the first authorized candidate, and the ticket stays open until it passes.
- Defers: Whether the addon needs the MSVC runtime beside it. The builder checks on a clean runner and records the answer in the README. Candidate dispatch and publication wait for Ian's permission.
