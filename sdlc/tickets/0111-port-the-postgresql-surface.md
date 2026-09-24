---
flow: build
priority: 111
opens: databases/postgresql sdlc/scripts sdlc/issues sdlc/planning/databases/postgres.md sdlc/planning/adr/0043-postgresql-single-row-calls-stop-by-deadline.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0111: Port the PostgreSQL surface

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the PostgreSQL extension from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API as the unpublished crate `thinkthen-postgresql` at `databases/postgresql`, in its own Cargo workspace. `CREATE EXTENSION thinkthen` keeps its SQL names: `thinkthen_decide` (scalar and array), `thinkthen_probability`, `thinkthen_choose`, `thinkthen_score`, `thinkthen_tag`, `thinkthen_annotate`, `thinkthen_details`, `thinkthen_usage`, `thinkthen_warm`, `thinkthen_recognize`, `thinkthen_relations`, and `thinkthen_relate`. `WHERE` and `ORDER BY` stay the filter, rank, and find verbs. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` lists PostgreSQL among the remaining surfaces.

Draft ADR 0047 (on the 0093 branch until 0093 lands) fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern. Ticket 0095 fixes the members this binding calls. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. ADR 0043 on main makes this ticket its owner. Accepted ticket 0105 is the template, and ticket 0110 (DuckDB) makes the same relate and deny choices. Ian can overturn every decision below.

## Design and decisions

1. **No C door underneath.** The extension links `thinkthen` by path through pgrx and never links `thinkthen-c` (ADR 0047 item 1). A C hop would add JSON text per call and a second FFI edge. From 0094 it takes only the pattern: one `unsafe` module tree, one error table, and the output-name collision check behind R2-26.
2. **Names.** The package is `thinkthen-postgresql`, so no second `thinkthen` library sits beside its dependency. The SQL extension stays `thinkthen`, and the installed files stay `thinkthen.control`, `thinkthen--<version>.sql`, and `thinkthen.so`. If pgrx 0.17 names the files after the package, the check's install step renames those three files and edits nothing else. Needing more stops the ticket.
3. **Engine and settings.** Every call uses `thinkthen::default_engine()`, built lazily in each backend. `_PG_init` registers settings and touches no engine. The key, the address, and the cache folder come from the server's environment (`THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, `THINKTHEN_CACHE`), as ADR 0017 section 5 rules for databases. `thinkthen.api_key` stays registered (`Suset`, hidden) and refuses when set, naming `THINKTHEN_API_KEY` and never echoing the value. This answers open question 4 of `postgres.md`: the environment alone at 0.1. Three reasons: ADR 0017's settings table, the setting's leak into `pg_stat_statements` on PostgreSQL 15 to 17 (`postgres.md`), and no public builder that starts from the environment. The extension adds no width or address setting, and the width stays the engine default of 4. Lever: an `EngineBuilder` member that starts from the environment, as a 0095 amendment.
4. **Saved answers move to the engine cache.** The per-backend answer map, `thinkthen.saved_answer_kb`, and the binding's hit counter leave. `thinkthen_warm` fills the engine's disk cache at ADR 0017's default folder for the server's operating-system user, or `THINKTHEN_CACHE`. A decide after a warm pass reads that cache and sends nothing, in the same session and in the next. `thinkthen_usage` reads `Counters` alone. Cost: a cached answer costs one file read, a 5.7 µs median by ADR 0017 section 5. The README tells a reader to raise the 100 MB cap before a warm pass over more than about 25,000 answers.
5. **Deadlines.** `thinkthen.deadline_ms` stays `Userset` with the range -1 to 2,147,483,647, so PostgreSQL itself refuses -2. Every call passes it through `CallOptions::deadline_millis` (ADR 0041, ADR 0043). `-1` is none, and `0` returns 57014 with nothing sent.
6. **Batches hear a cancel through the interrupt check.** The array `thinkthen_decide`, the `thinkthen_warm` finalizer, and `thinkthen_relate` pass one `CallOptions::interrupt` check. It reads `QueryCancelPending` and `ProcDiePending` with volatile reads in the FFI module and never calls into PostgreSQL. It never runs `check_for_interrupts!`, because that jumps across engine frames. After `Cancelled` the binding runs `check_for_interrupts!()`, which raises PostgreSQL's own error for a user cancel or a statement timeout. If it returns, the binding raises `thinkthen cancelled` with 57014. Any other interrupt, such as a memory-context request, leaves the check false and is serviced after the call. The tag's `run_batch` thread and its poll loop leave. No new send starts within one 50 ms tick (0095). Sent requests finish under 0073 before the statement ends.
7. **Single-row calls stop promptly on a cancel.** `decide`, `probability`, `choose`, `score`, `tag`, scalar `annotate`, `details`, `recognize`, and `relations` each run on a spawned worker, as 0105 decision 6 and 0110 decision 5 do. The worker owns a clone of the question, the text as a `String`, and a clone of an internal `CancelToken`. It builds its own `CallOptions` with that token and the deadline. The backend thread blocks every signal with `pthread_sigmask` before the spawn and restores its mask after, so PostgreSQL's handlers keep running on the backend thread. It then waits on a channel in 50 ms ticks and reads the two flags of decision 6. On a cancel it cancels the internal token, detaches the worker, and runs `check_for_interrupts!()`. The detached send finishes under 0073, and no retry starts. This amends ADR 0043: `pg_cancel_backend` and `statement_timeout` now stop a single-row call within one tick, and the deadline stays its budget. The acceptance list of `postgres.md` asks for exactly this. Cost: about 60 lines and one thread spawn per row, cache hits included. The record measures a 20,000-row scalar decide over cached answers and stops the ticket above 100 µs a row. Until its send ends, a detached send holds one width permit. Ian can overturn this and keep ADR 0043's deadline-only rule.
8. **Question arguments.** A question, set, or spec is `'@file.json'` or the file grammar's JSON text. Bare text refuses with the tag's sentence naming the `@` form (R2-4). The binding reads a named file itself under the privilege gate, the 1 MiB cap, the regular-file rule, and `open_beneath` confinement. It then calls `Question::from_json`, `QuestionSet::from_json`, `Recognize::from_json`, or `Relate::from_json`. A `Usage` from a file's text maps to `local` (0095). The members array of `choose`, `score`, and `tag` joins the question's JSON object as `options`, `levels`, or `labels` before `from_json`. A question that already carries its members and also gets an array refuses with `usage`. The tag's `with_members` rebuild and its lifting of the `recognize` section leave.
9. **Results.** `decide` returns true, false, or NULL. `probability` reads `Probabilities::YesNo` from `details`. `score` returns the position. `tag` returns `text[]`. Scalar `annotate` returns `AnnotatedRecord::value_json` as `jsonb`, and `details` returns `Details::to_json`. `thinkthen_usage()` returns `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`. Tests assert differences around a call (0095). `thinkthen_recognize` returns `(name, kind, start, end, strength)`, main's shape (port guide 4.3), and the `text` column becomes `name`. Offsets count Unicode scalar values, which PostgreSQL counts as characters in a UTF-8 database. `thinkthen_relations` returns `(relation, source_name, source_kind, target_name, target_kind, probability)`.
10. **Relate from rows.** `thinkthen_relate(query, rules)` reads a query whose columns are id, name, and kind. A two-column query of id and name reads kind `*` and takes only bare rules, as 0110 does. `rules` is a `text[]` of the command's inline rule strings, or `text` holding `'@file.json'` or a version-one relate file. Per ADR 0047 item 9, the binding dedupes rows by name and kind in first-seen order, calls `relate_with` once, and returns one row `(relation, source, target, probability)` per matching id pair. Ids stay `bigint`, as at the tag. SPI reads under `LIMIT 256`, and a 256th row refuses with 22023 naming the cap (R3-12's shape).
11. **Errors and panics.** One exhaustive `match` maps each `ErrorKind` to its SQLSTATE: `usage` 22023, `backend` 38000, `local` 58030, `cancelled` and `deadline` 57014, `defect` XX000. The message keeps the tag's shape, `thinkthen <kind>: <message> (retryable: yes|no)`, with the kind from `ErrorKind::name`. pgrx's `#[pg_extern]` guard turns a Rust panic into an error, and the binding writes no second guard. A worker that dies closes its channel, and the backend thread raises `defect`. `thinkthen` stops engine panics at its public methods (0086).
12. **Authority.** The install block that revokes PUBLIC, the scoped event trigger, the narrowed grant in the README, `thinkthen.file_directory` (`Suset`), and the `pg_read_server_files` gate move unchanged.
13. **Two engine images (G9).** A backend that also loads the Python wheel through PL/Python holds two engines. The README states whichever answer Ian gives to ADR 0047 item 5.

## What moves from the tag

- `src/lib.rs` and `src/descriptor_path.rs` split into files under 500 nonblank lines each: `src/ffi/` (the pgrx functions, `_PG_init`, the flag reads, the signal mask, and the descriptor opens), `files.rs` (the named-file gate), `call.rs` (the worker, options, and error table), `relate.rs`, and `warm.rs`. Only `mod ffi` carries `#[allow(unsafe_code, reason = "…")]`. `src/bin/pgrx_embed.rs` and `thinkthen.control` move.
- `fixtures/`, `examples.json`, and `tests/examples.py`. Each expected value is re-derived against 0092's generic arm, and each example runs through its own `psql` session.
- `runner.py`, rewritten onto main's `conformance/cases.json` with no skip list. `scripts/pgrx-package-locked.sh` moves to `databases/postgresql/`.
- `check.sh`, rewritten for a local server (see the gate section). Its acceptance steps keep their assertions: the slide sample, the grant suite, bad questions in batches, the deadline steps, the file gate, `from`/`to` refused, the 256th row, and the update-path trigger.
- `README.md`, rewritten for decisions 3 to 10, with the "Authority: who may do what" section kept.

These stay at the tag or retire: `package.sh` and its tarball (the release ticket, queue item 4), `tools/warm_wire_probe.sh` (the warm test covers it), the `synthetic-partial` feature, the three Docker containers, and the wire stub on port 8219. `NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 120 lines. `sdlc/planning/databases/postgres.md` is the design page, and this ticket updates it. The `site/src/data/examples/*__postgresql.json` files stay untouched. The builder files an issue in `sdlc/issues/` naming the changed recognize, relate, and usage columns for the site's owner.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 18 PostgreSQL rows. The port guide classes 17 as binding and R5-19 as engine. This ticket re-proves 15 binding rows and the PostgreSQL half of R5-19, and it retires R5-17 and R7-6 with the answer map. It also carries the PostgreSQL halves of fourteen cross-surface rows. Each re-proof runs against the real engine through a 0092 backend the check starts, unless marked as a unit test. "Counted" means that backend's `count` line. "Held" means 0092's held arm. Every held test ends by a release, a cancel, or a deadline, and runs under `timeout 30`. The record plants each bug below, shows its test turning red, then green once the bug is removed.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-18 | closed | After `CREATE EXTENSION`, PUBLIC holds EXECUTE on no extension-owned function. An ungranted role gets "permission denied for function thinkthen_decide" and zero counted sends. | Drop the install revoke block. PUBLIC holds EXECUTE. |
| R2-4 | closed | Bare `'names.json'` and `'form.json'` refuse with the pinned sentence naming the `@` form. The narrowed grant reaches the extension's functions and not a control function. A synthetic update script's new function is not PUBLIC's. | Read bare text as a path. The bare-path call answers. A second plant drops the event trigger, and the update test turns red. |
| R3-2 | closed | `'@/dev/zero'` refuses with the pinned "did not read" sentence within 2 s, and `SELECT 1` answers in that session. A 1,048,577-byte file names the cap. | Drop the regular-file check. `/dev/zero` answers with the cap sentence. |
| R3-8 | closed | A role with EXECUTE alone cannot read `@/etc/hostname`, and the refusal names `pg_read_server_files` and `thinkthen.file_directory`. Inside a configured directory the read answers. With `pg_read_server_files` it answers anywhere. | Skip the privilege gate. The EXECUTE-only read answers. |
| R3-9 | closed | A check step extracts the README's grant block and compares it byte for byte with the fixture `fixtures/grant.sql`. | Replace the block with a grant `ON ALL FUNCTIONS IN SCHEMA public`. The comparison fails. |
| R3-10 | closed | A deliberate `GRANT EXECUTE` to PUBLIC on `thinkthen_decide(text, text)` survives an unrelated `CREATE FUNCTION`. | Drop the `pg_event_trigger_ddl_commands()` join. The grant is revoked. |
| R4-6 | closed | Unit tests: a final-component symlink and an intermediate symlink that point out of the directory refuse, and a fifo refuses within 50 ms. The record runs the tag's swap probe once for 20,000 cycles and reports the leak count. | Confined reads call `open_plain`. The intermediate-symlink test reads the outside file. |
| R5-15 | closed | Unit test: an outside file over the cap gives the one refusal, never the cap sentence. | Check the size before confinement. The cap sentence appears. |
| R5-16 | closed | Unit test: a hard link inside the directory to an outside file refuses. | Drop the link-count check. The outside text reads. |
| R6-8 | closed | The tag's unit test `an_outside_path_refuses_before_any_open` moves unchanged. | Open before the spelling check. The test sees the open. |
| R1-19 | closed | A missing and a broken question file in the array form and in `thinkthen_warm` each name the file with zero counted sends. The broken file gives the pinned `local` sentence. | Replace a batch error with a fixed defect sentence. The file name is missing. |
| R1-22 | closed | Held, one scalar decide. `pg_cancel_backend` once the count reads 1 ends the statement with "canceling statement due to user request" within 200 ms. `statement_timeout = '300ms'` ends it within 500 ms. `deadline_ms = 200` returns 57014 with `thinkthen deadline` within 1 s. Each test then releases the reply, and the count stays 1. | Run single calls on the backend thread. The 200 ms assertion turns red, and the release at 3 s ends the test. A second plant omits the deadline, and the 1 s assertion turns red. |
| R3-21 | closed | A two-record array batch on the generic arm answers within 90 ms, measured inside the server. | Sleep one 100 ms tick before the first pull. |
| R2-19 | closed | Warm over 2,000 pairs counts N sends. A row-by-row decide pass over the same pairs adds 0 in the same session and 0 in a new one. 20,000 warm rows answer within 30 s, and the 20,001st refuses naming the cap. | Warm through a second engine built with `no_cache()`. The decide pass sends 2,000. |
| R6-14 | closed | Warm under one model, then decide the same pairs with a question naming another model. The second pass sends again. | Add a binding memo keyed by the evidence alone. The second pass adds 0. |
| R5-17 | waive | Retires with the answer map (decision 4). `SHOW thinkthen.saved_answer_kb` raises "unrecognized configuration parameter". | Keep the setting registered. `SHOW` answers. |
| R7-6 | open | Retires with the answer map. The R5-17 test covers it. | As R5-17. |
| R5-19, PostgreSQL half | closed; engine row, 0085 | Held, a 200-row array batch under `statement_timeout = '500ms'` ends with the timeout error. After the release, the next scalar decide in that session adds exactly one counted send. | Reuse one backend-wide cancel token across calls. The next decide returns `cancelled` with zero sends. |
| R2-24, PostgreSQL half | closed | `deadline_ms = 0` refuses the array form and warm with 57014 and zero counted sends. Held, `deadline_ms = 1000` ends a 200-row array batch with `thinkthen deadline` within 1.5 s and 4 counted sends. | Build batch options without the deadline. The spent case sends. |
| R2-24 G1 half (batch cancel) | closed | Held, a 200-row array batch. `pg_cancel_backend` once the count reads 4. The count stays 4 for 300 ms. The test releases, and the statement ends with the cancel error within 1 s, still at 4. The same shape with `pg_log_backend_memory_contexts` finishes the batch with 200 counted and a count row. | An interrupt check that returns false. The cancel test counts 200. A second plant reads `InterruptPending`, and the benign test fails. |
| R2-10, PostgreSQL half | partial | `SET thinkthen.deadline_ms = -2` fails with PostgreSQL's range error. `-1` runs, and `0` gives 57014 with zero sends. | Lower the floor to the `i32` minimum and treat any negative as none. `-2` runs. |
| R1-10 host half | engine | A test-only feature `panic-probe` adds `thinkthen_panic_probe()`. It raises XX000 with the panic's text, and `SELECT 1` answers in that session. A package check proves the shipped tree lacks the function. | Build the test library with `panic = "abort"`. The session dies. |
| R1-31, R2-31 | waive, partial | One error table. A unit test maps each `ErrorKind` to its SQLSTATE and kind word. A check step counts zero `catch_unwind` sites in `src`. | Map `Deadline` to 38000. A second plant adds a `catch_unwind`. |
| R2-32 | closed | Deny passes with `databases/postgresql/deny.toml`: the root file plus one `ignore` for RUSTSEC-2021-0127 (`serde_cbor` 0.11.2 under pgrx 0.17.0) and one `[[licenses.exceptions]]` entry for `foldhash` (Zlib). A `lint` step compares it with the root file. | Drop the `ignore`. Deny fails on RUSTSEC-2021-0127. |
| R3-29, R4-19, R5-34, PostgreSQL halves | partial, closed | Every `cargo` call passes `--locked` and `--offline`. `cargo pgrx package` runs only through the locked wrapper. A check step reads the scripts and fails on a call without the flags. | Drop `--locked` from the unit-test call. The step fails. |
| R3-30, R5-29, R5-30, R5-32, PostgreSQL halves | closed | The runner has no skip list. It reports every case as pass, fail, or not run with a reason, and the three counts sum to the file's count. Cases 24 and 40 print and count. | Skip case 40 silently. The sum check fails. |
| R1-28, R3-28, PostgreSQL halves | closed | The lint table equals the root's except `unsafe_code = "deny"`. `policy.py` refuses `unsafe` outside `src/ffi/`. | Put one `unsafe` block in `src/warm.rs`. `lint` fails. |
| R1-30, R3-31, PostgreSQL halves | closed | The packaged `thinkthen.so` holds zero copies of `$HOME`. | Drop `--remap-path-prefix`. The count is nonzero. |
| R3-32, R4-10, PostgreSQL halves | partial | `check.sh` states Linux only. A self-test runs it with a `uname` shim that prints `Darwin` and gets "not run". It uses no `sed -i` and no `date +%N`. The `package.sh` halves move with `package.sh`. | Delete the kernel check. The shimmed run reaches the server step. |

Rows that retire here: R1-29's PostgreSQL half (no image; the runtime's hash is pinned) and R5-36's (no wire stub; a missing backend reports not run under 0093's rung). R1-11's host half has no reachable input, because the setting stops at 2,147,483,647 ms, and 0086 carries it. R5-3 and R6-3 stay with 0096 and 0078, and the preload and benign tests observe them here. The freeze record's three follow-ups name no PostgreSQL code.

R2-29 asks for rulings on record. This ticket amends ADR 0043 (decision 7) and adds a PostgreSQL section to ADR 0047 for decisions 3, 4, and 10. The code review checks both records.

## Other acceptance

- Red first: the ported tests fail against an empty `databases/postgresql` workspace for the stated reason, then pass.
- `cargo test --locked --offline --lib` in `databases/postgresql` passes.
- The conformance runner restarts the local server with `THINKTHEN_BASE_URL` at each case's `/case/ID/` path. Case 24 runs through `deadline_ms = 0`. Case 40 asserts counter differences. Case 41 returns the same `start` and `end` as Rust. Cases 51 and 52 run through the relate file form. Cases 13 to 16, 18-find, 19, 26, and 31 report "not run: SQL spells filter, rank, and find with WHERE and ORDER BY". Case 23 reports "not run: SQL has no token; the R1-22 and R2-24 tests cover cancel".
- Preload and fork: with `shared_preload_libraries = 'thinkthen'`, two fresh sessions each answer with one counted send (0096). Plant: `_PG_init` makes one decide call. The server fails to start within `pg_ctl -t 10`, and the test turns red and ends.
- The worker survives a closed channel, and a channel that closes with no result raises `defect`. A Rust unit test covers each edge.
- The drawn slide SQL and every `examples.json` entry answer as expected. The two-column relate call runs as drawn.
- Secrecy: `SET thinkthen.api_key` gives 22023 naming the setting and `THINKTHEN_API_KEY`, and the value appears in no output and no server log line. `THINKTHEN_API_KEY` stays unset in the gate. A test reads every error message for the key and the address's credentials.
- Nothing reaches a non-loopback address. The server listens on no TCP port.

## The check it adds to the gate ladder

- `databases/postgresql/check.sh` joins the surface registry as landed. The `surfaces` rung (ADR 0047) runs it with the 0092 port for the case and generic arms. For count and release, each held test starts its own backend from the binary the rung builds, so one `release` never reaches the next test.
- **Where it runs.** On Beelink, the Linux machine that owns this repository, with no network and no Docker. The freeze record says the tag's check did not run, because it ran PostgreSQL 16 in Docker. Observed by command on 2026-09-24:
  - Ubuntu 24.04 with `postgresql-client-16` and `postgresql-server-dev-16` at 16.15-0ubuntu0.24.04.1, `/usr/bin/pg_config`, `cargo-pgrx` 0.17.0, `rustc` 1.93.1, libclang 17, and `cargo-deny`.
  - `cargo check --offline --locked --lib` on the tag's crate exits 0 in 69 s. The cargo cache holds pgrx 0.17.0 and its tree.
  - Every shared library the server package needs is installed: libicu74, libllvm17t64, libssl3t64, libxml2, libxslt1.1, liblz4-1, libzstd1, libldap2, libgssapi-krb5-2, libpam0g, libselinux1, libsystemd0, libuuid1, and libpq5 16.15.
  - No PostgreSQL server binary (`postgres`, `initdb`, `pg_ctl`) exists on the machine outside the cached image `postgres:16@sha256:a3b7f434…`. No `.deb` or source tarball is cached. So the check cannot run offline without Docker today.
- **Options.**
  - (a) Recommended. Fetch Ubuntu's `postgresql-16_16.15-0ubuntu0.24.04.1_amd64.deb` once with `apt-get download` (no sudo). It is 15,610,426 bytes, SHA256 `d60ae76fb862cd4cca7bfdcdcdcc271c5e1d75f6b7915305ce2d962735a4b666` from apt's metadata, and it matches the installed headers exactly. Keep it at `~/.cache/thinkthen-dev/postgresql/`. Cost: one network fetch, then offline forever.
  - (b) Build 16.15 from source with `cargo pgrx init --pg16 download`. Cost: one fetch, a long compile, and a rewrite of `~/.pgrx/config.toml`, which other worktrees read.
  - (c) Keep Docker with the cached pinned image. It works offline today, but it starts containers and breaks the freeze's no-Docker condition.
  - (d) Install `postgresql-16` system-wide. Cost: root, a fetch, and a system service on port 5432.
  The builder takes (a), fetches once, and the record says so. Ian can pick (c) instead.
- **How.** `databases/postgresql/runtime.sh` checks the package's SHA256, extracts it with `dpkg-deb -x` into the gitignored `databases/postgresql/.runtime/`, and proves the tree relocates: `postgres -V` answers, and `pg_config`'s `SHAREDIR` from a running server names the extracted tree. A missing or mismatched package, `dpkg-deb`, or `cargo-pgrx` reports "not run" with the fetch command, and never "pass" (R6-2). `check.sh` builds with `/usr/bin/pg_config`, copies the three packaged files into the extracted tree, runs `initdb` into a `mktemp` folder, and starts `postgres` as the user with `listen_addresses = ''` and its socket in that folder. `THINKTHEN_BASE_URL` and `THINKTHEN_CACHE` go in the server's environment, and each restart sets them fresh. An exit trap runs `pg_ctl stop -m immediate` and removes the folder. No root, no TCP port, no system service, and no container.
- **Time.** After a warm cargo target the SQL half ends within 6 minutes on Beelink. `check.sh` runs under `timeout 900`, and each `psql` call runs under `timeout 30`.
- `check.sh` drops `ENGINE_NULL`, `ENGINE_BASE_URL`, the `synthetic-partial` build, and the stub on port 8219. Every `cargo` step passes `--locked` and `--offline`.
- `lint` runs on `databases/postgresql`: the ADR 0047 manifest, lock, lint-table, and profile checks; the registry check; `ratchet.mjs` on `databases/postgresql/ratchet.json` for Rust and `ratchet.py.json` for Python; and deny as `cargo deny --offline --manifest-path databases/postgresql/Cargo.toml check --config databases/postgresql/deny.toml advisories bans licenses sources`. The deny plant is a git-sourced dependency, which `[sources] unknown-git = "deny"` refuses.
- **Deny.** Observed with that command against the root config on the tag's crate: `foldhash` 0.1.5 (Zlib, through `petgraph` under `pgrx-sql-entity-graph`) fails licenses, and `serde_cbor` 0.11.2 (through `pgrx`) fails advisories as RUSTSEC-2021-0127. Bans and sources pass. The binding's file adds the two entries of R2-32, and the root file stays untouched. `sdlc/issues/2026-09-22-unmaintained-crates-in-the-surface-lockfiles.md` argues the ignore. The builder revisits it at the first pgrx release without `serde_cbor`.
- **Lints.** The root forbids `missing_debug_implementations`, `unreachable_pub`, `unsafe_code`, and `unexpected_cfgs` outside its one `check-cfg` name. It denies `expect_used`, `unwrap_used`, `indexing_slicing`, and `panic`. The tag's lock handling and indexing need rewrites under those. If pgrx's generated code trips a forbid-level lint, the builder stops and records the case for an ADR 0047 amendment. The likely case is `pgrx_embed!`'s `cfg(pgrx_embed)` against `unexpected_cfgs`.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off, `pgrx = "=0.17.0"` and `libc` at the tag's locked version, both cached here. `serde_json` stays for the members merge of decision 8, and the record says whether it still earns its place. The tag's `serde` derive leaves unless the record names a use.
- Build: `cargo-pgrx` 0.17.0, installed here. Runtime: the PostgreSQL 16.15 package of option (a). Tests: Python 3 with the standard library only.
- These enter main for the first time. The code reviewer checks each entry, the binding lock, the deny file, and deny's result, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust: at most seven files and 1,450 nonblank lines, each file under 500. The tag measures 1,461 (1,337 in `lib.rs` and 124 in `descriptor_path.rs`). The answer map, `run_batch`, the stand-in glue, and `with_members` leave. The worker, the signal mask, relate from rows, and the relate file form arrive.
- Rust unit tests: at most 450 nonblank lines. The tag measures 366.
- Scripts: `check.sh` and `runtime.sh` together at most 850 nonblank lines. The tag's `check.sh` measures 841, and about 110 of those drive containers. The held, runtime, restart, and not-run steps add about 150. `runner.py` at most 330 (the tag measures 329), and `tests/examples.py` at most 60. Gate changes under `sdlc/scripts` at most 40 nonblank lines.
- Documentation: `README.md`, the new `NOTES.md`, `postgres.md`, the ADR 0043 amendment, and the ADR 0047 section, at most 300 net nonblank lines.
- Ratchet: `databases/postgresql/ratchet.json` and `ratchet.py.json` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen` or `conformance/`, or widening the root `deny.toml`.

## Exclusions

`package.sh`, tarballs, and release builds (queue item 4). PostgreSQL versions other than 16. A Mac run. A width, address, or key setting beyond decision 3. `PARALLEL SAFE` marking. Any change to `thinkthen`, `conformance/`, the root `deny.toml`, or `site/`. Any live or paid call.

## Dependencies

After 0086. Also after 0098 (labels, spec readers, the JSON methods, and `ErrorKind::name`), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), and 0094, since the plan puts C before every other surface. 0092 and 0099 have landed.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 3; total 13. Final level: 3. A thread and a signal mask sit inside a database backend, and a wrong interrupt rule loses a cancel or spends a paid batch.

## Review

- Design review: pending.
- Code review: pending.
