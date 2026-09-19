# Review leftovers from ticket 0006

Found 2026-09-19 by the independent review of ticket 0006. The review said merge. These five points were left unfixed, and ticket 0007 checks each one it touches.

1. `sdlc/scripts/live` reads `THINKTHEN_LIVE_LEDGER`, which lets any caller point the spend limit at a throwaway file. The variable exists only for the script's own test. The script should accept it only under the test, or the test should reach the ledger another way.
2. `sdlc/scripts/demos` captures an empty folder name when a page writes `--replay` with the folder in backticks, and it then skips the folder guard silently. Demo 01 does not hit it. The script should fail loudly on an empty name.
3. `crates/thinkthen/tests/harness/mod.rs` opens with `#![allow(dead_code)]`. `rust-standards.md` says no test file pastes a suppression at its top.
4. `sdlc/records/0006-the-two-variables-the-live-script-and-the-first-green-demo.md` says the ratchet is 4531 and that fourteen demos are red. The ratchet is 4520 and thirteen are red.
5. `--url ""` reports the ad-hoc message where the blank-address message would be clearer. Ticket 0007 removes `--adapter`, so the ad-hoc message itself changes there.
