# 0650: Package and check COBOL on darwin-arm64 and both Linux targets

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- Changes: `libraries/cobol/check.sh`, `libraries/cobol/checks/`, `libraries/cobol/README.md`, `libraries/cobol/README-PUBLIC.md`, `sdlc/tools.json`, `sdlc/scripts/release-pack`, `site/src/data/catalog.mjs`.
- COBOL adds darwin-arm64 and linux-arm64 to its supported and checked platforms in `sdlc/tools.json`, beside linux-x86_64. The release archive for each carries that target's native library.
- The gate and its drivers find the native library by the platform's file name, not a fixed `libthinkthen.so.0`, and `checks/exports.py` confirms the generated copybook layout on each target.
- The README states the platforms the archive supports. It no longer says the declarations fit only Linux x86_64.
- The per-case fixture drivers stay; 0651 replaces them with one packet dumper when it deletes the session family they use.

### Proof

- On the M5 through `pm job run --machine m5 0650 SHA --memory 8 -- sh libraries/cobol/check.sh`, the gate passes on darwin-arm64. Before the change the same command exits 77 because the manifest lists COBOL as unsupported there.
- At the 0619 container checkpoint, the release suite passes the COBOL gate in the arm64 container on the M5 and in the x86_64 container on Yellow.

## Evidence

- `sdlc/tools.json` lists only `linux-x86_64` for COBOL under `libraries`, and its `cobol` tool (`cobc` at `4.0-early-dev.0`) only for Linux. `sdlc/scripts/check_verdict.py platform` returns 77 off those platforms, and `libraries/cobol/check.sh` exits with it.
- `check.sh`, `checks/session.py`, `checks/smoke.py` and `checks/native_parity.py` copy the native library to `libthinkthen.so.0` or default to `libthinkthen_c.so`.
- `libraries/cobol/README.md` says the generated declarations support the Linux x86_64 layout and must be regenerated for another platform. All three targets are LP64; `checks/exports.py` compares the generated storage with the installed C compiler's layout, so it shows whether one copybook serves all three.
- The `cobol` case of `sdlc/scripts/release-pack` unpacks one target's C archive into `native/`.
- 0652 chooses the supported GnuCOBOL range. Until then the macOS gate uses the GnuCOBOL release the M5 provides, and the developer records that version in the manifest.
- After 0620, the release workflow reads supported targets from the manifest, so no workflow file changes here.
