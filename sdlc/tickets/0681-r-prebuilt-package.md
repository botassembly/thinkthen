# 0681: Ship R binaries through R-universe and vendor crates for source builds

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0682, 0688

## Outcome

- Users install the R-universe binary with no Rust toolchain.
- The source package vendors its crates, so a source build with Cargo works offline.
- `tools/msrv.R` is deleted.
- The R conformance runner becomes a packet dumper following the 0642 contract, and the R fixture scripts it replaces are deleted.

### Proof

- On the M5 through pm job, a clean R library with no Rust toolchain on the path installs the R-universe-format binary built for the candidate from a local repository and runs a named call. Its dump matches the 0642 oracle.
- In the pinned arm64 container, an offline source install with Cargo succeeds from the vendored source package.

## Evidence

- **Binaries exist but are not the tested path.** R-universe already serves binary R packages (`.github/workflows/install-check.yml:182-198`, `sdlc/scripts/install-check-r-binary-test.py:16`). The routine check still installs from source with the repository's pinned Rust toolchain (`check.sh:45-48`), so an ordinary user's install is never tested.
- **Source builds fetch crates.** `libraries/r/thinkthen/DESCRIPTION` declares `SystemRequirements: Cargo, rustc`, which is normal for an extendr package, but the source package does not vendor its crates, so an offline source build fails.
- **Unix only.** The package declares `OS_type: unix`.
- **Boilerplate.** `tools/msrv.R` is extendr boilerplate.
- **Fixture consumers.** About 25 R scripts drive conformance.
