# 0383: Qualify the Node package on Windows x86-64

Status: COMPLETE.

Milestone: 0.2

Depends on: 0498
Depends on: 0530

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision 36382970a, accept

## Outcome

On a real Windows host, a caller installs the final npm package and makes a real call with no library-path setting. 0498 builds the `win32-x64` addon and its loader changes, and 0530 assembles the final package. This ticket proves them on Windows at the first authorized candidate.

## Evidence

- Starts from: Windows stage 0 (0373) did not build the addon. `loader.js` refuses `win32`, its `SHIPPED` list names four platforms, `package.json` limits `os` and `files`, and `check.sh` pins the `win32` refusal. One npm package carries every platform's addon, so a packing mistake breaks every platform. The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) brings Windows into 0.2; the [0521 assessment](../records/0521-surface-contract-assessment.md) replaced the earlier deferral.
- Keeps: Each existing platform's addon name and bytes. The refusal sentence for a platform the package does not ship.
- Changes: Run 0498's Windows load case against 0530's final npm artifact on the Windows runner. Record whether the addon needs the MSVC runtime beside it. If it does, add the dependency to 0517's package design and to the npm package. Claim `.github/workflows/windows.yml`, plus the TypeScript package files only if the runtime must be added.
- Proof: Install the final npm artifact assembled by 0530 on `windows-2025` with no user library-path override or source-tree fallback, make one call and count its loopback requests. The packed listing holds every inventoried addon. Linux and macOS TypeScript checks stay green. Windows execution is owed to the first authorized candidate, and the ticket stays open until it passes.
- Defers: Building the addon belongs to 0498 and assembly to 0530. Candidate dispatch and publication wait for Ian's permission.
