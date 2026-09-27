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
