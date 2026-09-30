# The R check rebuilds thinkthen and every dependency on each run

Status: open.
Kind: debt
Pay when: 0335 slice 2 lands, or sooner if the machine stays loaded.
Debt: 019
Severity: medium

Seen 2026-09-30 at load 17 to 20: the R package install compiles thinkthen and all of its Rust dependencies in a fresh temporary folder each time the R check runs, because the build uses no shared target folder. Every other binding reuses its lane's build. Keeping it costs several minutes of all cores per R check, and it adds load that makes other lanes' timing tests fail.

Fix: point the R install's Cargo build at a lane-local target folder (for example through `CARGO_TARGET_DIR` in the R check script), keeping the install itself in a scratch folder. Owner: none yet.
