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

## Focused Quick Fix candidate

The [Quick Fix build record](../records/2026-09-29-clean-package-gates-quick-fix-build.md) names separate current-source receipts for the five assigned clauses. C's held scalar/JSON/bulk cancellation witness now reaches three real held requests and returns without a new send; TypeScript's selected shapes suite passes with twenty matching export names; Rust's existing example-output test passes pinned bytes; Objective-C's privacy and J1 runners start normally under a minimal environment; Zig's direct installed selector runs four shared/static consumers with 42 exact bodies each. These are **focused candidate fixes**, not a rerun or closure of all nine package gates. The Zig build also found `Tests/types.py` shadowing stdlib `types` under a clean interpreter; its same-mechanism rename was added to this claim and has a separate direct J1 proof. The remaining Python, Polars, DuckDB and SQLite clauses stay open under their respective holds and qualification work.

## Evidence

Local experiment 302 `experiments/302-polyglot-package-verification/`: one folder per surface with REPORT.md and logs; `FINDINGS.md` consolidates; `waves.log` is the run record. Inputs sealed at the pin.

## Final full-24 re-verification (2026-09-29, pin 4c0ef210)

After the five fixes merged at e90de34a7 and the call-facts tickets 0275-0281 landed: **14 PASS, 10 FAIL**, all gates required to pass under minimal `env -i` environments this time. Evidence: local experiment 302 `finalcheck-4c0ef210/` (24 reports) and `FINAL-RUN-SUMMARY.md`.

- **The five fixes hold green** (zig, objective-c, c, rust, typescript).
- **New finding — JVM locale masking**: java, kotlin, and scala gates fail in a clean environment because the JVM decodes non-ASCII command-line arguments as ASCII without LANG/LC_ALL; round one passed by ambient-locale accident. One-line fix (export LC_ALL=C.UTF-8 in libraries/jvm/check.sh or move non-ASCII off the CLI); belongs in the environment-hardening sweep.
- **New regressions between pins, likely from the call-facts wave**: dart and flutter gates fail on one stale planted-negative expected-message (`wrong-probability`, PLANTED_NEGATIVE_WRONG_REASON, in checks/plant-check.py and flutter/plant-check.py); R fails shared conformance case 40-decide-counters deterministically ("the engine is already set with batch = Max; start a new R session to change it" — session-pinned settings interaction).
- **Unchanged opens**: python (contract ruling pending), polars (stale batch tests, held lane), duckdb (held lane), sqlite (same record-sends-every-time counter bug as originally filed — the *.json filter fix has not landed).
- **Typescript note stands**: export check still asserts a literal count; derive it from the binding source.


## Current routing after the final report

At current main `f1802619d`, the five full-gate passes above remain independently measured at `4c0ef210`. The later JVM locale correction is already landed at `5f53b3a58`; its [focused receipt](../records/qf-jvm-j1-offset.md) proves case41 through Java, Kotlin and Scala under an explicit UTF-8 child locale. It does not change the historical 14/10 table.

Fresh Medium code review accepted `ac5e03d75` for the [Dart/Flutter and R harness correction](../records/qf-package-counter-and-negative-diagnostics.md). R must keep its current session-settings contract: case40's counter measurement belongs in a fresh child because the ordinary answer checks already warmed the first child's cache. Dart's consumers should print the asserted outcome and probability, preserving the exact planted-negative marker. Do not weaken the marker to accept an object identity string. The corrected existing cases pass; these three reported failure causes are fixed. A fresh whole-package result remains separate.

SQLite0284 includes the `*.json` counter filter; DuckDB0286 retains all seven reported cases. Polars0289 and Python0287 retain their package criteria. The accepted SQL/frame ADRs ended the earlier design hold; the historical phrase “contract ruling pending” is no longer their scheduling state. These are active or dependency-ordered builds, not externally blocked items.

The detailed receipts also name unresolved qualification concerns outside the six corrected harness paths: Flutter's wrapper `pub get --offline` rewrote its locked `vector_math` when that version was absent; a Flutter check child lost the Cargo-wrapper override; Ruby fixture reads need an explicit UTF-8 environment; and Ruby's local `file://` dependency plant conflicts with a globally forced Cargo offline setting. Carry these to the package environment/release batch, refresh current source before changing it, and require locked dependency resolution. A missing cached dependency is an environment prerequisite, never permission to validate a silently changed lock. No new full-gate count is asserted here.

The TypeScript literal export count is not a remaining defect by itself. Its test also compares the three public export name sets. Keep an independent expected surface; deriving the expected count from the same actual output could conceal a missing export.

## Follow-up from the completed verification program

The [process and maintenance record](../records/2026-09-29-polyglot-verification-lessons.md) separates adopted standards from remaining implementation. The package-environment batch owns minimal parent and child environments for the release families introduced by 0269–0272. Prove UTF-8 arguments, explicit compiler overrides, unchanged dependency locks and independence from ambient `PYTHONWARNINGS`; preserve each gate's exact intended failure checks. These clauses stay open until the changed runners have receipts.

The same batch owns the repeated recording-entry counters. Share a helper among Python runners under `conformance/` where their semantics match. Specify one independent fixture containing actual entries, the backend marker, locks and temporary files; only actual entries count. R and Ruby may retain small native implementations against that fixture rather than acquire a Python dependency. Coordinate this change after the active SQLite/DuckDB claims clear. SQLite's immediate missing JSON filter remains part of 0284; passing that case alone does not prove consolidation.

R's per-call `batch` and other uniform keywords already belong to accepted preparation ticket 0288. Add a same-session proof that two calls use different call-level settings and produce their expected bodies without restarting R. Preserve the separate engine-configuration lifetime rules. The fresh-child correction at61350a253 fixed the existing counter test; it did not implement per-call settings. No duplicate R design ticket is needed.


## SQLite counter correction in 0284

Fresh High review accepted `a843a3fc52812e2062d9aa90170764725bc8a246`. The SQLite recording counter selects answer `*.json` files and excludes the backend marker; a fixture places a `.locks/one.lock` beside one answer and requires one entry. The installed selector runs the same witness, and the build record retains selected source and source-matched unpacked-archive receipts. This closes the original SQLite counter clause, not the other surfaces or a full package/release-runner gate. See [0284 build](../records/0284-sqlite-settings-and-keyed-many-build.md).

## Python and R preparation refresh

The [source refresh](../records/2026-09-29-python-r-dispatch-preparation.md) preserves every cause in experiment302's detailed Python report, including schema, input conversion, batch assumptions and interpreter compatibility. The earlier C3 summary named only the annotate signature. No additional cause is closed by preparation. R already has per-call batch/context and a same-session facts witness;0288 completes the uniform interface without treating engine replacement as a missing call option. The Python large-allocation RSS campaign must move to the explicit stress route when that host lane opens.
