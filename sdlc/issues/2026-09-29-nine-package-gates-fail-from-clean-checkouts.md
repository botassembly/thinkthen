# Nine package gates fail from clean checkouts

Status: open. Independent verification of all 24 integrated package surfaces at main `680b67ba` (2026-09-29, local experiment 302) found nine packages whose own gates fail deterministically from a clean copy. Fifteen pass, including ten of the eleven merged language packages. Every failure was reproduced from receipts before filing; no fixes were applied by the verifier.

## Failures, with causes

| Package | Failing stage | Cause | Class |
| --- | --- | --- | --- |
| libraries/zig | installed-package test | `Tests/installed.py` omits a file its own build script requires | code defect |
| libraries/objective-c | first Python step | `checks/types.py` shadows stdlib `types`; gate only passes where ambient `PYTHONWARNINGS` preloads stdlib at startup; no CI runs it | harness defect |
| libraries/c | cargo test (held-reply cancellation) | `tests/door/main.rs:333-334` pins a 6-arrival count record packing made unreachable (max 3) | stale test (packing) |
| libraries/python | pytest (7/85) | tests and binding disagree on `_annotate_frame` arguments; five more reproducible failures | code/test disagreement |
| libraries/rust | fmt + example tests | `examples/slide.rs` fmt failure; `tag.rs`/`decide.rs` print `{:?` debug dumps instead of pinned answers | code defects |
| libraries/typescript | package suite | one stale assertion | stale test |
| libraries/polars | three test binaries | batch/packing drift: `27-decide-many`, mid-column deadline, 20-texts-1-request | stale tests (batching) |
| databases/duckdb | settings_suite + conformance | 4 + 3 cases fail after all builds/lints pass; surface is mid-flight in its own lane | under analysis |
| databases/sqlite | tests/test_settings.py | counter omits the `*.json` filter every sibling has; engine's `.locks` file miscounted; engine correct | harness defect |

Six of nine trace to the documented contract changes in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md` not reaching package test harnesses. The product code mostly matches the new contracts; the gates do not.

## Also worth fixing (from PASS reports)

csharp's gate depends on ambient environment; objective-c README lacks node/jsonschema prerequisites; several cosmetic findings — all listed per surface in the experiment.

## Evidence

Local experiment 302 `experiments/302-polyglot-package-verification/`: one folder per surface with REPORT.md and logs; `FINDINGS.md` consolidates; `waves.log` is the run record. Inputs sealed at the pin.
