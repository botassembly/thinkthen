# Record: ticket 0110, port the DuckDB surface

Branch `ticket/0110-port-duckdb-surface`. Built on Beelink on 2026-09-25 with the spike 253 toolchains under `~/.cache/thinkthen-toolchains/duckdb/v1.5.5/`: the stock CLI `v1.5.5 (Variegata) d8cdaa33fd` at sha256 `3d33b1df…5788`, and a venv holding `duckdb==1.5.5` under CPython 3.13.5. Nothing was downloaded.

## What landed

- `databases/duckdb`, its own Cargo workspace, on `thinkthen`'s public API and `libduckdb-sys =1.10505.0` with no wrapper crate. `src/` holds 9 files. Every `unsafe` line sits in `src/ffi.rs` or `src/signal/ffi.rs`.
- `check.sh`, which builds the shipped and `test-hooks` extensions, runs the source checks, deny, and its plant, loads the extension in the stock CLI, and runs the suites.
- `tools/`: `harness.py`, `verbs_suite.py`, `settings_suite.py`, `signal_suite.py`, `conformance.py`, `site_examples.py`, `source_checks.py`, `selftests.sh`, `setup.sh`, `version.env`, and `requirements.txt`. That is 11 files, under the ticket's 16.
- `deny.toml`, the root copy plus one exception for `zlib-rs`.
- `README.md`, a new `NOTES.md`, the first part of the ADR 0038 amendment, the planning page's wrapper paragraph, the site-example issue, and the query-hook issue moved from the tag with an update.
- `sdlc/surfaces.txt` moves `databases/duckdb` to landed.

## Deviations from the ticket

- `src/ffi/` became `src/ffi.rs` and `src/signal/ffi.rs`. `policy.py` allows `unsafe` only in a file named `ffi.rs`, and the ticket may not change the ladder scripts. R4-16's types rule reads the same way: every logical type is made and destroyed inside the `Logical` wrapper in `src/ffi.rs`, whose `Drop` destroys it.
- The binding uses `libduckdb-sys` alone. `duckdb`'s `VScalar` registers no init callback, and decision 10 needs one on every scalar. `foldhash` and `tiny-keccak` left the tree with `arrow` and `hashlink`. `cargo deny` with the root config now refuses only `zlib-rs` 0.6.8 (Zlib), through `flate2` and `zip` in `libduckdb-sys`'s build dependencies. The deny plant removes the `zlib-rs` exception, and `check.sh` pins deny's rejection line for `zlib-rs`.
- The conformance runner reads find cases, case `18-annotate-two-groups`, and relate cases as "not run" from one closed list. SQL finds with `ORDER BY` and `LIMIT` over decide. Main's public API refuses `on` in a library question set. Relate lands in 0118. The recognize cases run through `thinkthen_relations` with each case's own question file and compare the relations. Entity offsets of a described kind are not readable from SQL, because `thinkthen_recognize` takes kind names only. `verbs_suite.py` checks case 41's offsets on the generic arm.
- R3-13's `SA_SIGINFO` half is not proved: the Python host installs a one-argument handler. The chain, the ignored action, and the default action are proved.

## Results

- `check.sh` passes at the branch head: fmt, clippy with `-D warnings`, 4 unit tests, both builds, the source checks, deny and its plant, the stock CLI call, and 31 suite cases (verbs 12, settings 10, signal 9). The site-example check runs 8 drawn blocks and names 2 closed divergences. The selftests pass.
- Ratchets: `src` holds 2202 non-blank Rust lines and `tools` holds 1115 non-blank Python lines. Each ceiling equals its total. The request total of decision 17 added 88 Rust lines in `src/engines.rs`, since main has no per-call request limit to reuse.
- Conformance: 49 pass, 0 fail, 5 not run, 54 cases.
- Access cases: all 35 cache cases agree with DuckDB's `COPY … TO`, and all 35 `@file` cases agree with `read_text`. Each refused cache case sent nothing and created nothing in the case folders. The check reads folder listings, not `strace`.
- SIGINT: a held batch of 64 texts at throttle 8 and a single held `thinkthen_details` each read `cancelled` within 100 ms, and the count stayed put after release. 50 stop-then-answer rounds, 50 signals between queries, and 20 chained signals all pass.
- The volatile test uses `EXPLAIN`, with a count of 0.
- Request total (decision 17, Ian's ruling of 2026-09-25): a total of 3 and a decide over 10 distinct rows sent exactly 3 requests on the generic arm, and the next call read the pinned sentence with the count still 3.

## Planted bugs

Each plant changed one line, rebuilt the extension, and ran its one case. All eight went red and the source was restored.

| Plant | Case | Read |
| --- | --- | --- |
| The settings check skipped when an engine is already kept | `cache_cap_is_checked_on_a_map_hit` | expected an error, got `true` |
| The throttle cast wraps with `as u8` | `throttle_range` | expected an error, got `true` |
| Every refused probe open allowed | `access_cases_match_duckdb` | COPY refuses, we allow |
| No cache folder shape check | `cache_folder_shape` | expected an error, got `true` |
| Scalars registered without `volatile` | `volatile_scalars_are_not_folded_at_plan_time` | wanted 0 sends while planning, got 1 |
| Answers read by first place in the group | `r1_1_answers_map_back_by_text` | the rows came back wrong |
| The stop latches on any past signal | `r1_21_the_next_query_answers_after_a_stop` | the next query read `cancelled` |
| The total checked only when an engine is built | `the_process_request_total_holds_across_calls` | all 10 rows answered |

## Not yet done

- Plants for the signal handler chain, the panic guard, and the secrecy case.
- R5-21's 10,000-signal stress run, R2-18's 20,000-row open count under `strace`, R1-15's `strace` open count, the fork test, and the Python ratchet's measured ceiling review.
- `examples.json` and its runner. The site-example check covers the drawn blocks instead.

## For the landing agent

- The `surfaces` registry step runs `cargo deny` on this crate with the root `deny.toml`, whose `exceptions = []` refuses `zlib-rs` 0.6.8 (Zlib). The crate's own `deny.toml` carries that one exception under ADR 0047. The step needs the exception in the root file, or it needs to read `databases/duckdb/deny.toml`.
- `policy.py`'s binding check passes on this crate as it stands. The root workspace already excludes `databases`.
- `sdlc/surfaces.txt` moves `databases/duckdb` to landed, as the port brief asked.
