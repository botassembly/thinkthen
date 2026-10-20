# 0650: Package and check COBOL on darwin-arm64 and both Linux targets

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0740

## Outcome

- Changes: `libraries/cobol/check.sh`, `libraries/cobol/checks/`, `libraries/cobol/README.md`, `libraries/cobol/README-PUBLIC.md`, `sdlc/tools/cobol.json`, `sdlc/scripts/release-pack.d/cobol`, `site/src/data/catalog/cobol.mjs`.
- COBOL adds darwin-arm64 and linux-arm64 to its supported and checked platforms in `sdlc/tools/cobol.json`, beside linux-x86_64. The release archive for each carries that target's native library.
- The gate and its drivers find the native library by the platform's file name, not a fixed `libthinkthen.so.0`, and `checks/exports.py` confirms the generated copybook layout on each target.
- The README states the platforms the archive supports. It no longer says the declarations fit only Linux x86_64.
- The per-case fixture drivers stay; 0651 replaces them with one packet dumper when it deletes the session family they use.
- COBOL's Linux tools, including cobc, are declared in `sdlc/tools/cobol.json` and join the Linux image through 0740; a missing tool fails the check, never skips.

### Proof

- On the M5 through `pm job run --machine m5 0650 SHA --memory 8 -- sh libraries/cobol/check.sh`, the gate passes on darwin-arm64. Before the change the same command fails on the missing `cobc` tool, because the manifest declares `cobc` for Linux only.
- Linux arm64: `pm job run --machine m5 0650 SHA --memory 16 -- sdlc/scripts/linux-check surface libraries/cobol`; Linux x86_64: the same command with `--machine gator --memory 7`.

## Evidence

- `sdlc/tools.json` lists only `linux-x86_64` for COBOL under `libraries`, and its `cobol` tool (`cobc` at `4.0-early-dev.0`) only for Linux. `sdlc/scripts/check_verdict.py platform` returns 77 off those platforms, and `libraries/cobol/check.sh` exits with it.
- `check.sh`, `checks/session.py`, `checks/smoke.py` and `checks/native_parity.py` copy the native library to `libthinkthen.so.0` or default to `libthinkthen_c.so`.
- `libraries/cobol/README.md` says the generated declarations support the Linux x86_64 layout and must be regenerated for another platform. All three targets are LP64; `checks/exports.py` compares the generated storage with the installed C compiler's layout, so it shows whether one copybook serves all three.
- The `cobol` case of `sdlc/scripts/release-pack` unpacks one target's C archive into `native/`.
- 0652 chooses the supported GnuCOBOL range. Until then the macOS gate uses the GnuCOBOL release the M5 provides, and the developer records that version in the manifest.
- After 0620, the release workflow reads supported targets from the manifest, so no workflow file changes here.
