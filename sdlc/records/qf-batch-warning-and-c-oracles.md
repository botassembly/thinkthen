# Batch warning and C oracle Quick Fix

The saved-question contract in `specification/question-file.md` says a file without an authored threshold has no tuned-for batch setting. Record 0472 establishes the same distinction for complete results. `public/results/member.rs` had begun treating every saved batch as calibration metadata. The shared helper now requires `authored_threshold` before comparing the saved batch, or the established default tuning batch of one, with the running setting.

The existing C batch-metadata assertions retain both warning sides, kind/count values, warning absence for batch-only decide and dynamic choose files, and their request count. The existing native legacy-projection test now expects warning absence for its score file, which cannot carry a threshold. Its projection equality and attempt assertions remain present.

The three other C failures came from stale expectations. The request-estimate contract in `specification/backends.md` and record 0461 adds `largest_request_bytes`, `largest_request_estimated_input_tokens` and `token_estimate_method` to plan and call facts. The golden test preserves every original reply byte and optional-value rule while inserting only those three fields. Eleven independent encoded-body literals pin the outgoing wording, readings, question order and framing. The existing capture arm checks all sixteen successful request bodies exactly, including both recognition stages. A separate cold capture checks the unchanged annotation and scalar inputs used by the malformed arm. Each expected maximum comes from the pinned bodies for that call, and its token estimate uses independent integer ceiling arithmetic. The explicit capture route alone normalizes to the original test route during reply comparison. The malformed reply, failure facts, legacy values and usage-persistence expectations remain unchanged.

The plan expectation adds the same three fields in their established order, using its existing independently pinned 182-byte body and 166-token upper estimate. Missing-key, no-send and refusal-sentinel checks remain present. Record 0509 requires a started deadline to retain its configured budget and original expiration. The C deadline driver now requires the exact one-second message. Its budget remains one second; the unchanged source test still requires one completed record, two actual requests and two listener requests. The native public-controls regression verifies that starting options again does not reset expiration.

## Evidence

The prior baseline in `target/qf-legacy-annotation-admission/baseline.log` retains the unchanged C warning failure. This repair's initial golden run failed on the missing request-estimate fields before updating the oracle. Logs belong to `target/qf-batch-warning-and-c-oracles/`.

All four focused C cases passed. The existing native legacy-projection suite passed four cases. Public controls passed twenty-five cases with one declared ignored case. The full C all-target run passed twenty-two library tests and seventy-nine door tests. Its shared conformance report passed fifty-five applicable cases and explicitly omitted the one internal-invariant injection case. Clippy then refused the enlarged golden test function. Its expected-output calculation moved to one private helper; the affected golden case and lint checks were rerun without repeating the full C suite.

The final affected golden test, C all-target Clippy and native library/projection/control Clippy with warnings denied passed. Root and C formatting, both measured Rust ratchets, repository policy and whitespace checks passed. Policy reports existing large-file warnings outside this change. Heavy checks used owned user scopes with an 8 GiB memory cap, a 1 GiB swap cap and two build jobs, a cleared environment and the existing helper's owned configuration/cache/state folders. Every job completed and was reaped. The lane allocated 40,730,900 KiB, below 40 GiB, and warm builds remain present.

The root Rust ceiling decreases from 176011 to 176009. The C Rust ceiling increases from 18273 to 18332 for independent request pins and the existing golden test's capture and expectation logic. Repeated encoded requests share literals; the original golden replies remain the sole exact response oracle. No fixture framework, cache-key computation, hash receipt or new public API was added.

## Review and integration

The combined landing lint found the C-language ceiling still counted one removed deadline-fixture line. The existing measurement is 2752; lowering its metadata fixes that gate without changing the tested C source.

A fresh read-only reviewer accepted source 2660db24. Integration preserves the accepted product and test changes; its source-ceiling conflict resolves to the existing tool's measured 176318 lines after 0511. Branch evidence covers the affected behavior; no new full main qualification is claimed.

## What the build taught us

Provenance must control calibration warnings; an effective default threshold does not establish authored tuning. Additive facts require updating the owning byte oracle without relaxing old reply fields. Deriving request maxima from independent encoded-body literals preserves both stage accounting and exact caller output. Deadline wording describes the configured budget, while expiration remains absolute across composed calls.

This evidence covers Linux and local offline loopback fixtures. Other installed surfaces, remote machines, paid calls, release operations and landing remain outside this Quick Fix.
