# 0520 — complete-facts-reader-compatibility

Status: OPEN.

Milestone: 0.2

Reviews: revision e94ab9b5c63ba851e308b1f0b3744371f1e1d4a8, accept

## Outcome

Python, TypeScript, Ruby and R public complete calls preserve the facts the engine emits instead of failing while reading them. This fixes a confirmed compatibility bug before the wider generated-results migration.

## Evidence

- Starts from: The PM's 2026-10-09 suspected-regression message and fresh pure-reader reproductions at 7d8f7b428. Python, TypeScript and Ruby reject all four newer facts observations; R rejects nullable token estimates and usage persistence. The full landing check also fails the R smoke with `invalid complete result`.
- Keeps: Existing public result shape, older recorded snapshots, distinct missing and null values, typed errors, failure facts, zero additional model calls, secrecy and Rust-owned observations. This bridge does not replace 0513's generated-results outcome.
- Changes: Admit and preserve `largest_request_bytes`, nullable `largest_request_estimated_input_tokens`, `token_estimate_method` and the shared `usage_persistence` observation in the four current readers and declared host types. Claim `libraries/python/thinkthen/_complete.py`, `libraries/python/thinkthen/complete.pyi`, `libraries/python/tests/test_complete.py`, `libraries/python/tests/test_complete_surface.py`, `libraries/python/tests/fixtures/complete.json`, `libraries/typescript/_complete.js`, `libraries/typescript/_complete.d.ts`, `libraries/typescript/tests/complete.test.mjs`, `libraries/typescript/tests/complete_public.test.ts`, `libraries/ruby/lib/thinkthen/complete.rb`, `libraries/ruby/tests/test_complete.rb`, `libraries/r/thinkthen/R/complete.R`, and `libraries/r/tests/complete.R`. Name any needed existing installed runner file before editing. Keep core and the C ABI unchanged.
- Proof: Reproduce rejection before the repair. Pass current engine-emitted facts through each public eager reader and terminal batch reader, including a saved call, unavailable token estimate and persistence observation, using existing saved or loopback exchanges. Check all observations survive, old snapshots still work, and no extra requests occur. Run one installed complete call per surface, and rerun the failed R smoke. Extend existing conformance cases rather than adding a checking framework.
- Defers: Generated result objects and presence across all languages (0513), language API migration, new proof machinery, paid calls and release management. Fix before further result-reader migration.
