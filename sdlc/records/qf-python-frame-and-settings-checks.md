# Python frame and settings checker Quick Fix

Status: Accepted by fresh independent Medium code review at `069fe2c9` and integrated. Scope is the existing frame test fixture and the settings checker's Python source path; Python product and shared Rust source are unchanged.

## Cause and correction

`test_a_frame_keeps_types_and_nested_failures` uses a private listener that indexes scalar `asked["state"]` values to return one successful and one failed row. The default Max scheduler now packs the two frame rows into a dict state, so the listener raised `TypeError: unhashable type: 'dict'` before the test could assert Float64 score, list tag, nested failed structs, the empty frame, or exactly two sends. The existing test was reproduced red on current main. Its engine now sets `batch=1`, the current public API for the per-record fixture. Every prior dtype, value, nested failure, empty-frame and send-count assertion remains. The separately reviewed 0214 and 0233 captured listener tests retain default packing and exact batch-one body/digest coverage; this fixture adds no batch decoder or duplicate scheduler proof.

The settings checker still read `libraries/python/src/engine.rs` for Python `EngineBuilder` calls after those setters moved to `libraries/python/src/engine/settings.rs`. Inspection found the real `base_url`, `model`, `throttle`, `max_requests`, `max_request_bytes`, `cache_at`, `profile`, `timeout`, `max_retries`, `record` and `replay` builder calls in that child. Only `ENGINE_SOURCES["Python"]` changes. The bidirectional setter/cell comparison, all other port paths and its planted refusal checks remain unchanged. On the older warm CLI binary, the baseline table reported the eleven false Python mismatches plus two help flags absent from that stale binary. A focused current-main CLI rebuild removed that binary skew before the final table check.

## Focused proof

With `THINKTHEN_API_KEY` unset, pinned Python 3.13.5 and the warm native extension, `python -m pytest -q -p no:cacheprovider --tb=short tests/test_door.py::test_a_frame_keeps_types_and_nested_failures` went from the listener traceback above to **1 passed in 0.18 s**. The unchanged test still asserts its original stable column, failure and empty-frame contract. No adjacent frame case was needed because the fix is one test-local engine setting and those sibling proofs were retained from the reviewed 0233 run.

`flock -o /run/user/1000/thinkthen-codex-1.lock cargo build --locked --offline --package thinkthen --bin thinkthen --quiet` refreshed only the CLI needed by the checker. With `target/debug` on `PATH`, `python3 sdlc/scripts/settings --self-test` reported **10/10 cases hold**, and `python3 sdlc/scripts/settings` reported **46 rows, 54 flags, 6 environment names, 15 question-file keys, 0 failures**. The planted negative refusal proof and every other surface path remain in force. `python -m py_compile` on both edited Python files and `git diff --check` passed. The Python source/test ratchet remained exactly **3,431/3,431**; no ceiling changed. No full surface, gem, wheel, provider or stress run was made.

## Review boundary

Review the one engine `batch=1` fixture line against its retained assertions and the one `ENGINE_SOURCES` path against the actual builder calls. No test, assertion, helper or product behavior was deleted. The stronger existing default Max and request-identity cases remain in 0214/0233. Root owns landing and any wider gate.

## Independent review

The reviewer confirmed that every frame assertion remains and that the separate default-Max listener test still pins one packed request, three batch-one requests, exact captured bodies and digests. It independently passed the frame case, that retained batching case, the checker self-test (10/10) and the actual 46-row settings table with zero failures. Source and the worktree remained unchanged during review. This restores two inherited checks; it does not close a new product item or claim a broad suite pass.
