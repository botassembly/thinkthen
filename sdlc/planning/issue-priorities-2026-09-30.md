# Issue priorities, 2026-09-30

Status: current, rewritten 2026-09-30 night by the queue batches Quick Fix against main `d8018dd96`. It classifies every open issue and remaining ticket, then groups the ready work into lane batches. Git history holds the earlier ranked table. Ian can overturn any class, batch or default below.

Main today: 0304 complete (the old store and batching code are gone, `5b7ec74c1`), 0314 and 0291 landed (every port reads the Rust schema), 0322 complete, 0338 (test binaries 44 to 11), 0339 (ollama), 0341 (stale connection), 0342 (relate menu) and 0343 (rate setting). Running: lane claude-1 builds 0335 slice 2 (a replay smoke per binding, paying the R and Dart check debts); lane claude-2 builds 0344 (both-ways edges carry `"either":true`).

## Batches

Each batch is one lane's work in one area of files, so batches running at once do not collide. A batch of two tickets builds them in order in one lane. Each ticket lands as its own merge; the two tickets of a batch may land as one merge when both pass their checks together. Every batch also edits `CHANGELOG.md` and `sdlc/ratchet.json`; landers take those one at a time and rebase.

| Order | Batch | Work | Files it touches | Starts | Runs beside |
| ---: | --- | --- | --- | --- | --- |
| 1 | B1 demos | ticket 0350: record scripts write a clean fixture; a demo of a first cut before a list over 255 (Debt 024, docs page 11, stumble row 9) | `demos/*/record.sh`, one new `demos/NN-*/` folder, `demos/README.md`, three issue files | now | lanes claude-1 and claude-2, then B2 and B3 |
| 2 | B2 batch setting | ticket 0349: `audit` and `diff` read `meta.batch_setting` (Debt 008) | `crates/thinkthen/src/cli/{audit.rs,audit/,diff.rs,asking/row.rs}`, `public/results/member.rs`, `core/result/batch_warning.rs`, any conformance or type expectation that pins full `meta`, `specification/{result.md,result.schema.json}`, ADR 0085, `tests/backend/{audit_write,diff}.rs` | when 0344 lands (both edit the schema) | B1, B3, lane claude-1 |
| 3 | B3 reader and C door | ticket 0345: one capped question-file reader (Debt 011); then ticket 0346: C door rows from the crate's types and door tests in their own folders (Debts 012, 025) | `crates/thinkthen/src/public/{question.rs,relate.rs}` and one new reader module, `cli/question_text.rs`, `libraries/c/src/{door.rs,call.rs}`, `libraries/c/tests/door/`, `libraries/typescript/src/door.rs`, `libraries/ruby/src/ffi/question_file.rs`, `libraries/python/src/asked.rs`, `specification/question-file.md` | when 0344 lands (both edit `asked.rs`) | B1, B2, lane claude-1 |
| 4 | B4 SQL hosts | ticket 0347: the SQL hosts call the public API (Debt 018 items 1, 2, 3, 9, 10); then ticket 0348: SQL store proofs (Debt 010) | `databases/{sqlite,duckdb,postgresql}/` sources, tests and harnesses; `crates/thinkthen/src/public/{error.rs,engine.rs,question.rs}`; `core/relate_file.rs` | when 0344 and 0335 slice 2 land; lands after B3, which also edits `public/question.rs` | B5, B6 |
| 5 | B5 macOS archive | ticket 0351: the macOS static library and R package export no SQLite names (Debt 001), with an M5 proof | `libraries/c/localize.sh`, `libraries/r/check.sh`, `libraries/r/thinkthen/src/Makevars*`, the symbol gate in `libraries/c/tests/door/main.rs` | when 0335 slice 2 lands; its door-test edit rebases on 0346 | B4, B6 |
| 6 | B6 binding checks | two Quick Fixes, ready as written: `libraries/zig/check.sh` runs `run_matrix.py facts` and `facts-allocation`; the Flutter host test keeps only its facade call and drops `strict.dart` (issue text names the counts and fixture to move) | `libraries/zig/check.sh`, `libraries/dart/flutter/` | when 0335 slice 2 lands (it edits both check scripts) | B4, B5 |

Order follows the brief: 0.1 blockers first (B1 to B5), then debt that slows builders (B6: every Dart consumer change is made twice until the Flutter copy goes). B4 and B5 are also blockers, but they wait on lane claude-1. Four lanes run at most: now lanes 1 and 2 plus B1; after 0344, B1, B2 and B3 beside lane 1; after 0335 slice 2, B4, B5 and B6 as lanes free.

Work outside the lanes:

- **R1, the relate decision run.** One capped paid Beatles Bench run through `sdlc/scripts/live` (ruling 13) on the 0342 menu, after 0344 lands so the bench pins one commit. Conditions and bar: `sdlc/issues/closed/2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md`, "The decision run". Blocks 0.1.
- **E1, the loopback TLS count.** A local experiment, no network: `sdlc/issues/2026-09-26-count-secure-connections-at-sixteen-jobs.md`, "A loopback count first". Any lane or a `pi-job`; it becomes a ticket only if the count exceeds 16.
- **Marketing, owner of `site/`.** Blocks 0.1: the reference page (`2026-09-30-reference-page-exit-codes-and-key-rule-drift.md`); the providers and Liquid d1 pages, including the `--timeout 90` line for stumble row 19 (`2026-09-29-docs-page-naming-supported-providers.md`); the site recording conversion, the five `--dry-run` examples and the site checks (`2026-09-30-site-replay-folders-have-no-fixture.md`, which holds the conversion command); the overhead benchmark for the README line (`2026-09-29-readme-key-backend-and-overhead-lines.md`).
- **Ian.** The first release rehearsal ran on 2026-09-30 under his test approval and stopped at `resolve` on a bug of ours. It runs again after that fix lands. Then phase 4 and the registry accounts already on his list.

## Every open issue

35 open. Class: **batch** (a ready ticket or Quick Fix in a batch above), **running** (a lane owns it now), **outside** (a run, experiment or another owner), **waits** (a named trigger), **after 0.1**.

| Issue | Blocks 0.1 | Class | Owner or trigger |
| --- | --- | --- | --- |
| `2026-09-30-demo-record-scripts-write-beside-their-fixture.md` (Debt 024) | yes | batch | B1, 0350 |
| `2026-09-25-docs-how-tos-and-spec-claims-owed.md` | page 11 only | batch for page 11 | B1, 0350; pages 12 to 24 after 0.1; page 23 marketing |
| `2026-09-20-new-user-stumble-register.md` | rows 9, 18, 19 | batch for row 9 | B1 for row 9; ticket 0128 phase 4 for row 18; marketing for row 19 |
| `closed/2026-09-30-audit-and-diff-lost-the-batch-setting.md` (Debt 008) | yes | closed | B2, 0349 landed |
| `2026-09-30-question-file-reader-copies-and-uncapped-loaders.md` (Debt 011) | yes | batch | B3, 0345 |
| `2026-09-30-c-door-relate-rows-have-no-owner.md` (Debt 012) | yes | batch | B3, 0346 |
| `2026-09-30-c-door-tests-race-under-nextest.md` (Debt 025) | no | batch | B3, 0346 |
| `2026-09-25-public-library-api-gaps.md` (Debt 018) | items 1, 2, 3, 9, 10 | batch | B4, 0347; items 6 and 7 after 0.1 |
| `2026-09-30-sql-host-store-proofs-are-partial.md` (Debt 010) | yes | batch | B4, 0348 |
| `2026-09-30-static-library-exports-sqlite-symbols.md` (Debt 001) | yes | batch | B5, 0351 |
| `closed/2026-09-30-zig-check-skips-its-facts-lifetime-modes.md` (debt) | no | closed | B6 Quick Fix landed |
| `closed/2026-09-30-flutter-strict-consumer-copies-dart-bravo.md` (Debt 021) | yes, by its trigger | closed | B6 Quick Fix landed |
| `2026-09-30-dart-check-never-runs-under-the-surfaces-rung.md` (Debt 022) | yes | running | lane claude-1, 0335 slice 2 |
| `2026-09-30-r-check-rebuilds-every-dependency.md` (Debt 019) | no | running | lane claude-1, 0335 slice 2 |
| `2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md` | yes | running, then outside | lane claude-2, 0344, for the shape; then R1 |
| `2026-09-26-count-secure-connections-at-sixteen-jobs.md` | no | outside | E1 |
| `2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` | yes | outside | marketing |
| `2026-09-29-docs-page-naming-supported-providers.md` | the Liquid timeout line | outside | marketing |
| `2026-09-30-site-replay-folders-have-no-fixture.md` (Debt 007) | yes | outside | marketing; our conversion proof landed with 0304 slice 5 |
| `2026-09-29-readme-key-backend-and-overhead-lines.md` | yes | waits | marketing's overhead benchmark, then the queue owner writes one sentence |
| `closed/2026-09-30-release-resolve-writes-the-version-line-into-its-outputs.md` | yes | closed | Quick Fix landed; ticket 0128 phase 3b dispatches the rehearsal again |
| `2026-09-30-duckdb-macos-extension-may-export-sqlite-names.md` (Debt 026) | if the rehearsal shows a `sqlite3_` name | waits | the rehearsal's macOS DuckDB jobs |
| `2026-09-25-release-and-install-for-0-1.md` | it is 0.1 | waits | every blocker above; Ian's rehearsal dispatch and registry accounts |
| `2026-09-30-spec-no-calls-edges-need-a-real-send.md` | no | waits | the release QA suite's edge list |
| `2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md` (Debt 014) | no | waits | upstream, ollama/ollama#18718 |
| `2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md` (Debt 020) | no | waits | upstream ureq-proto; Ian's resend choice has a recorded default: keep the rule |
| `2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md` (Debt 002) | no | waits | upstream Zig |
| `2026-09-30-polars-door-cannot-test-lazy-streaming.md` (Debt 004) | no | waits | a user, or clean advisories |
| `2026-09-24-rank-by-graded-relevance-for-search-reranking.md` | no | waits | a user |
| `2026-09-26-every-surface-should-give-back-run-facts.md` | no | after 0.1 | tickets 0300 and 0302 stay open for it |
| `2026-09-25-recognize-and-relate-scale-and-shape.md` | no | after 0.1 | a relate ticket |
| `2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md` | no | after 0.1 | coordinator default 3 below |
| `2026-09-23-annotate-options-from-a-file-or-a-record.md` | no | after 0.1 | a ticket |
| `2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md` | no | after 0.1 | tickets per part |
| `2026-09-30-batch-command-runs-many-questions-in-one-process.md` (idea) | no | after 0.1 | stays as an idea |
| `2026-09-30-opentelemetry-traces-after-0-1.md` (idea) | no | after 0.1 | stays as an idea |
| `2026-09-30-proxy-service-for-shared-limits-and-traces.md` (idea) | no | after 0.1 | stays as an idea |

Closed by this Quick Fix: `closed/2026-09-30-live-batching-flake-and-unexplained-usage-calls.md`, paid by `87602e2ad` (the unexplained calls) and `f0308dd9d` (ticket 0341, the one cause found for a lone exit 4).

## Remaining tickets

- **0335** slice 2 and **0344**: running in lanes claude-1 and claude-2.
- **0345 to 0351**: drafted here, ready, in batches B1 to B5.
- **0334**: landed. Its deferred ADR 0114 build slice 2, a `backend` setting in each binding and SQL extension, waits until after 0.1: engines built from the environment already honor `THINKTHEN_BACKEND` and the configuration file's `backend`.
- **0290**: withdrawn. Its E1 to E9 corpus would duplicate `conformance/`, and ADR 0111 changed the bodies it would pin. Its page items moved to page 24 of the docs issue.
- **0295** and **0296**: deferred until after 0.1. Python already judges Polars and pandas Series in one engine call; the expression namespace and the `.tt` accessor are additive.
- **0300** and **0302**: open for the other hosts, after 0.1, under the run facts issue.
- Older Codex tickets whose status lines stop short of landed (for example 0268 to 0273) hold release runner work that the release issue owns.

## Coordinator defaults

Taken under the workspace rule to record reviewed choices and proceed. Ian can overturn each one.

1. Both-ways relate edges get their shape before 0.1. Ticket 0344 chose a trailing `"either":true` member over a `pair` array, so directed edges keep their bytes and there is one edge shape.
2. The release rehearsal waits for 0351's M5 proof, so its macOS jobs confirm the archive.
3. Relation pairs keep the whole text for 0.1, with a distance limit as an opt-in later.
4. Homebrew stays a Mac option; the curl script covers Linux.
5. 0290 is withdrawn, and 0295 and 0296 wait until after 0.1 (above).
6. `Engine::usage` stays per engine, and its doc says so (0347). Detailed rows on every surface carry `meta.batch_setting` for ADR 0085's warning (0349).
