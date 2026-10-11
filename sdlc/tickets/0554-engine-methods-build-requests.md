# 0554: Make the Rust Engine's per-function methods build Requests

Status: OPEN.

Milestone: 0.2

Depends on: 0551, 0552

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision 3875b55bd4784d43e1df5c72aa5c0a449bfa1f8f, reject

Reviews: revision f4345a84d734673b38c654a60f37e2f40832a6ff, accept

Reviews: revision 018d71671b851391aa92a5fd29057a61d46fad81, accept

## Outcome

The public Rust `Engine` admits every call through `Request`, the same as every binding. Each per-function method is a thin wrapper that builds a `Request`. The library carries no reader that only the command line uses.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), finding A3.
  - `decide`, `choose`, `score`, `tag` and `details` (`public/engine.rs:256-422`) and their `*_input` forms (`public/images/calls.rs:19-197`) go straight to `ask_all` and skip `Request` admission.
  - `public/` has 710 `pub fn` and 45 `impl Engine` blocks.
  - The result-shape checks are copied at `engine.rs:310,338,372`, `images/calls.rs:66,92` and `images/many.rs:151`.
  - `CallOptions` carries a reader for the command line only (`public/options.rs:118`). `pull.rs:186,303,378` branch on it, and `request/cli_atomic.rs` plus `execute_cli_request` (`execution.rs:136`) live in the library.
  - The session queue wakes every 10 ms while the producer is idle (`request/session_queue.rs:110`).
- Keeps: the public Rust API signatures, unless a deletion is reviewed, and every Rust public case.
- Changes:
  - Have the text methods call the input methods.
  - Make the per-function methods build a `Request`, and delete the duplicated checks.
  - Replace the command-line reader with a readiness hook that the command line supplies, and move command-line preparation under `cli/`.
  - Have the session queue wait for a wake signal or the deadline.
- Proof:
  - Rust public cases and CLI cases pass.
  - The product line count in `crates/` falls, and `sdlc/ratchet.json` falls with it.
- Defers: nothing.

## Progress

- 2026-10-11 started
- 2026-10-11 next: Bounded intake and snapshot repairs at b1f95b370 are under fresh code review; the combined source ceiling remains above the approved target and compilation awaits the owned 0549 campaign.
- 2026-10-11 next: Final engine repairs at 018d71671 passed both library clippy configurations, all 130 routine native-completion tests and policy; fresh delta review and the unmet combined source ceiling still block closure.
- 2026-10-11 next: Final code passed fresh review and all 130 routine native-completion tests, but the combined 0556 integration measures 190417 Rust lines before 0549 and still needs substantive cleanup before a candidate.
- 2026-10-11 next: Broader public checks exposed a preserved-message mismatch and a repeated control callback during a held send; both are being repaired in the combined core before cleanup and whole-ticket acceptance.
