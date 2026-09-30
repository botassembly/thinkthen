Status: Closed by the quick fix landed as `Land quick fix: release builds clear host setup, apt, offline crates, and library tests`. Found by the second release rehearsal, run 36780048676, on 2026-09-30. Owner: ticket 0128 Phase 3b. Resolution: `host-setup` in `sdlc/scripts/release-workflow` asks the Python that installed uv for its binary through `uv.find_uv_bin()`, reads that binary's version, and puts its folder first on the path for the rest of the step. It accepts `uv 0.9.17` alone or followed by a space and a build note. A mismatch prints the binary's path and the line it read. The host-setup self-test in `sdlc/scripts/workflows` now places a stray uv first on the path that fails if run, and its pinned uv prints a build note; all six uv cases failed on the old script. A new case feeds `uv 0.9.18` and requires the printed line.

Kind: bug

Pay when: before the next rehearsal dispatch.

Keeping it stops both macOS `build` jobs in `host-setup`, so no macOS file is built or smoked.

# Release host setup refuses its own uv on macOS

## The problem

`sdlc/scripts/release-workflow host-setup`, lines 238 and 239, runs `python3 -m pip install 'uv==0.9.17'` and then requires `uv --version` to equal `uv 0.9.17` exactly. On both macOS runners, `macos-15` and `macos-15-intel`, pip reported `Successfully installed uv-0.9.17`, and the next line failed:

```
release-workflow: uv version differs from pin
```

The same pattern passed for `maturin` a line earlier, so the Python scripts folder is on the path. The message does not print what `uv --version` read, so the log cannot tell a different `uv` earlier on the path from a longer version line. The uv 0.9.17 Linux wheel prints exactly `uv 0.9.17`. The two Linux `build` jobs skip this branch, because it runs only on macOS or with `RELEASE_SMOKE=1`. The Linux `smoke` jobs will reach it.

## A fix

Run the pinned copy by its interpreter, `python3 -m uv --version`, and print the line read in the failure message. Accept `uv 0.9.17` alone or followed by a space and a build note. Add a self-test case that feeds a longer version line and a different version through a fake `uv`.
