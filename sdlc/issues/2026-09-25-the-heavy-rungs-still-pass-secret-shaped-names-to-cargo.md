# The heavy rungs still pass secret-shaped names to cargo and its tests

Status: Open.

Filed on 2026-09-25 by ticket 0127, as its ticket said the landing would. No key was used, and no request left the machine.

## What happens

Ticket 0127 made every test child build its environment from nothing, and `heavy-lock` now unsets each stray `THINKTHEN_*` name before a heavy rung runs. The rung itself still runs under the caller's whole shell. `cargo`, each test binary it starts, and each surface's `check.sh` see every secret-shaped name the developer's shell holds, such as an unrelated service key. A test that prints its own environment, or a crash report that captures it, would print those names. The child helpers keep them out of each grandchild. Nothing keeps them out of the test process itself.

## What would fix it

Run each heavy rung under an allow list: `PATH`, `HOME`, `LANG`, the toolchain names (`CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `CARGO_TARGET_DIR`, `RUSTC_WRAPPER`), the lock names, and the two path overrides `heavy-lock` keeps. Refuse or drop any other name that looks like a secret. The coordinator deferred this from 0127 because each surface's toolchain needs its own measured list, and a wrong list fails a rung on one machine only.

## Done when

A heavy rung started with a planted `FAKE_SERVICE_API_KEY` runs a test that finds the name absent, and a toolchain the rung needs still works on each machine that runs rungs.
