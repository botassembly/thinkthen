# What 0304 slices 4 and 5 must move before the old store goes

Status: open. Found by reading main at `ec130d3ee`. Owner: ticket 0304 slice 5, which deletes these files; slice 4 takes items 1 and 2 if it removes `ask_chunks` and moves `recognize`. Slice 4 took neither, so both stay with slice 5. Source: `sdlc/planning/after-slice-3-prep.md`, section 1 (slice 4 items 3 and 4, slice 5 items 1, 2 and 4) and section 4 items 7 and 8. Merged on 2026-09-30 with the `cache prune` and `status` issue, the recording page issue and the SQL host store proof issue, now in `closed/`.

Kind: debt

Pay when: 0304 slice 5 lands, before 0.1.

Keeping it leaves two batching paths, so a fix to one can miss the other. Until slice 5, `cache prune` cannot shrink `thinkthen.sqlite`, so the default cache grows past its size target.

## Callers of the files ADR 0111 deletes

ADR 0111 step 4 removes `ask_chunks`, and step 5 deletes `engine/schedule.rs`, `core/batch.rs` and the old recorder. Neither step names a new home for these callers:

1. **`thinkthen check`.** It sends its probe through `engine.split` and `engine.ask_chunks` (`cli/check.rs:66,116`). `check` must not read or write the cache (`specification/check.md:19`), so it needs a send-only call on the new send stage. The conformance runner and the facade tests also call `ask_chunks` (`cli/conformance_tests/runner.rs:50`, `engine/facade_tests.rs:136`).
2. **The command's many-line `recognize`.** It runs each line through `schedule::over_records` (`cli/recognize.rs:131,157`), which reaches `Engine::records` and `engine/schedule.rs`. An `Asker` returns one input's questions at once, so it cannot hold recognize's three dependent steps. The prep note's options: run each step across all lines; move a small ordered runner into `cli`; or compose steps per line on one thread. It recommends the `cli` runner, which keeps today's streaming and width.
3. **`engine/schedule.rs`.** `engine/pipeline.rs:17` and `engine/pipeline/run.rs:15` import its `Input`, and `engine/facade.rs:33` re-exports its types to `cli/schedule.rs`, which survives.
4. **`core/batch.rs`.** `quoted_plan` and `quoted_plan_of` serve `cli/asking/judged.rs`, `cli/annotate.rs:357`, `public/asking.rs:80`, `public/bulk/annotation.rs:74` and `cli/conformance_tests.rs`. The file also defines `Setting`, `BatchError` and `BatchRecord`, which the prep note counts in about 26 places outside tests.
5. **The old recorder.** `engine/store.rs:139,228` call `recorder::require_private`, the duplicate folder check that slice 2 deferred.

Each piece moves before its file goes. The slice 4 and slice 5 tickets name where each one lands, and their proof lists run `check`, the conformance runner and many-line `recognize`.

## `cache prune`, `cache unused` and `status` miss the question store

Since 0304 slice 2 (`1fbbe08e0`), the seven record functions keep their answers in `thinkthen.sqlite` (`engine/store.rs:28`). These commands still read only the old digest-named files:

- `cli/cache.rs` calls `engine/cache_prune` for `unused`, `prune` and `prune --dry-run`.
- `engine/cache_prune/scan.rs:84-103` (`scan_final`) skips every name that is not a digest file, so it never sees `thinkthen.sqlite`. It filters names before it opens a file, so no run was needed.
- `cli/status.rs:158-205` (`cache_status`) counts entries and bytes through `cache_prune::inspect`. `status --json` is already `thinkthen.status/2` since ticket 0334, with no question store fields.

So `cache prune` cannot shrink the question store, `--answered-by-other-than` cannot remove an old model's answers from it, and `status` does not count it. The model-mismatch message tells users to run that prune (`specification/result.md:248`). `find`, `recognize` and `relate` still write old entries until 0304 slice 4, so prune still trims those.

Slice 5 does what ADR 0111 says (`adr/0111…:232`): `cache prune` and `cache unused` keep their selectors as SQL over the store's `taken_at`, `answered_by` and keys, delete states no answer uses, then run `PRAGMA incremental_vacuum`. `status` counts the store. ADR 0114 says whichever of slice 5 and ticket 0334 lands second adds its fields to `status/2`; 0334 has landed, so slice 5 adds its cache fields there.

## The recording page describes the retired backend marker

Found by the release QA suite against main `c22512868` (edge E-088, part B of its first edge-specs ticket); the text still stands at `1cad2821a`.

ADR 0111, section 3, removed the backend marker and the folder gate: "This removes per-digest lock files, the folder gate, the backend marker and its admission probe." The address sits inside every cache key, so a folder pointed at a new address misses and resends. `specification/recording.md` still describes the old rules:

- Line 50: the first write-capable use of a folder binds it to the backend address, and a later mismatch exits 5 naming the resolved endpoint.
- Lines 51 to 54: the marker's schema, its write order and the unmarked-folder rules.
- Lines 32 to 34 and 115 to 117: the prune target and selection over old entries only.
- Line 119: "Prune ignores and preserves `.thinkthen-backend.json`."

The build follows the ADR. It writes no marker, and a hand-written marker naming another address does not stop `--cache` from sending. But `thinkthen status --json` still reports `cache.binding` as `unbound`, `matching` or `mismatched` (`cli/status.rs:45,191,273`), a field the specification does not list and the ADR retired.

The recording page states the ADR 0111 rule: answers from two addresses never mix because the address is in every key, and a cache at a new address misses and resends. The marker paragraphs and the prune sentence go. `status` drops `cache.binding`, or the specification names it and says what it means now. The size and splitting sections of `specification/backends.md` and the model-mismatch sentence follow.

## The SQL hosts' question store proofs are partial

Deferred by ticket 0304 slice 3b. Slice 3e closed the PostgreSQL missing-part row with `databases/postgresql/check.sh` `an_annotate_row_missing_its_part_fails_alone`. Keeping the rest risks a store regression in DuckDB or PostgreSQL that no check counts.

- The `databases/duckdb` and `databases/postgresql` shared case runners do not count answer rows in `thinkthen.sqlite`, as the SQLite (`databases/sqlite/tests/conformance.py`) and C door (`libraries/c/tests/door/cases.rs` `stored`) runners do. DuckDB's harness deletes each child's folder when the child ends, so the count needs a harness change.
- The prep note `sdlc/planning/0304-slices-3b-3d-prep.md` asked for the store's edge rows through the SQLite extension on the 3.50.0 host. The shared cases cover a hit and a miss. No test covers a read-only replay folder or a busy wait there.

Done when each gap has a test that fails when the store stops answering. Slice 5's surface sweep reruns these runners, so it takes this part; otherwise its own ticket pays it before 0.1.
