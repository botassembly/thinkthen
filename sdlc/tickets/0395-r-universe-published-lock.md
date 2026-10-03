# 0395: Fix the R package's published-shape lock update that failed on R-universe

Status: in progress. Lane claude-3. Branch `ticket/0395-r-universe-published-lock`. Parent: ticket 0128 phase 4. Cherry-picked to `release/0.1` under ADR 0116 item 5, so the fix ships in 0.1.2.

Milestone: 0.1

## Outcome

1. In the published shape, `libraries/r/thinkthen/tools/config.R` syncs the shim's lock with `cargo update --workspace` after it rewrites the engine dependency from a path to `=VERSION` on crates.io. The lock gains the registry `thinkthen` entry and keeps every other pin. The `cargo build --locked` in `Makevars` then succeeds.
2. R-universe's two steps succeed on a clean copy of `libraries/r/thinkthen`: `R CMD build`, then `R CMD INSTALL` of the tarball, with network and an empty cargo home.
3. The outside-the-repository step of `libraries/r/check.sh` serves the engine the way crates.io does. A private cargo home replaces crates-io with a local directory source. That source holds the tarball's vendored registry tree and `cargo package`'s copy of `thinkthen`. The step stays offline.
4. The step also checks that the lock's `thinkthen` entry comes from the crates.io registry and that every package keeps its version.

## Evidence

- Starts from: R-universe run 37126788263, "thinkthen 0.1.1", job "Source pkg and vignettes" (id 111213656739).
  - R-universe cloned tag `v0.1.1` at `9463cef05`, ran `R CMD build` on `libraries/r/thinkthen`, and installed `thinkthen_0.1.1.tar.gz` on Ubuntu with R 4.6.
  - configure stopped with `error: package ID specification `thinkthen` did not match any packages`, the hint `a package with a similar name exists: `thinkthen-r``, and `Error: cargo could not resolve thinkthen 0.1.1 from crates.io`.
  - crates.io serves `thinkthen` 0.1.1, so the registry was not the cause.
  - Reproduced on blue in a `--rm` `rocker/r-ver:4.6` container: R 4.6.1, cargo 1.99.0, rustup stable, network on. `R CMD build` then `R CMD INSTALL` of a clean `git archive v0.1.1 libraries/r/thinkthen` gave the same three lines.
  - Root cause: the published shape rewrites `src/rust/Cargo.toml` so `thinkthen` comes from crates.io, then runs `cargo update --package thinkthen`. The shipped `Cargo.lock` holds `thinkthen` only as a path package with no `source`. Cargo keeps a sourceless lock entry only while the manifest still names that path. With the path gone, cargo drops the entry, so the spec `thinkthen` matches no locked package and `cargo update` refuses it.
  - Why the gate passed: `check.sh`'s outside step (ticket 0128) served the engine through `[patch.crates-io] thinkthen = { path = ... }`. A patch is a path source, so the old sourceless entry survived and `cargo update --package thinkthen` matched it. The check never saw a registry `thinkthen`.
  - Experiment: the same container build with `cargo update --workspace` in place of `--package thinkthen` downloaded `thinkthen v0.1.1` from crates.io, built, installed and loaded 0.1.1.
- Keeps: the three shapes and how `config.R` picks them; the published shape's `=VERSION` rewrite and its single-path-dependency guard; `Makevars`' `--locked` build; the tarball and repository shapes' `--locked --offline` builds; `check.sh`'s rule that every `cargo build|test|clippy|run|vendor|package|fetch` call carries `--locked --offline`; the outside step's offline run, its empty target, and its answer check.
- Changes: `libraries/r/` only.
  - `libraries/r/thinkthen/tools/config.R`: `cargo update --workspace` and a comment saying why `--package thinkthen` cannot match.
  - `libraries/r/check.sh`: the outside step builds a directory source from the tarball's `vendor/registry` and the packed crate, replaces crates-io with it in the private cargo home, and checks the lock's `thinkthen` source and the unchanged pins.
- Proof:
  - `bash libraries/r/check.sh` passes on this branch. With `config.R` put back to `--package thinkthen`, its outside step fails with the R-universe error.
  - On blue, in `--rm` `rocker/r-ver:4.6` containers running `R CMD build` and then `R CMD INSTALL` of the tarball, with the image removed afterwards:
    - Against real crates.io: the `v0.1.1` package folder with this branch's `config.R` installs and loads 0.1.1. crates.io has no 0.1.2 until the release publishes, so 0.1.1 is the newest real proof.
    - At 0.1.2: the cherry-picked `release/0.1` package folder installs and loads 0.1.2. Its cargo home replaces crates-io with a directory source of `cargo package`'s 0.1.2 crate and its vendored dependencies, as the check does.
  - `python3 sdlc/scripts/tickets` and `sdlc/scripts/lint` with the private-names list.
  - On the new `release/0.1` head, `bash libraries/r/check.sh`.
- Defers:
  - R-universe's own rebuild. It runs only after the `v0.1.2` release publishes.
  - The other R-universe jobs (Windows, macOS, Wasm, Linux binaries and docs). They were skipped because the source job failed, so 0.1.2 is their first run.
  - The `sed: can't read .../man/*.Rd` line in the same log. The package ships no man pages; R-universe continued past it.

## What Ian can overturn

- Building the published shape against crates.io with network, rather than vendoring every crate into the GitHub tree. R-universe source builds have network, and ticket 0128 chose this shape.

## What the build taught us
