# 0734: Start the Linux gate router before resolving container tools

Status: COMPLETE.

Milestone: 0.2.1

Phase: 1

Depends on: 0619, 0726

Reviews: revision 4b5fb48, accept

## Outcome

- Changes: `sdlc/scripts/gate`, `sdlc/scripts/tools-self-test.py`, `sdlc/scripts/README.md`.
- The configured gate starts the existing host container router with host Python before resolving tools owned by the container image. Preserve absolute host Docker capture before isolation.
- Actual checks inside the container and native macOS tests keep the existing job entrypoint. Tool versions, image pins, check selection, limits and command exits stay unchanged.
- Extend the existing GateRouting regression to expose the bootstrap cycle on a Linux host without the image tool prefix. Preserve its Docker-path and native-test routing assertions.
- Add no resolver change, helper, runner, selector, host installation, permission change or cache workaround.

### Proof

- Run the existing `python3 sdlc/scripts/tools-self-test.py GateRouting` through an admitted job. Before the repair, the regression exposes host-side resolution of container-only tools; after it, the router starts with host Python and the actual checks remain isolated through job.
- Run the configured closing gate on the pushed repair. Then resume the existing Yellow `gate package` checkpoint on the repaired main revision and retain its actual exit; missing or failed checks never count as passes.

## Evidence

The phase 1 checkpoint on main d81fb759ad230f1070bf5fe56af6110666827e60 failed before Docker construction. Yellow job 1792498503807-704028 exited 2 after 2348 ms: Rust resolution tried the unwritable container prefix and isolated PATH selected the system Node instead of the pinned host tool. Read-only investigation found inherited Rust-root variables unset and no stale image prefix. The host router help runs with host Python. The cycle is gate:8 calling job for the router, job:20-23 selecting common, and tools.py:364 selecting the container prefix on Linux before that router can build the image. Existing GateRouting replaces job with a stub and therefore missed the cycle. This repairs the container routing outcome of 0619 and 0726; 0691 retains the operational checkpoint receipt.

## Progress

- 2026-10-20 started
- 2026-10-20 next: Fresh review accepted 4b5fb48d9 after intended before-repair job 1792499331292-70163 failed on missing fixture tool and final job 1792499439666-78533 passed all routing status isolation and syntax checks, preserving intermediate fixture failure 1792499414413-75239; this gate-definition repair lands separately to unblock the Yellow checkpoint and now needs its configured closing gate.
