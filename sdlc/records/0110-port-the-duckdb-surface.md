# Record: ticket 0110, port the DuckDB surface

Branch `ticket/0110-port-duckdb-surface`. Built on Beelink on 2026-09-25 with the spike 253 toolchains under `~/.cache/thinkthen-toolchains/duckdb/v1.5.5/`: the stock CLI `v1.5.5 (Variegata) d8cdaa33fd` at sha256 `3d33b1df…5788`, and a venv holding `duckdb==1.5.5` under CPython 3.13.5. Nothing was downloaded.

## What landed

- `databases/duckdb`, its own Cargo workspace, on `thinkthen`'s public API and `libduckdb-sys =1.10505.0` with no wrapper crate. `src/` holds 14 files. Every `unsafe` line sits in a file named `ffi.rs`: `src/ffi.rs` for the types, chunk reads and writes, setting registration, and LOAD, and one `ffi.rs` beside the questions, scalars, tables, and signal modules. Each file holds under 500 nonblank lines.
- `check.sh`, which builds the shipped and `test-hooks` extensions, runs the source checks, deny, and its plant, loads the extension in the stock CLI, and runs the suites.
- `tools/`: `harness.py`, `verbs_suite.py`, `settings_suite.py`, `signal_suite.py`, `conformance.py`, `site_examples.py`, `source_checks.py`, `selftests.sh`, `setup.sh`, `version.env`, and `requirements.txt`. That is 11 files, under the ticket's 16.
- `deny.toml`, the root copy plus one exception for `zlib-rs`.
- `README.md`, a new `NOTES.md`, the first part of the ADR 0038 amendment, the planning page's wrapper paragraph, the site-example issue, and the query-hook issue moved from the tag with an update.
- `sdlc/surfaces.txt` stays main's copy, with `databases/duckdb` planned. The landing sets the registry line.

## Deviations from the ticket

- `src/ffi/` became one `ffi.rs` per module. `policy.py` allows `unsafe` only in a file named `ffi.rs`, and the ticket may not change the ladder scripts. The first port held every C API call in one 928-line `src/ffi.rs`. A mechanical split moved the file system into `src/questions/ffi.rs`, the scalar callbacks into `src/scalars/ffi.rs`, and the usage table and warm aggregate into `src/tables/ffi.rs`. The engine calls behind the scalars moved into `src/scalars/calls.rs`. R4-16's types rule reads the same way: every logical type is made and destroyed inside the `Logical` wrapper in `src/ffi.rs`, whose `Drop` destroys it.
- The binding uses `libduckdb-sys` alone. `duckdb`'s `VScalar` registers no init callback, and decision 10 needs one on every scalar. `foldhash` and `tiny-keccak` left the tree with `arrow` and `hashlink`. `cargo deny` with the root config now refuses only `zlib-rs` 0.6.8 (Zlib), through `flate2` and `zip` in `libduckdb-sys`'s build dependencies. The deny plant removes the `zlib-rs` exception, and `check.sh` pins deny's rejection line for `zlib-rs`.
- The conformance runner reads find cases, case `18-annotate-two-groups`, and relate cases as "not run" from one closed list. SQL finds with `ORDER BY` and `LIMIT` over decide. Main's public API refuses `on` in a library question set. Relate lands in 0118. The recognize cases run through `thinkthen_relations` with each case's own question file and compare the relations. Entity offsets of a described kind are not readable from SQL, because `thinkthen_recognize` takes kind names only. `verbs_suite.py` checks case 41's offsets on the generic arm.
- R3-13's `SA_SIGINFO` half is proved with a host handler installed through `ctypes` and `sigaction` before LOAD. It reads signal 2 and the parent's process id.
- Review finding 1 asked for a per-text cost equal to an annotate set's group count. The engine groups a set's members by their `on` pointers, and a library set refuses `on`, so every library set holds one group. A two-member set over three texts sent 3 requests on the generic arm. A total of 2 over those texts sends 2 and refuses. The test pins that, and the cut stays one text per request.
- LOAD now returns false when DuckDB does not offer the v1.5.5 C API. No test reaches it, since the stock CLI always offers it.
- The engine map builds an engine outside its lock and stores it under the lock, so a fork during a build never leaves the child's map locked. A racing build of the same key loses to the one stored first.

## Results

- `check.sh` passes at `d2b0f43f`, the head with every review fix and the fork test, after the merge of main at `eb3fae21`: fmt, clippy with `-D warnings`, the unit tests, both builds, the source checks, deny and its plant, the stock CLI call, every suite, conformance, the site examples, and the selftests. It prints 51 `ok` lines. The site-example check runs 8 drawn blocks and names 2 closed divergences.
- Ratchets: `src` holds 2254 non-blank Rust lines and `tools` holds 1349 non-blank Python lines. Each ceiling equals its total. See the re-scores below.
- Conformance: 49 pass, 0 fail, 5 not run, 54 cases.
- Owed proofs, added after the first review: R1-15 counts 0 opens of `q.json` under `strace` with access off. R2-18 counts 1 open over 20,000 rows under `SET threads = 1`. R5-21's 10,000 SIGINTs while four threads allocate end with exit 0 well under 60 s. An `@file` read of `/dev/zero` stops at 1 MiB. The secrecy case now covers every verb's refusal on the refuse arm with its `backend` kind pinned, every details member on the generic arm, a usage error, a local error, and an address carrying a password, with more than 0 counted sends.
- Fork (0096): a parent loads the extension, warms, decides under a total of 10, and forks. The child opens its own database, sets a total of 1, and answers, and its `requests_sent` reads 1. The parent's reads 2 before and after the fork, and the backend counts 3. The parent bounds its wait for the child at 30 s. The test also stands as the proof for the map's build outside its lock. It shows the child uses the parent's map after a fork. It does not fork during a build.
- Access cases: all 35 cache cases agree with DuckDB's `COPY … TO`, and all 35 `@file` cases agree with `read_text`. Each refused cache case sent nothing and created nothing in the case folders. The check reads folder listings, not `strace`.
- SIGINT: a held batch of 64 texts at throttle 8 and a single held `thinkthen_details` each read `cancelled` within 100 ms, and the count stayed put after release. 50 stop-then-answer rounds, 50 signals between queries, and 20 chained signals all pass.
- The volatile test uses `EXPLAIN`, with a count of 0.
- Request total (decision 17, Ian's ruling of 2026-09-25): a total of 3 and a decide over 10 distinct rows sent exactly 3 requests on the generic arm and read the pinned sentence, and the next call read it too with the count still 3. The plant runs the call unchecked and checks the total only when the map builds an engine.

## Planted bugs

Each plant changed one or two lines, rebuilt the extension, and ran its one case. The source was restored and touched after each. All went red except the one row marked green.

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
| Probe only when the map builds an engine | `two_databases_each_judge_their_own_access` | B's decide answered |
| One process-wide `@file` cache read before the open | `two_databases_each_judge_their_own_access` | B read A's file |
| A one-argument host handler is not called | `r6_6_a_chained_host_handler_sees_every_signal` | the host saw 0 of 20 |
| The callback body runs without `catch_unwind` (test-hooks build) | `r1_10_a_panic_in_each_boundary_reads_defect` | the process aborted |
| An engine failure echoes the key | `secrecy_no_key_or_credential_in_any_message` | the sentinel key in a backend error |
| Read the file on every call | `r1_15_and_r2_18_file_opens_under_strace` | 10 opens for 1 |
| Open the file with `std::fs`, past the caller's settings | `r1_15_and_r2_18_file_opens_under_strace` | the refused read answered |
| `Mutex::lock` in the handler | `source_checks.py` | R5-21 names `lock` and `Mutex` |
| The total keeps a spent count a fork inherits | `a_forked_child_answers_from_a_zero_total` | the child read the spent-total sentence under a total of 1 |
| Annotate spends two requests per text | `an_annotate_set_spends_one_request_per_text` | 1 send for 2 |
| No cap on an `@file` read | `an_atfile_read_stops_at_one_mib` | the 30 s timeout fired |
| The total checked only at the engine call | `a_negative_request_total_refuses_before_the_map` | a NULL row answered |
| Install without `SA_SIGINFO` | `r3_13_an_siginfo_host_handler_gets_the_number_and_sender` | a garbage sender id |
| Count engine calls only (R6-6) | `r6_6_a_chained_host_handler_sees_every_signal` | green: see below |

The R6-6 plant starts the invoke after the chunk read. It stays green. A chunk read takes microseconds, and no outside test can land a signal inside a read that short. The fix stands, since it is strictly earlier, and this gap is deferred.

## Not yet done

Deferred gaps, each with its reason:

- `examples.json` and its runner. The site-example check covers the drawn blocks, and the relate block stays on its divergence list.
- A test that lands a SIGINT inside the chunk read (R6-6). See the plant table.
- A test for LOAD's false return. The stock CLI always offers the v1.5.5 API.

## Re-scores

The coordinator approved these on 2026-09-25, and Ian can overturn each one.

- Files: `src/` holds 14 files against the ticket's budget of 12. The review's splits of `src/ffi.rs` and `src/scalars.rs` added them. The coordinator approved the 14 on 2026-09-25 as queue owner.
- Ratchets: `src` rises to 2254 non-blank Rust lines for the split's module headers and imports, the `@file` cap, and the map's build outside its lock. `tools` rises to 1288 non-blank Python lines for the strace, stress, `SA_SIGINFO`, secrecy, `/dev/zero`, and total cases. The re-review asked for the fork test, which raises `tools` to 1349. The coordinator confirmed that rise from 1288 to 1349 on 2026-09-25 as queue owner, for the 61-line fork test. Ian can overturn it.

## For the landing agent

- The `surfaces` script already reads the binding's own `deny.toml`. This crate's copy is the root file plus one license exception, `{ crate = "zlib-rs", allow = ["Zlib"] }`, for `zlib-rs` 0.6.8 through `flate2` and `zip` in `libduckdb-sys`'s build dependencies. The landing adds its `BINDING_DENY` entry: `"databases/duckdb": [("licenses", "exceptions", [{"crate": "zlib-rs", "allow": ["Zlib"]}])]`.
- The landing sets `databases/duckdb` to landed in `sdlc/surfaces.txt`. This branch carries main's copy unchanged.

