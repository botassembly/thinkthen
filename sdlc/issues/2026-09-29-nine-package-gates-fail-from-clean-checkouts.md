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

The causes differ. The sealed receipts identify packing drift in C and Polars, an export-count mismatch in TypeScript, example output and formatting defects in Rust, and separate harness or build-input defects in other packages. Python has mixed failures and DuckDB needs further analysis. The [contract drift issue](2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md) explains relevant behavior changes; it does not establish one cause for six failures. The 15 PASS / 9 FAIL result above describes the sealed revision, not current main.

## Also worth fixing (from PASS reports)

csharp's gate depends on ambient environment; objective-c README lacks node/jsonschema prerequisites; several cosmetic findings — all listed per surface in the experiment.

## Current-main preparation, 2026-09-29

The [bounded reconciliation](../records/2026-09-29-clean-package-gates-preparation.md) compares the sealed `680b67ba` receipts with `origin/main` `ac19afc9d`. The five priority causes remain in current source; **no current full package gate was rerun**, so this is not a new nine-gate failure measurement or a closure. One clean-interpreter Objective-C startup check still fails at the first Python step. The C held test must keep a real in-flight request before each token fire: the proposed count of three follows the packed wire, but its request and cache assertions need focused execution. Zig's missing file is required by the installed project's default build and must be copied, not removed from that build. The TypeScript assertion and Rust example outputs still disagree with their current exports and pinned answers.

The 0276 child-environment cleanup, 0277 wrapper facts, privacy cleanup, version checks and API inventory gate changes landed after the sealed pin; none edits the five failing lines above. They are retained improvements, not proof that the original nine clauses are green. The Python frame/Series clauses, Polars, DuckDB and SQLite remain held; Python's independent settings count and preferred-host Pydantic compatibility are classified in the record without executing frame work. C#/JVM and C++/Dart package work belongs to their claimed lanes. Keep all nine original criteria separate until each affected gate or a narrower equivalent has passing evidence.

## Evidence

Local experiment 302 `experiments/302-polyglot-package-verification/`: one folder per surface with REPORT.md and logs; `FINDINGS.md` consolidates; `waves.log` is the run record. Inputs sealed at the pin.
