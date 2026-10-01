# System grading of 2026-09-30, and the work left before 0.1

Twenty reviewers each graded one difficulty area of ThinkThen at commit `58014776c`. Each report uses one template and one rubric, so the reports compare side by side. This page consolidates them. It was written in workspace experiment 2040. Every blocking item and every claim a report marked unconfirmed was checked again against main at `f367545a0` (2026-09-30, evening). Items main has fixed since the graded commit are marked with the fixing commit.

The area reports sit beside this page as `01-batching.md` to `20-record-replay.md`. They keep the text the reviewers wrote at the graded commit. Their line numbers refer to `58014776c`.

## The rubric in brief

Each area report cites this page as its rubric.

- **Complexity**, 1 to 5, says how hard the area is, not how well it is built. It is the rounded mean of seven measured drivers: code size, states and concurrency, rules and refusals, surfaces touched, settings read, contract weight, and churn and debt over the last seven days.
- **Quality**, A to F: does the code do what the specification and ADRs say, and does the contract text still match the code? A is no drift. B is minor drift or a divergence on a secondary path. C is one primary-path divergence, or text that misleads a user about what is sent, paid or printed. D is several. F means the area does not do its job.
- **Reliability**, A to F: behavior under failure, load and time. A needs a counting test for every failure path, no fix commit in seven days, and no open reliability issue. B allows two fix commits, and code rewritten in the last seven days caps here. C means test gaps, repeated regressions, load flakes, or wall-clock limits in routine tests. D is an open defect that changes results, spend or secrecy.
- **Maintainability**, A to F: how cheaply a new person can change the area safely. A is one owner per concern, no duplicates, files well under the 500-line cap, and no lint suppressions. B allows a few duplicated helpers or a file near the cap. C means the same logic in three or more places, unclear ownership, or files at the cap. D means changes span many surfaces with no shared owner.
- **Cleanup sizes**: S is one file or function, under about 100 changed lines. M is several files in one area, or a fixture change that bindings follow. L crosses areas or surfaces, or needs an ADR. An item **blocks 0.1** when it makes a published contract false, risks spend or secrecy, or fails a gate.

## The twenty areas

| No. | Area | Complexity | Quality | Reliability | Maintainability |
| ---: | --- | :---: | :---: | :---: | :---: |
| 1 | Batching | 4 | B | B | B |
| 2 | Question cache | 4 | B | C | B |
| 3 | Process-wide send limits | 4 | B | C | B |
| 4 | Cancellation and deadlines | 4 | B | C | C |
| 5 | Spend accounting, never sending twice | 4 | C | C | C |
| 6 | Key secrecy | 3 | B | B | B |
| 7 | C interface and its 13 languages | 4 | C | B | C |
| 8 | Same answer on all surfaces | 4 | C | C | C |
| 9 | SQL extensions | 5 | B | C | C |
| 10 | Packaging and release | 4 | D | C | C |
| 11 | Settings across tiers and surfaces | 4 | B | B | C |
| 12 | Test suite speed and reliability | 4 | B | C | C |
| 13 | Probability, thresholds, calibration | 4 | B | B | B |
| 14 | recognize | 4 | B | B | B |
| 15 | relate | 4 | C | B | B |
| 16 | Backend wire dialects | 3 | B | B | B |
| 17 | Named backends and addressing | 4 | C | B | B |
| 18 | The configuration file | 3 | B | B | B |
| 19 | Input framing | 3 | B | B | C |
| 20 | Record and replay | 3 | B | B | B |

Totals. Complexity: one area at 5, fourteen at 4, five at 3. Quality: 14 B, 5 C, 1 D. Reliability: 12 B, 8 C. Maintainability: 11 B, 9 C. No area earned an A on any dimension, and only packaging's quality fell below C.

## The overall picture

ThinkThen is hard almost everywhere. Seventeen of twenty areas touch every surface, and most carry over 25 rules. The code mostly does what its contract says: fourteen areas match on every primary path. No A appears, for three shared reasons. Most areas were rewritten or changed after landing in the last seven days, which caps reliability at B. Routine tests still hold wall-clock limits, which drops four areas (2, 3, 4 and 12) to C. Ticket 0352 replaced most of them with event waits after the graded commit (`9e33cd639`). The same rule is written in several places: a row's usage sum, the batch setting, the 255-entity cap, the framing dispatch, each surface's case matching, and five export checkers. The worst drift is contract text left behind by the ADR 0111 rewrite. It sits in `records.md`, `result.md`, `audit.md`, `relate.md` and `backends.md`, and it would mislead a user about requests, retries or cost. Packaging earned the only D because no rehearsal has finished a build. Its four known bugs landed at `f080fd50c`. The third rehearsal (run 36791693601) never started a job, because GitHub refused the account's billing. Several files sit at or near the 500-line cap, so the next rule added to them will fail a gate: `engine/http.rs` (496), `tests/library/public_env.rs` (499), and `libraries/c/tests/door/cases.rs` (768, where no check reaches it).

## The top ten cleanup items

Ranked by value against size, across the whole repo. Blocking items come first, then items that remove a whole class of failure cheaply. Ticket 0352 (wall-clock waits in routine tests, areas 2, 4, 12, 20) would rank high, but it landed at `9e33cd639`, so it is left out.

| Rank | Item | Areas | Size | Blocks 0.1 |
| ---: | --- | --- | :---: | :---: |
| 1 | Correct six contract sentences that drift from the code: request counts and entries in `records.md:106-139`, the retryable statuses in `result.md:157`, the model-refusal lines in `audit.md:202-204`, the re-bill on a grown entity set in `relate.md:54` and `:97`, keyless `ollama` in `backends.md:57`, and the surfaces in `CHANGELOG.md:7` | 1, 5, 10, 13, 15, 17 | S | yes |
| 2 | Give `cache prune --older-than` a parser that cannot panic on a multi-byte unit (`cli/cache.rs:146`) | 2 | S | yes |
| 3 | Run the 500-line cap over `libraries/` and `databases/`, split `libraries/c/tests/door/cases.rs`, and split the files within ten lines of the cap (`engine/http.rs`, `tests/library/public_env.rs`, `engine/usage/tests.rs`, `cli/args.rs`, `tests/public_controls/call_facts.rs`) | 1, 3, 5, 7, 11, 12 | M | no, but Debt 027 says pay before 0.1 |
| 4 | Sum a row's usage by the one rule of ADR 0111 section 7, in one helper beside `pipeline::Answered` (`engine/facade/each.rs:296-304` disagrees with the other askers) | 1, 2, 5 | S | no |
| 5 | Send `Connection: close` from every loopback fixture, or share one fixture backend, which removes the HTTP/1.0 reuse flake class (Debt 020) from ten ports and the SQL suites | 7, 8, 9 | M | no |
| 6 | Resolve the batch setting in one function instead of five copies of the tier order and its sentence | 11 | M | no |
| 7 | Add a static check that a derived `Debug` on a type holding text needs a named allowance | 6 | M | no |
| 8 | Replace the five copied `exports.py` checkers with `sdlc/scripts/check-c-exports.py` | 7 | S | no |
| 9 | Remove the `dead_code` suppressions whose reasons are stale or name landed tickets (`engine/mod.rs:26`, `:158`, `:178`, `Engine::usage`, `core/answer.rs:42`) | 4, 5, 13 | S | no |
| 10 | Count options in the annotate dry run (`cli/annotate/plan.rs:57`), then share one dry-run planner so `--plan` always equals the live packing | 1 | S, then M | no |

## Fixed on main since the graded commit

- Area 5 item 1, `Engine::usage()` promising process totals: 0347 changed the doc to "This engine's totals" (`8439ee2b7`, landed `efc6a6a99`).
- Area 10 item 1 and area 12 item 2, the four rehearsal bugs (uv on macOS, the apt note, the DuckDB bridge crates, five library tests that start the command): the fixes landed at `f080fd50c`. The rehearsal half of area 10 item 1 stays open as pre-0.1 item 9. The fix review filed Debt 028 for the gate gap.
- Area 9 item 3, SQL hosts copying engine code: 0347 (`efc6a6a99`). It did not block 0.1.
- Area 20 item 1, the site's fifteen replay folders, and the five `--dry-run` examples: marketing converted them in its site ticket (`ebe9bb7a2`). The site smoke passes 97 of 97. Parts 3 and 4 of the site issue remain.
- The reference page, in part: the same site ticket added exit 7, reserved only 8, and named `thinkthen.status/2` (`ebe9bb7a2`). Exits 130 and 143, the local and named-backend key rules, and three moved links remain.
- The wall-clock waits in routine tests (area 2 item 3, area 4 item 2, area 12 item 1, area 20 item 8): ticket 0352 landed at `9e33cd639`. Area 3's rate-spacing tests (`tests/backend/backoff.rs`, `named_backends/rate.rs`) were not changed, so area 3 item 1 stayed open until ticket 0356 measured them from the launch. The ticket filed Debt 029 (binding tests) and Debt 030 (the piped batching pause) for what remains.
- The relate decision run (R1) and docs page 11 were done before the graded commit.

Two queue-owner issues from marketing's site ticket are open and do not block 0.1. `sdlc/issues/2026-09-30-settings-table-row-and-recording-page-a-site-reader-hits.md` asks to rewrite two draft-form lines the site shows, in `settings.md:67` and `recording.md:30`. `sdlc/issues/2026-09-30-site-fixtures-converted-and-plan-examples-moved.md` is a note recording parts 1 and 2 of the site issue as done.

## Unconfirmed claims, rechecked

- Confirmed by reading main: area 2's `--older-than` panic (`str::split_at` at `text.len() - 1` falls inside a multi-byte last character); area 16's half-filled `usage` refusal (`ResponseUsage` requires both counts); area 3's re-closed gate spending a pacer slot (the loop calls `pace` again after dropping the permit, in `acquire_open` of `engine/http.rs`); area 5's live wrapper passing no cap to the job (`sdlc/scripts/live` sets only the key and `PATH`); area 7's find gap (`libraries/c/src/call.rs:48` allows `details` on four verbs only, and the `find` arm writes the selected unit alone).
- Still unconfirmed, and needing a run: area 11's huge library timeout (whether the HTTP client panics), area 19's lone CR in a table and `relate` reading its whole input, area 5's poisoned-queue skip, area 6's echoed `key_env`, and area 9's macOS DuckDB export names (the rehearsal decides it).

## Brief corrections

The reviewers' starting briefs mislabeled four paths. `cli/file_size.rs` installs the `SIGXFSZ` policy and is not the `@FILE` cap. `cli/normalize.rs` joins a negative `--threshold` before Clap and is not framing. ADR 0104 is annotate's record-failure policy and not recording. `engine/roots.rs` reads the private TLS root bundle and is not folder trust. Areas 18, 19 and 20 name the correct owners.

## The pre-0.1 list

This merges the reports' blocking items, the open issues that `sdlc/planning/issue-priorities-2026-09-30.md` marks as blocking 0.1, the release rehearsal work still open in ticket 0128 phase 3b, and the two release-QA bugs, with duplicates joined. Items 7, 8 and 20 are here because their debt issues say pay before 0.1, although the priorities table marks them not blocking. "Lane" says whether a lane is on the item now.

| No. | Item | Source | Owner | Size | Lane |
| ---: | --- | --- | --- | :---: | --- |
| 1 | Correct six contract sentences that drift from the code (top-ten item 1) | areas 1.1, 5.2, 10.5, 13.1, 15.1, 17.1; `sdlc/issues/2026-09-30-contract-sentences-that-drift-from-the-code.md` | queue owner | S | no |
| 2 | `cache prune --older-than 5é` panics instead of printing the usage sentence | area 2.1; `sdlc/issues/2026-09-30-cache-prune-older-than-panics-on-a-multi-byte-unit.md` | queue owner | S | no |
| 3 | Show whether a huge library `timeout` can overflow the client's clock, then cap it or pin it | area 11.3 (blocks only if it panics); `sdlc/issues/2026-09-30-library-timeout-has-no-upper-bound.md` | queue owner | S | no |
| 4 | `find` returns no probability through the C door, so the 13 C-interface languages cannot return it, and no runner proves it | areas 7.1, 8.1; release QA; `sdlc/issues/2026-09-30-find-returns-no-probability-in-c-interface-languages.md` | queue owner | M | no |
| 5 | A JSON record nested 128 deep is refused as "not valid JSON"; name the depth limit and give it its own sentence on every surface | release QA; `sdlc/issues/2026-09-30-deep-json-record-refused-as-not-valid-json.md` | queue owner | M | no |
| 6 | Count stored answers in the DuckDB and PostgreSQL runners, and add SQLite's read-only replay and busy-store tests (Debt 010) | areas 2.6, 9.1; priorities; `sdlc/issues/2026-09-30-sql-host-store-proofs-are-partial.md` | queue owner | M | yes, claude-2 (0348) |
| 7 | Run the 500-line cap over `libraries/` and `databases/`, and split `libraries/c/tests/door/cases.rs` (768 lines) (Debt 027) | areas 7.2, 12.4; `sdlc/issues/2026-09-30-c-door-cases-test-over-the-file-cap.md` | queue owner | M | no |
| 8 | Run the library-only tests in a routine gate (Debt 028) | the rehearsal fix review; `sdlc/issues/2026-09-30-no-routine-gate-runs-the-library-only-tests.md` | queue owner | S | no |
| 9 | Rerun the release rehearsal once billing works: every job passes on four targets with zero "not run", carrying the package proofs of the release issue's items 6 and 7. The third attempt (run 36791693601) started no job | area 10.1; ticket 0128 phase 3b; `sdlc/issues/2026-09-25-release-and-install-for-0-1.md` item 1 | queue owner | M | no, blocked on item 19 (lane claude-3 was freed at `d3fd19419`) |
| 10 | Inspect the macOS DuckDB extension for `sqlite3_` names, and hide them if present (Debt 026) | area 9.2; priorities; `sdlc/issues/2026-09-30-duckdb-macos-extension-may-export-sqlite-names.md` | queue owner | M | no, waits on item 9 |
| 11 | Add publish jobs for NuGet, Packagist, Maven Central, pub.dev and R-universe, or state which languages ship as GitHub assets or tags only | area 10.4; release issue item 5 | queue owner | L | no |
| 12 | Finish the reference page: exits 130 and 143, the local-server and named-backend key rules, and three links to moved issues | areas 4.1, 6.1, 17.2; priorities; `sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` | marketing | S | no |
| 13 | Run every library and database sample on the site, and add the status-word check, the pandas page and the stray code-tag check (site issue parts 3 and 4) | area 20.1; priorities; `sdlc/issues/2026-09-30-site-replay-folders-have-no-fixture.md` | marketing | M | no |
| 14 | Write the providers page and the Liquid d1 page, with `--timeout 90` on a first run (stumble row 19) | priorities; `sdlc/issues/2026-09-29-docs-page-naming-supported-providers.md`; `sdlc/issues/2026-09-20-new-user-stumble-register.md` | marketing | M | no |
| 15 | Run the overhead benchmark, then the queue owner writes the README's overhead sentence | priorities; `sdlc/issues/2026-09-29-readme-key-backend-and-overhead-lines.md` | marketing | M | no |
| 16 | Point the site's R install line at R-universe | release issue item 4 | marketing | S | no |
| 17 | Set up the registry accounts and trusted publishing (NuGet, Packagist, Maven Central, pub.dev, R-universe, crates.io, PyPI, npm, RubyGems), the `release` environment, and tag rulesets, which need GitHub Pro or a public repository | release issue items 2 and 5; ticket 0128 | Ian | M | no |
| 18 | Phase 4, the release run: 0.1.0 everywhere, the README install commit (closes stumble row 18), publishing, the tap, the history reset, and the public install checks | ticket 0128 phase 4; release issue item 2 | Ian | L | no |
| 19 | Fix the GitHub account billing, which stopped the third rehearsal before any job started | ticket 0128 phase 3b, third attempt | Ian | S | no |
| 20 | Replace the timed stops and short waits left in the binding tests (Debt 029) | ticket 0352; `sdlc/issues/closed/2026-09-30-binding-tests-still-time-stops-and-wait-on-short-bounds.md` | queue owner | M | done, ticket 0356 |

Counts: 20 items. By owner: queue owner 12, marketing 5, Ian 3. By size: S 7, M 11, L 2. A lane is on 1 of them now (item 6). Item 3 blocks only if the run shows a panic, and item 10 only if the rehearsal shows a name.
