# 0068: Cached result metadata

Status: Independently accepted; final integrated local gates passed; landing pending. Hosted run `35756403716` for `8912884` was cancelled after Ian disabled GitHub Actions. Ian explicitly authorized local full gates as the verification authority; no hosted success is claimed.

## Result and handoff

Detailed result writers emit only `meta.cached`, in the old `replayed` position after `requests_sent`. The two serialized Rust fields carry the new name directly, initialized from unchanged private request metadata. Stored-only results include explicit replay and cache hits; mixed annotation remains false. Bare answers, requests, counters, recording bytes/digests, and `--replay` do not change.

Cost, trials, and the find-probe read boundary accept old and new rows. A present canonical key wins even when false or invalid; existing reader validation remains authoritative. Aggregate report names remain unchanged. Marketing must refresh detailed-output captures. The library handoff records the canonical field for the real-engine swap; no active surface branch changed.

## Review and verification

- Independent Sol design review accepted ADR 0036 and the bounded level-2 SWE-2 implementation route. The approved handoff already requested the new spelling.
- SWE-2 observed serializer and cost/trials/probe compatibility failures before the corresponding changes. Focused checks passed afterward.
- Coordinator review rejected an attempted change to the preregistered probe helper and checksum. Re-reviewed design moved adaptation to a transient input copy before the frozen reader. `fixture.py`, `cases.json`, and `hashes.sha256` are byte-identical to baseline `7a0ccd4`; the original checksum check passes. Captured probe rows, recordings, and measurements are unchanged.
- A fresh Sol code review required annotation-specific proof of exactly one canonical field and no legacy alias. The implementer added it and the same reviewer accepted it.
- The first coordinator lint run refused the 501-line result module. Direct output-field names removed the two extra Serde attributes, restoring 499 nonblank lines without raising the 500-line limit. Re-review accepted this equivalent representation. The exact total ceiling is 33,840, one line above baseline after the strengthened annotation assertion; existing helpers/tests were reused.
- Final coordinator command: `sdlc/scripts/install && sdlc/scripts/lint && sdlc/scripts/test && sdlc/scripts/spec && git diff --check`, with provider key empty and Cargo offline mode, run sequentially. Exit 0: policy, package checks, audit, format, clippy, docs, 568 Rust tests, one intentional child-harness ignore, doctests, cost/trials/probe checks, 27 specification checks, seven transform-page checks, replay comparisons, and nineteen green how-tos. Frozen-file diff and checksum checks also passed. After integrating main `692ba59` and preserving its website/product updates, the coordinator proved all program, test, gate-script, probe, transform, demo, specification, and workflow files still matched the reviewed `8912884` tree and reran the full ladder on the combined tree. It again passed 568 Rust tests, the single intentional child-harness ignore, all replay/transform/specification checks, and nineteen how-tos. No workflow setting or workflow file was changed.

## Evidence limitation

Two earlier worker backend-suite invocations overlapped in the same target directory and each reported 271 passed, one failed. Only result tails were retained; the failed test names and exact causes could not be recovered from worker logs or parent shell lookup. Shared fixed-name test folders make overlapping executions unsafe, but that does not establish the cause of these two failures. The coordinator then ran the backend suite alone: all 272 passed with complete captured output. The final sequential full ladder also passed all 272 backend tests. These observations do not label the earlier failures harmless or waive any gate.

No provider call, publication, dependency, security policy, recorded measurement, site, library, or database change was made. The next proposed private control is fast failure for typed refused connections under ADR 0017; width, cancellation/deadlines, fork recovery, and host signal ownership remain unfinished.
