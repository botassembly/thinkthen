# 0148 engine settings: preflight

Status: design amendment accepted on 2026-09-27. It started at ticket branch `7089d431`, merged with main `8ff1f0fe`. No runtime file changed and no runtime gate ran. A fresh read-only reviewer accepted corrected amendment `2d5c842a`; the coordinator accepts it within the authorized settings outcome. The exact additional file claim must land before the builder starts.

## Findings

- The accepted body calls for a two-way settings-page check in `sdlc/scripts/settings`, but its `opens` omitted the script. The spec rung already runs that script. The amendment adds it to `opens`.
- The named older-recording-folder issue requires replacing the inaccurate library error. Main's `crates/thinkthen/src/public/error.rs:198` still says `the recording folder uses a retired layout`; `cli/failure/recording.rs:56-60` instead gives the actionable command sentence. The amendment adds `public/error.rs` to `opens` and fixes only that match arm.
- `engine/recorder/identity.rs:36-50` already allows an unmarked folder when `writing` is false and rejects one with entries when writing. `specification/recording.md:40` promises this. Demo 27 has one digest entry and no marker. The new `EngineBuilder::replay` can use that engine path without engine edits. A regression copies the fixture before reading it.
- `conformance/consumer/consumer/tests/public/main.rs` imports test modules; it is inside the accepted `conformance/consumer` claim. The new settings test module needs one parent import. No other parent import was substantiated. `public/settings.rs` has 289 physical lines and `public/error.rs` has 242, below the 500 nonblank-line ceiling. The proposed one-arm error edit fits the accepted source budget. The Rust edge test allowance alone rises by 35 lines to 185.
- Calibration profile remains separate ticket 0203. SQL settings remain ticket 0149. Main includes ticket 0200; this amendment changes no SQL function or engine contract.

## Proof and handoff

Read-only code and specification checks above confirm the issue. No loopback run was needed before the builder creates `replay`, and no paid call is authorized. `git diff --check` passed and `sdlc/scripts/tickets` reported zero evidence failures. The fresh reviewer should inspect the exact new `opens` entries, the old-folder rule, the copied-fixture regression, the error wording, and the 185-line edge-test budget. A builder then follows the accepted ticket and the amendment. Ian can overturn the library wording or added test budget.

The fresh amendment review found that a loopback count could not observe the unchanged demo 27 fixture’s saved remote address. The corrected proof keeps that fixture unchanged, checks a no-key replay hit and an exact local legacy-folder miss, and verifies marker and entry state. The existing shared settings case owns the counted loopback no-send proof. A fresh bounded rereview returned ACCEPT at `2d5c842a`. It checked the separate legacy-folder and counted loopback proof, retained identity rules, exact added file scope, unchanged engine, and the 185-line edge-test budget.


On 2026-09-27 the coordinator accepted the independently reviewed implementation-budget amendment in ticket 0148. The worker had completed focused shared settings checks on all seven surfaces but stopped at the size limits; the internal request remained in its CLI log. The coordinator owns the missed handoff. The fresh review supports the measured source and runner growth, requires the remaining folder and C edge proof, and flags Ruby bulk draining for final code review. No implementation has landed; the uncommitted candidate still needs its remaining proof, exact ratchets and independent semantic review.

## Implementation checkpoint, 2026-09-27

The settings implementation is checkpointed on the ticket branch under the accepted measured budgets. Focused shared settings cases passed on the command, Rust, Python, TypeScript, Ruby, R and C; the Rust old-folder and relate profile cases passed; the settings checker self-test passed ten plants; root Clippy and offline SQL compiles passed before this checkpoint. The Ruby adapter drains `decide_many` results so a request-limit refusal follows the public iterator and records the two earlier requests. The empty `impl Ask {}` has been removed. This is work in progress: the full Rust folder-rule table, C boundary and held-arm cases, required mapping red plants, exact final ratchets and fresh semantic review remain. No engine source, dependency, paid service or main branch changed. SQL settings remain ticket 0149, and calibration profile remains ticket 0203.
