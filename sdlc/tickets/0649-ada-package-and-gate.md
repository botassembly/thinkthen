# 0649: Package Ada without fixed paths for darwin-arm64 and both Linux targets

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688, 0728

## Outcome

- Changes: `libraries/ada/check.sh`, `libraries/ada/thinkthen.gpr`, `libraries/ada/checks/native_session.py`, `libraries/ada/checks/full_case.py`, `libraries/ada/README.md`, `libraries/ada/README-PUBLIC.md`, `sdlc/tools/ada.json`, `sdlc/scripts/release-pack.d/ada`, `site/src/data/catalog/ada.mjs`.
- The gate builds into a folder from the shared scratch helper, not `/tmp/thinkthen-target-ada`, and finds the native library by the platform's file name, not `libthinkthen_c.so`.
- Ada adds darwin-arm64 and linux-arm64 to its supported and checked platforms in `sdlc/tools.json`, beside linux-x86_64.
- `thinkthen.gpr` selects linker options by target through a scenario variable. The release archive for each supported target carries that target's native library.
- The supported GNAT range is stated in the README. Its lowest and newest versions are both in the tool manifest, and both are tested.
- The existing fixture consumers stay; 0648 replaces them with the packet dumper when it deletes the session family they use.
- Version fields: the package release number stays. No format version is added.

### Proof

- On the M5 through `pm job run --machine m5 0649 SHA --memory 8 -- sh libraries/ada/check.sh`, the gate passes on darwin-arm64. Before the change the same command exits 77 because the manifest lists Ada as unsupported there.
- At the 0619 container checkpoint, the release suite passes the Ada gate in the arm64 container on the M5 and in the x86_64 container on Yellow, with both GNAT versions.

## Evidence

- `libraries/ada/check.sh` defaults `CARGO_TARGET_DIR` to `/tmp/thinkthen-target-ada` and names `libthinkthen_c.so` as the native library, so it builds outside the scratch rules and cannot find the macOS library.
- `sdlc/tools.json` lists only `linux-x86_64` for Ada under `libraries`, and its `gnat` tool only for Linux at 13.3.0. `sdlc/scripts/check_verdict.py platform` returns 77 for an unsupported platform, and `check.sh` exits with it.
- `thinkthen.gpr` links `-ldl -lpthread -lm` unconditionally beside `native/lib/libthinkthen.a`.
- The `ada` case of `sdlc/scripts/release-pack` copies one target's `libthinkthen.a` into `native/`.
- `libraries/ada/README.md` supports only Linux x86-64 with GNAT 13.3. The site's Ada entry in `site/src/data/catalog.mjs` names the per-target archive.
- 0618 already turned missing tools into failures, so the gate no longer exits 77 when a tool is missing.
- After 0620, the release workflow reads supported targets from the manifest, so no workflow file changes here.
