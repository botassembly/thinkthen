# Package-gate defect preparation (from experiment 302)

Preparation record for the nine clean-checkout gate failures in issue `2026-09-29-nine-package-gates-fail-from-clean-checkouts.md`, verified at main `680b67ba`. Each bundle gives the ticket author the starts-from evidence and a proof sketch; nothing here is fixed or decided. Current main at filing: `c8d403e44190bf7b68513d78ca7cc0fcecb79f84` — confirm each still reproduces before opening its ticket.

## Cluster A — stale tests after the batch/packing contract change (one sweep likely)

**A1 libraries/c.** Starts from: `libraries/c/tests/door/main.rs:333-334` loops `for arrived in [1,2,6]` and `backend.wait(6)`; the engine now packs the five `tests/c/cancel.c:77` texts into one request, so arrivals cap at 3 and the wait always times out (`left: 3, right: 6`). Proof sketch: update the expected arrival schedule to the packed wire (derive from `specification/records.md` "Order and requests"); keep the held-reply cancellation semantics asserted; rerun the door test 3x deterministically. Evidence: experiment 302 `15-c/logs/{04,05,06}*.log`.

**A2 libraries/polars.** Three stale assertions: `27-decide-many` default batch drift; mid-column deadline can no longer fire (one request); 20 texts now 1 request not 20. Proof sketch: re-derive expected request counts from the packing spec per case; keep deadline semantics via a batch=1 variant if mid-column firing must stay proven. Evidence: `21-polars/REPORT.md` findings 1-3.

**A3 libraries/typescript.** One stale assertion; package otherwise self-consistent. Locate via `20-typescript/logs/check-run.log`; proof sketch is a single expectation update plus rerun.

## Cluster B — harness defects (gate-infra fixes, no product change)

**B1 libraries/objective-c stdlib shadow.** `checks/types.py` shadows stdlib `types`; gate passes only where ambient `PYTHONWARNINGS` preloads stdlib at interpreter startup (this dev host, by accident). Fix shape: rename the runner module (or move it out of the script-path directory) and make sibling imports explicit; proof: gate passes under `env -i PATH HOME` with no ambient preloading, 3x; also add the missing node/jsonschema README prerequisites. Evidence: `11-objective-c/logs/probe-privacy-*.log`, `importtime-*.txt`.

**B2 databases/sqlite counter filter.** `tests/test_settings.py:87` counts every file but `.thinkthen-backend.json`; engine writes `<folder>/.locks/` by design (`cache_lock.rs:132-137`), so the lock file counts as an entry. Fix shape: filter to `*.json` exactly like the five sibling surfaces (python/postgresql/duckdb/c/consumer paths in the report). Proof: case `record-sends-every-time` passes; `25-defect-fault` remains documented not-run. Evidence: `24-sqlite/REPORT.md` findings 1-3 with sibling citations.

## Cluster C — code defects (small, mechanical)

**C1 libraries/zig missing file.** `Tests/installed.py` requires a file its own build step does not produce. Fix shape: make the build emit it or drop the requirement; proof: clean-checkout `check.sh` reaches its installed stage and passes. Evidence: `01-zig/logs/{01,02,03}*`.

**C2 libraries/rust examples.** `examples/slide.rs:8` fmt failure; `tag.rs:13` and `decide.rs:15` print `{:?}` of the Call result instead of the pinned answer text. Fix shape: format; print the typed answer fields; update or restore pinned `.txt`. Evidence: `19-rust/REPORT.md` findings 1-3.

**C3 libraries/python annotate_frame signature.** Tests and binding disagree on `_annotate_frame` arguments at this pin; 7 pytest failures reproduce 3/3. Decide which side is contract (specification owner) before the fix; proof: 85/85 pytest plus the phases the early stop skipped (conformance, examples, wheel build, pandas lane). Evidence: `16-python/logs/03b-check.log` and findings.

## Cluster D — under its own lane

**D1 databases/duckdb.** 4 settings_suite + 3 conformance failures; the surface is mid-flight in the queue owner's current work — fold into that lane rather than a new ticket. Evidence: `22-duckdb/REPORT.md`.

## CI environment note (from PASS reports)

csharp's gate is conditional on ambient environment; objective-c's accidental dependency is the same class. When the workflow tickets (0269-0272) define runner environments, run every package gate under a minimal allow-listed env (`env -i` plus explicit vars) so host accidents stop masking failures.

## Suggested ticket split

Three tickets cover clusters A (one stale-test sweep), B (two harness fixes), and C1-C3 individually or as one small-defects ticket; D1 stays in its lane. Every bundle above maps to the five-part Evidence section the ticket template expects.
