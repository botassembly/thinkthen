# The R check rebuilds thinkthen and every dependency on each run

Status: closed in ticket 0335 slice 2.
Kind: debt
Pay when: 0335 slice 2 lands, or sooner if the machine stays loaded.
Debt: 019
Severity: medium
Paid: 2026-09-30

Seen 2026-09-30 at load 17 to 20: the R package install compiles thinkthen and all of its Rust dependencies in a fresh temporary folder each time the R check runs, because the build uses no shared target folder. Every other binding reuses its lane's build. Keeping it costs several minutes of all cores per R check, and it adds load that makes other lanes' timing tests fail.

Fix: point the R install's Cargo build at a lane-local target folder (for example through `CARGO_TARGET_DIR` in the R check script), keeping the install itself in a scratch folder. Owner: none yet.

Paid in ticket 0335 slice 2. `tools/config.R` now takes a named `CARGO_TARGET_DIR` in the repository shape and keeps it after the install, and the R check and its replay smoke install with `target/r` in the lane. A second smoke install took 4.8 s, down from 58.4 s. The tarball and outside-the-repository installs still build in scratch, because they prove the shipped shapes.
