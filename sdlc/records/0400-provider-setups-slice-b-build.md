# 0400 slice B: provider setup parser and captured settings

Status: in progress; independent source review and coordinator checkpoint pending. This record owns the lane 2 parser candidate based on main `110e6ac88`. Slice C owns concurrency defaults; slice D owns measured pages. No live job or ledger operation ran.

## Implementation

The existing backend entry accepts the ADR 0117 setup fields. All seven nonempty legal built-in subsets are accepted; built-in transport, key, model, path and wire overrides remain refused. Added entries reuse the existing BothSides form. Prices use the existing exact decimal parser. Inline profiles retain their original JSON bytes for the existing profile parser, including duplicate-field refusal. Configuration errors can carry the profile parser's value-free reason while preserving exit 5; top-level price errors retain their existing usage classification.

Named and exact canonical built-in posting-URL selection carry setup prices/profile independently of rates. Named settings survive explicit address/model overrides. Captured top-level prices have their own provenance below setup prices; explicit setters win regardless of order. The CLI has a separate edge path and applies both prices and profiles there, including check. Rust, Python and SQL continue using the captured builder. No public API was added, and no accepted precedence or profile semantics changed.

The existing packer splits multi-question requests at `max_questions`, including tag labels. The ticket's two-label refusal example therefore cannot establish a refusal under `max_questions: 1` without changing the accepted existing profile semantics. The consumer refusal witnesses use `max_evidence_bytes: 3`, which refuses before any send and can be overridden explicitly. The legal-subset edge table still includes `max_questions: 1`. This follows the accepted ADR's preservation rule; it introduces no API deviation.

## Proof so far

All builds use user systemd scopes with MemoryMax 12G, MemorySwapMax 1G, offline Cargo, two Cargo jobs and empty RUSTC_WRAPPER. Build launchers source the repository allow-list. Tests use scratch usage homes and fake loopback keys. The only key values in test source are fake markers. No credential file was created.

The configuration subset/canonical selection edge table passed 2 tests. The earlier named-backend focused run passed 29 tests with 2 child-only tests ignored; its parents execute those child halves. Python's focused captured-profile test passed 1 test with zero counted requests on refusal and one after explicit override. A final expanded named-backend run and DuckDB consumer build remain pending at this record revision. Settings validation passes against the lane binary: 66 rows, 68 flags, zero failures. Tickets and children validation pass. Offline policy passes after naming the new edge-table file as a test source under the existing boundary policy.

The first outside-in setup run found three fixture expectations needing correction: the profile parser deliberately says `is not valid JSON` for a duplicate field, and the evidence sentence itself is 44 bytes, so an explicit 20-byte profile still refuses. The witnesses now use a 100-byte explicit profile. These were test-fixture corrections, not changed parser or preflight behavior. No red-before-implementation claim is made; a targeted precedence mutation receipt remains pending.

Required lint, final ratchet measurement and fresh independent source review are pending. Full test, spec, surfaces, native and hosted checks did not run. The coordinator names the checkpoint and controls any later rebase or landing.

## Growth

The source/test growth earns the shared setup parser, typed selection and separated fallback provenance, plus independent legal-subset, canonical-route and outside-in refusal/price/wire/replay/consumer witnesses. Before raising the ratchet, the builder checked the existing rate, price and profile parsers and reused them; existing BothSides encoding is reused; no new parser, transport or surface setting was introduced. The final exact total and delta will be recorded after formatting and required lint.

Ian can overturn the accepted format and slice boundaries. The parent owns review, checkpoint, final rebase and landing.
