# 0681: Ship R binaries through R-universe and test the binary install

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0688

## Outcome

- Changes: `libraries/r/check.sh`, `libraries/r/thinkthen/tools/msrv.R`, `libraries/r/README.md`, `libraries/r/README-PUBLIC.md`.
- Users install the R-universe binary with no Rust toolchain, and the routine check tests that path: `check.sh` builds a binary package from the source tarball, serves it from a local file repository in R-universe layout, and installs it into a clean library with no `cargo` or `rustc` on the path.
- The source tarball keeps its vendored crates, so a source build with Cargo works offline.
- `tools/msrv.R` is deleted.
- The R packet dumper is 0713.

### Proof

- On the M5: `pm job run --machine m5 0681 SHA --memory 8 -- sh -c 'cargo build --locked --offline -p conformance-backend && sh libraries/r/check.sh'`. The binary install with no Rust toolchain on the path runs a named call. Before the change, the check installs only from source with `R CMD INSTALL` and the pinned Rust toolchain.
- The offline source install from the vendored tarball runs in the pinned arm64 container at the 0619 container checkpoint.
- The R-universe Linux binary is proven on the candidate in 0687 through the `r-universe` install-check channel.

## Evidence

- **Binaries exist but are not the tested path.** R-universe already serves binary R packages: `r_universe` in `sdlc/scripts/install_check_channels.py` reads `botassembly.r-universe.dev/bin/linux/.../src/contrib`, and `sdlc/scripts/install-check-r-binary-test.py` pins that index. The routine check in `libraries/r/check.sh` builds the source tarball with `tools/make-tarball.sh` and runs `R CMD INSTALL` with `RUSTUP_TOOLCHAIN` set to the repository's pin, so an ordinary user's binary install is never tested.
- **Vendoring is on main.** `libraries/r/tools/make-tarball.sh` already vendors the `thinkthen` crate from `cargo package` and the registry tree from `cargo vendor`, and writes `src/rust/.cargo/config.toml` to replace crates.io. `part_r` in `sdlc/scripts/release-pack` builds the release tarball with it. The original claim that the source package does not vendor its crates is stale.
- **Unix only.** `libraries/r/thinkthen/DESCRIPTION` declares `OS_type: unix` and `SystemRequirements: Cargo, rustc`.
- **Boilerplate.** `libraries/r/thinkthen/tools/msrv.R` is extendr boilerplate. `tools/config.R` and `configure` do not call it.
- 0682 no longer blocks this ticket: the binary install test does not depend on the thin binding, and both edit `check.sh` only in different places. The dumper that needed 0682 moved to 0713.

## Amendment, 2026-10-19

Split the R packet dumper into 0713 and narrowed this ticket to the binary install test and the boilerplate deletion. Vendoring already landed.
