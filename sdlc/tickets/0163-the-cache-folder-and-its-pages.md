---
flow: build
priority: 163
opens: crates/thinkthen/src/engine/cache_prune.rs crates/thinkthen/src/cli/status.rs crates/thinkthen/src/cli/cache.rs crates/thinkthen/tests/backend/default_cache.rs crates/thinkthen/tests/backend/default_cache/prune.rs crates/thinkthen/tests/status.rs specification/recording.md specification/threshold.md specification/settings.md sdlc/planning/adr/0010-one-wire-shape-two-variables-and-a-smaller-version-one.md SECURITY.md libraries/c/README.md libraries/polars/README.md libraries/python/README.md libraries/r/README.md libraries/ruby/README.md libraries/rust/README.md libraries/typescript/README.md databases/duckdb/README.md databases/postgresql/README.md databases/sqlite/README.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0163: The cache folder survives a bad entry, and the pages say what it holds

Status: built on the codex-5 ticket branch, pending fresh independent code review and landing. The accepted outcome remains the 2026-09-26 ruling. Owner: Codex. Prerequisites 0148, 0155, 0158 and 0159 landed before this build.

Review route: a fresh independent Codex reviewer checks the frozen final diff, including the measured source-ceiling amendment, before landing.

## Outcome and authority

One malformed or foreign entry no longer stops `cache prune` or `status` for a whole folder. Prune carries on and names each bad entry by its file name. `status` carries on and reports how many bad entries it found. Every library and database README, the settings table and `SECURITY.md` say that the answer cache is on by default and that each entry holds the judged text. Each README says how to turn the cache off, or says that its surface has no off switch and how to move the folder. `specification/recording.md` and `SECURITY.md` say that whoever can write a cache or recording folder decides the answers read from it. `recording.md` and `threshold.md` state answer drift as a measurement over the recordings on disk shows it. `recording.md` says that a command's exit waits for the usage lock.

Five issues from local experiment 273 block 0.1 by the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. The folder-writers issue blocks 0.1 only for its documentation half. The backlog row once listed a sixth issue, the "never recorded" sentence. That issue is ticket 0158's, and this ticket removes it from the row. Commit `c09e9551` on 0158's branch rewrites `recording.md` line 84 and closes that issue, so this ticket leaves it alone. Ian can overturn each design choice.

## What happens today

Read from `origin/main` `8084d38a`.

- `engine/cache_prune.rs:200-240`, `scan`, returns `Error::CacheEntry` at the first digest-named entry that is not a regular file, changes identity while read, fails to parse, or carries the wrong name. A digest-named file that vanishes or cannot be opened fails the whole scan as a storage error at line 208 or 212. Both callers, `inspect` at line 66 and `run_at` at line 92, pass the error up. `cli/status.rs:123-129` calls `inspect` and maps any error to `Failure::StatusState`, so `status` fails whole. `recording.md` line 94 says "A scan refusal deletes nothing". The messages name no file. Report 08, finding 3 (`sdlc/issues/2026-09-26-one-bad-cache-entry-breaks-status-and-prune.md`).
- No `libraries/*/README.md` or `databases/*/README.md` says that a cache entry holds the judged text. `README.md` line 38 says it for the command. The settings table's "Answer cache" row (line 63) gives the default and not the contents. In PostgreSQL the folder belongs to the server's operating-system user, and every role that can call the functions shares it. Report 12, finding 2.1 (`sdlc/issues/2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so.md`).
- Three surfaces cannot turn the cache off. DuckDB's `SET thinkthen_cache` only moves the folder, because `databases/duckdb/src/engines.rs:129` calls `cache_at` and nothing calls `no_cache`. PostgreSQL's `thinkthen.cache` only moves it. An empty value keeps `THINKTHEN_CACHE` or the platform folder (`databases/postgresql/src/call.rs:127` and `:147`, `databases/postgresql/README.md` line 49). The C library reads only `THINKTHEN_CACHE`, and `settings.md` line 63 marks its cell "not on this surface".
- `engine/recorder.rs:137` checks folder privacy only for the platform default. Entries carry no integrity check. Report 12, finding 2.2, changed `"noul": 0.93` to `0.01` in one entry, and the next run printed `false` with `requests_sent: 0`. `recording.md` line 62 treats every entry field as untrusted text for printing and says nothing about answers (`sdlc/issues/2026-09-26-folder-writers-decide-the-answers.md`).
- `recording.md` line 78 and `threshold.md` line 45 say borderline answers moved "by up to 0.08" and that experiment 259 saw gaps "up to 0.09". ADR 0010's amendment of 2026-09-25 holds the same numbers. Report 09, finding 3, counted the Beatles Bench recordings, all answered by `jev-1.13.0`: 4,075 of 5,290 repeated digests differed, 263 by more than 0.1, and the largest gap was 0.45. One digest replays as 0.21, 0.25 and 0.66 for one label, so it crosses the default cut of 0.5 (`sdlc/issues/2026-09-26-the-spec-understates-answer-drift.md`).
- `recording.md` line 26 says "Counting never holds back a request." `engine/usage.rs:263` takes the usage lock with `File::lock`, which has no timeout, and the command's exit waits for the usage writer. ADR 0049 item 3 accepts the wait. Report 07, finding I2, held the lock for 12 seconds, and `decide` printed `true` at once and exited after 11.73 seconds (`sdlc/issues/2026-09-26-recording-page-omits-the-exit-wait-on-the-usage-lock.md`).

## Design

### A bad entry is skipped

`scan` no longer stops at a bad entry. It sets the entry aside and keeps going. A bad entry is a digest-named object that meets any of these:

- It is not a regular file. A symlink or a directory counts.
- It fails to open for a reason other than not found, such as permission denied.
- It changes identity while read, or a read of it fails.
- It fails to parse.
- It names another schema or adapter.
- Its response names no nonblank model (`Entry::inspected` returns `MissingModel`, `core/recording.rs:212`; `recording.md` line 94).
- Its name does not match its digest.

A digest-named file that is not found at `symlink_metadata`, at `open`, or at the final `symlink_metadata` vanished after `read_dir` listed it. `scan` skips it as not an entry and neither counts nor names it. One file therefore never fails the whole folder. A failure of `read_dir` itself, of the folder checks in `inspect`, or of the folder lock still fails the command as today.

`scan` returns the good entries and the names of the bad ones, sorted by name. The name matched the digest pattern, so it is 64 lowercase hex characters and `.json`, and printing it prints no entry text.

### What prune prints

Prune never deletes, follows or opens a bad entry again. It applies the alias rule, the selectors and the target to good entries only, as today. A bad entry's bytes count toward no total.

On success prune writes this whole line to standard output, where both counts after the semicolon cover good entries only:

```text
removed N entries and B bytes; N entries and B bytes remain
```

It then writes one whole line to standard error for each bad entry, in name order:

```text
thinkthen: cache prune: left `NAME` in place; it is not a valid entry
```

It exits 0. The alias refusal at exit 2 and a later filesystem failure print as today and name no bad entry.

### What status prints

`status` reports a count of bad entries and names none. It no longer fails when an entry is bad.

- The human output prints `cache_bad_entries N` on the line right after `cache_bytes`. When there is no cache to inspect, it prints `cache_bad_entries unavailable`, as `cache_entries` does today.
- `status --json` adds `cache.bad_entries` right after `cache.bytes`. It is a whole number, or `null` when there is no cache to inspect.
- A folder that does not exist yet reports `cache_bad_entries 0`, as it reports `cache_entries 0`.
- The addition keeps `thinkthen.status/1`, by the rule ticket 0160 writes for results.
- `recording.md` line 20 lists what `status` reports. It adds "the count of bad cache entries" after "answer-cache entry count and allocated bytes".

### The recording page states the rule

`recording.md` line 94 becomes: "Prune validates every digest-named object before it deletes one. The name must match the digest recomputed from the fixed adapter, stored URL, and exact request bytes, and the response must name a nonblank model. A digest-named object that is not a regular file, cannot be opened or read, does not parse, or fails either check is a bad entry. Prune and `status` never follow, open again or delete a bad entry. Prune leaves it in place and names it on standard error with ``thinkthen: cache prune: left `NAME` in place; it is not a valid entry``, and `status` counts it as `cache_bad_entries`. A digest-named file that disappears while prune or `status` reads the folder is not an entry. Unknown names, locks, and dot-prefixed temporary files are ignored. A later filesystem failure can leave an oldest prefix deleted and prints no success line." Line 92's sentence on the printed line adds: "Both counts after the semicolon cover valid entries only." The `status` sentence on line 96 adds: "It reports bad entries as a separate count."

### Every surface says where judged text goes

Each library and database README gains one paragraph under its cache setting. It opens with: "The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. `cache prune` is the only thing that removes entries." The last sentence is exact for each surface:

| README | Last sentence |
| --- | --- |
| `libraries/rust` | "Turn it off with `EngineBuilder::no_cache`." |
| `libraries/python` | "Turn it off with `tt.Engine(cache=False)`." |
| `libraries/typescript` | "Turn it off with `new tt.Engine({ cache: false })`." |
| `libraries/ruby` | "Turn it off with `ThinkThen::Engine.new(cache: false)`." |
| `libraries/r` | "Turn it off with `tt_engine(cache = FALSE)`." |
| `libraries/polars` | "The column calls use your own `thinkthen::Engine`, so turn it off with `EngineBuilder::no_cache` when you build that engine." |
| `libraries/c` | "There is no off switch on this surface. Set `THINKTHEN_CACHE` before `thinkthen_engine_new` runs to move the folder." |
| `databases/sqlite` | "Turn it off with `thinkthen_cache(NULL)`." |
| `databases/duckdb` | "There is no off switch on this surface. `SET thinkthen_cache = '/absolute/folder'` moves the folder, and `THINKTHEN_CACHE` moves the folder `thinkthen_warm` uses." |
| `databases/postgresql` | "There is no off switch on this surface. `thinkthen.cache` moves the folder, and an empty value keeps `THINKTHEN_CACHE` or the platform folder." |

The Polars row differs from the coordinator's ruling on purpose. The Polars door adds a trait to the caller's own `thinkthen::Engine` (`libraries/polars/README.md` line 9), and that engine's builder has `no_cache`. The settings table has no Polars column, so the switch is the Rust one.

`databases/postgresql/README.md` adds: "The folder belongs to the server's operating-system user. Every role whose calls resolve to the same folder shares its answers, so row text leaves the database's own access control, row-level security included. When roles must not share answers, give each its own folder with `ALTER ROLE ... SET thinkthen.cache`."

The settings table's "Answer cache" row says "Each entry holds the judged text" in its "What it does" cell. `SECURITY.md` gains one paragraph that names the cache and its contents, and the trust rule below.

### Whoever writes a folder decides its answers

`recording.md`, after line 62, gains: "A cache or recording entry is trusted for its answer. Whoever can write a folder that `--cache`, `--record`, `--replay` or `THINKTHEN_CACHE` names decides the answers read from it, and a changed entry replays with no sign. Keep such a folder private to the people whose answers it holds. Do not restore a shared cache across a trust boundary." `SECURITY.md` says the same in one sentence. The warning on writable folders is not in this ticket.

### Drift is a measurement

The builder measures drift over three recording sets with no network.

- The repository set is every tracked `thinkthen.recording/1` entry outside `site/`, read after ticket 0159 lands. `site/` is excluded because its Beatles recordings copy the Beatles Bench set and its re-key belongs to the marketing lead.
- Ticket 0159 re-keys `crates/thinkthen/tests`, `demos/` and `transforms/rows` onto `jev-1.13.0`. It keeps the 879 entries in `probes/01` to `probes/09` and the entries in `specification/fixtures/systemone` on `jev-latest` keys (0159 lines 79 to 81, and decision 3). So the repository set mixes keys that asked for `jev-latest` with keys that ask for `jev-1.13.0`.
- The measurement compares entries only within one key. A repeat counts only when both entries hold the same digest. A question recorded both in a probe and in a re-keyed demo has two different digests, so it does not count as a repeat.
- The record reports the probes in `probes/01` to `probes/09` as their own set. It reports the rest of the repository set beside them, and the Beatles Bench recordings as a third set.
- The Beatles Bench set is the Beatles Bench recordings. They live in the separate Beatles Bench Git repository, which `site/scripts/pull-bench.mjs` reads from a local clone. The builder reads that repository at the commit `site/examples/beatles/BENCH` names, with `git archive` or `git show` at that commit. It does not check out, fetch or change the clone.

The measurement compares answer probabilities only. It never compares usage, models, or any other field. These terms hold in the record and on the pages:

- An entry's probabilities are, for each question in `response.answers`, the `noul` value of a `noul` answer and every value in the `probabilities` map of a `choice` or `score` answer. The `confidence`, `score`, `choice` and `legend` fields are not probabilities. A question and label pair names one probability.
- A digest is repeated when two or more entries in different folders of one set hold it.
- The gap of a repeated digest is the largest absolute difference between the two values of any one question and label pair, taken over every pair that both entries hold and over every two entries of that digest. A pair that only one entry holds is not compared.
- A repeated digest differs when its gap is above 0.
- A repeated digest crosses 0.5 when some question and label pair holds a value below 0.5 in one entry and a value of 0.5 or more in another. This is the side rule of `threshold.md` line 15.

For each set, the record counts repeated digests, those that differ, those whose gap is above 0.1, and those that cross 0.5, and it gives the largest gap. Report 09 did not state its rule, so the new counts can differ from its 4,075, 263 and 0.45. The numbers go into `sdlc/records/0163-answer-drift.md`. That record names the Beatles Bench commit, the repository commit it read, and keeps the short script it ran.

- `recording.md` line 78 keeps experiment 212's sentence, replaces the experiment 259 sentence with the new counts, and cites `sdlc/records/0163-answer-drift.md`. It adds: "A borderline answer can cross the cut from one call to the next. The not-sure band in [threshold.md](threshold.md) marks where a second look pays."
- `threshold.md` line 45 changes in four places. Report 09 contradicts two of its sentences, because 263 repeated digests moved by more than 0.1 and the largest gap was 0.45.
  - "The movement behind a flip is small but real." becomes "Repeated calls can move an answer far enough to cross a cut."
  - Experiment 212's sentence stays.
  - "Experiment 259 saw gaps up to 0.09 on the same model." becomes one sentence that cites `sdlc/records/0163-answer-drift.md` and gives its largest gap and its count of digests that cross 0.5 for each set.
  - "A band narrower than about 0.1 on each side of a cut does not keep a flip out." becomes "Size a band from the gaps that record reports." The page writes no fixed width in advance, because the record sets the sizing.
- ADR 0010 gains a dated amendment that points to `sdlc/records/0163-answer-drift.md`. The 2026-09-25 amendment stays as history.

### The exit waits for the usage lock

`recording.md` line 26, after "Counting never holds back a request", adds: "The command's exit waits for its usage write, and that write waits as long as another process holds the usage lock. ADR 0049 item 3 accepts the wait."

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **Prune and status skip a bad entry.** They do not move it aside, because moving is a write the user did not ask for. Prune names each one. `status` counts them.
2. **Prune exits 0 when it skipped an entry.** It trimmed what it could, and a nightly job should not fail while the cache grows. The standard error lines name each skipped entry.
3. **The SQL defaults stay as they are, and no off switch is added.** The cache stays on for the three SQL extensions. Two of them, DuckDB and PostgreSQL, cannot turn it off today, and neither can the C library. The coordinator ruled that this ticket adds no switch. Each README says what the cache holds and how to move it. Ticket 0149 carries `off` for DuckDB and PostgreSQL.
4. **The trust rule is documentation only.** The writable-folder warning waits, by the issue's own placement.
5. **Drift is measured over the recordings already on disk.** No paid call.
6. **A vanished file is not an entry, and an unopenable one is bad.** A file that `read_dir` listed and that is gone at open raced with a writer or another prune. A file that is there and cannot be opened is one the user must look at.

## Edge cases

Every prune row runs with `--max-size 1` unless it names other options. "Good" means an entry the Rust recorder wrote.

| Input | Expected |
| --- | --- |
| A current entry, an old-model entry, and a bad entry. Every request asks for `jev-latest`. The current reply names `jev-1.13.0` and the old reply names `jev-old`. The bad entry is a valid recording whose question holds a private marker and whose reply names `jev-latest`, renamed so its name does not match its digest. Prune runs with `--answered-by-other-than jev-1.13.0` and no `--max-size`. A second prune then runs on the same folder with `--answered-by-other-than jev-latest` | First prune: the old entry is removed. The current entry and the bad entry stay. Standard output is the whole success line. Standard error is exactly one bad-entry line naming the bad entry. Exit 0. Second prune: no good reply names `jev-latest`, so the alias check refuses with exit 2 and the alias message, and it names no bad entry. Both files stay. No output of either prune carries the marker |
| One good entry and a bad entry whose name does not match its digest | The good entry is removed. The bad one stays. Standard output is `removed 1 entries and B bytes; 0 entries and 0 bytes remain`. Standard error is exactly one bad-entry line. Exit 0 |
| The same with an entry whose response names a blank model | The same |
| The same with a digest-named directory | The same |
| The same with an entry naming `thinkthen.recording/2` | The same |
| The same with a digest-named file of mode `0000`, on Unix | The same, and the file keeps its mode |
| The same with a digest-named symlink to a good entry outside the folder. The link's name is that entry's own digest name | The symlink stays and is named. The target keeps its bytes. Standard output is `removed 1 entries and B bytes; 0 entries and 0 bytes remain`, where B is the in-folder entry's bytes. Exit 0 |
| `status` with `THINKTHEN_CACHE` naming the folder of any of rows 2 to 7, before prune | `cache_entries 1` and `cache_bad_entries 1`. Exit 0 |
| `status --json` over a platform cache holding only a digest-named symlink | Exit 0. `cache.entries` is 0 and `cache.bad_entries` is 1. No output carries the target's bytes |
| `status` over a folder with no bad entry | Output as today, with `cache_bad_entries 0` after `cache_bytes` |
| `status` with no absolute home | `cache.bad_entries` is `null` |
| A digest-named file that vanishes between `read_dir` and open | Skipped as not an entry. Neither counted nor named |

Row 1 plants all three entries with `plant_recording` and the literal `jev-latest` in each request. It does not use the `plant` helper or the tests' `DEFAULT_MODEL` constant. Ticket 0159 changes that constant to `jev-1.13.0`, and the `plant` helper (`default_cache.rs:29-35`) builds its request from it. The alias check refuses only a model that some good request asked for, so a request built from the constant would let the second prune remove the current entry. Ticket 0159 line 82 makes the same change for `prune_refuses_the_alias_and_keeps_the_upgrade`.

Row 7's link must pass every check if a scan follows it. So the test plants a good entry with `plant_recording` in a sibling folder, `folder.with_extension("outside")`, with a question that differs from the in-folder entry's. It names the link with the digest name that `plant_recording` returned for that outside entry. It records the outside entry's bytes before prune and checks after prune that they are unchanged. It removes the sibling folder at the end.

## Proof

The existing bad-entry tests were moved into a private child module under `tests/backend/default_cache/`, reusing the parent helpers. The old unsafe status case failed red at exit 5 against the accepted exit 0 before the code change. The cases now exercise the CLI boundary with exact diagnostics and status shapes.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `tests/backend/default_cache/prune.rs`, `prune_model_selection_and_scan_before_delete_hold`, renamed to `prune_selects_by_model_and_names_a_bad_entry` | Row 1 | (a) Stop at the bad entry as today: exit 5 and the old entry stays. (b) Delete the bad entry: the file is gone. (c) Let the alias check read the bad entry: its reply names `jev-latest`, so the second prune exits 0 and removes the current entry |
| `tests/backend/default_cache/prune.rs`, `prune_refuses_digest_mismatch_blank_model_and_nonregular_entries_before_deletion`, renamed to `prune_leaves_and_names_each_kind_of_bad_entry` | Rows 2 to 6, and row 8 over those five folders. It gains the other-schema and mode `0000` cases and a `status` run before each prune | (a) again. (d) Fail the whole scan when one file cannot be opened: the mode `0000` case exits 5. (e) Count a bad entry as good: `status` prints `cache_entries 2`. (f) Count a bad entry's bytes: the remaining bytes are not 0 |
| `tests/backend/default_cache/prune.rs`, `prune_refuses_a_digest_shaped_symlink_without_following_it`, renamed to `prune_names_a_digest_shaped_symlink_without_following_it` | Row 7, and row 8 over its folder. It gains a `status` run before prune | (g) Follow the symlink as a regular file: replace both `symlink_metadata` calls in `scan`, the first check at `cache_prune.rs:208` and the check after the read at `:224-225`, with `fs::metadata`. The link then passes every check, so `status` prints `cache_entries 2` and `cache_bad_entries 0`, and prune prints `removed 2 entries and B bytes; 0 entries and 0 bytes remain`, writes no bad-entry line, and removes the link. The correct build prints `cache_entries 1` and `cache_bad_entries 1`, and its prune removes only the in-folder entry, leaves the link in place, and writes exactly one bad-entry line naming it. Under both builds the outside entry keeps its bytes, because `remove_file` at `cache_prune.rs:149` removes the link and not its target. (e) again: `status` prints `cache_entries 2` |
| `tests/status.rs:174`, `an_unsafe_cache_entry_uses_the_status_failure_without_leaking_local_bytes`, renamed to `an_unsafe_cache_entry_is_counted_without_leaking_local_bytes` | Row 9 | (h) Fail status as today: exit 5 |
| `tests/status.rs:22` and `:67`, the exact-shape pins | Rows 10 and 11. In the `:22` test, the JSON pin at line 43 and the human pin at line 53 gain the new field in place. The `:67` test gains `assert_eq!(value["cache"]["bad_entries"], serde_json::Value::Null);` beside its single-field asserts at lines 76 to 81. It also runs the human `status` with the same relative home and asserts that standard output holds the line `cache_bad_entries unavailable` | (i) Omit the field or print it in another place: the whole-output pin differs. (j) Print 0 in place of `null` with no cache: the `:67` JSON assert differs. (k) Print 0 in place of `unavailable` with no cache: the `:67` human assert differs |

Row 12 has no test. A race between `read_dir` and open needs a test-only hook to plant, so review checks the not-found match in `scan` by reading it.

The page changes carry no test of their own. No `mustmatch` page holds `status` output, so no page example changes with the new field. `recording.md` line 20, which lists what `status` reports, adds the count of bad cache entries after the entry count and allocated bytes. Each new number names its record.

The four questions, for the changed tests:

- **What behavior does it protect?** Prune as the one bound on the cache, and `status` as a read-only report, with one bad file present.
- **What credible regression fails it?** A scan that stops at the first bad file again, a prune that deletes or follows a file it could not read, a bad file counted as good, and one unopenable file failing the whole folder.
- **Why does no existing test catch it?** Existing tests do catch the change. `default_cache/prune.rs` cases for model selection, bad-entry kinds and symlink handling and `status.rs:174` plant bad entries and pin the whole-folder refusal, so all four go red. This ticket rewrites them to pin the new rule rather than adding a new file. No existing test plants another schema or an unopenable file, so those become new rows in the `:349` table.
- **Does it need a test-only hook?** No. The folders, files, modes and symlinks are real.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `engine/cache_prune.rs`: measured +37 nonblank lines against the old engine estimate of 40. `engine/error.rs` stayed with ticket 0170 and was untouched.
- `cli/status.rs` and `cli/cache.rs`: measured +11 against the old estimate of 20.
- `tests/backend/default_cache.rs` and its private `prune.rs` child: measured +146 together against the old 45 estimate; each file stays under 500 nonblank lines. `tests/status.rs`: +5 against 10.
- Pages, READMEs, `SECURITY.md` and the ADR amendment: measured +15 nonblank lines against the old 75 estimate, separate from Rust source; see `sdlc/records/0163-build.md`.
- `sdlc/ratchet.json` equals the measured 77,849 after the code slice, +199 over its previous 77,650. Product +48 and tests +151 are separate. The coordinator authorized a measured amendment for necessary distinct behavior; fresh review judges it.
- No dependency. No paid call.

## Stop rules

1. A necessary budget overrun receives an exact group, per-file and aggregate measurement and a duplication review before fresh code review. A dependency still requires coordinator routing.
2. Stop if prune would delete, open again or follow an entry it set aside.
3. Stop if a printed line would carry anything from inside an entry.
4. Stop if the drift measurement needs a network call, or if the local Beatles Bench clone lacks the commit `site/examples/beatles/BENCH` names. Record what was measured and report.
5. Stop if any plant stays green.
6. Stop if ticket 0148, 0155, 0158 or 0159 has not landed.
7. Never run `sdlc/scripts/live`.

## Build order

It builds after tickets 0148, 0155, 0158 and 0159 land. 0148 opens every library and database README, 0155 opens `cli/status.rs`, `recording.md` and the database READMEs, and 0158 rewrites `recording.md` lines 59 and 84. 0159 re-keys every recording a gate replays, and its stop rule 7 forbids another build beside it (backlog line 72). It merges with ticket H4 on the settings table's "Answer cache" row, in the order the coordinator sets. It may build beside ticket 0162.

## Scope and exclusions

Excluded: a new off switch on any surface, a warning or refusal for a writable folder, an integrity check on entries, turning the SQL caches off by default, an expiry setting, the dot-prefixed partials that review 11 found, the "never recorded" sentence, and `site/`.

## Routing

Builder: Codex Sol Medium in codex-5. Reviewer: a fresh independent Codex session for the frozen final code and documentation diff.

## Complexity

Contract 1; state and timing 1; reach 2; proof 1; cost of error 2; total 7. Final level: 2. The risk is a prune that touches a file it could not trust, which rows 1, 6 and 7 guard.

## Deferred gaps

- The writable-folder warning and a keyed check on entries. `sdlc/issues/2026-09-26-folder-writers-decide-the-answers.md` keeps them.
- No off switch on DuckDB, PostgreSQL or C. Ticket 0149 carries `off` for DuckDB and PostgreSQL, as `sdlc/issues/2026-09-26-follow-on-tickets-0149-and-0157-carry.md` lists. The C library's switch has no owner yet. `sdlc/issues/2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so.md` keeps it.
- Turning the cache off by default for the SQL extensions, and an expiry setting. `sdlc/issues/2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so.md` keeps them.
- `Engine::builder()` ignoring the configuration file's `cache: false`, in the severity 3 roll-up.
- A test for a file that vanishes during the scan. It needs a test-only hook.

## What Ian can overturn

- Decision 2: prune exits 0 when it skipped an entry.
- Decision 3: the SQL caches stay on by default for 0.1, and no surface gains an off switch here. This is the coordinator's ruling.
- Decision 4: the folder trust rule ships as documentation only.
- Decision 6: an unopenable file is a bad entry, and a vanished one is not an entry.

## Closes

`sdlc/issues/2026-09-26-one-bad-cache-entry-breaks-status-and-prune.md`, `sdlc/issues/2026-09-26-the-spec-understates-answer-drift.md` and `sdlc/issues/2026-09-26-recording-page-omits-the-exit-wait-on-the-usage-lock.md`. The documentation items of `sdlc/issues/2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so.md` and `sdlc/issues/2026-09-26-folder-writers-decide-the-answers.md`, which then keep only their deferred items. Item 3 of `sdlc/issues/2026-09-26-architect-review-08-cache.md`. Ticket 0158 closes `sdlc/issues/2026-09-26-recording-page-says-a-failure-is-never-recorded.md`.

## Evidence

- Starts from: Local experiment 273, report 08 finding 3, report 12 findings 2.1 and 2.2, report 09 finding 3, and report 07 finding I2, as the five issues record them. The starting code at `origin/main` `8084d38a`: `cache_prune.rs:66`, `:92`, `:200-240`, `:208` and `:212`, `core/recording.rs:212`, `cli/status.rs:123-129`, `recorder.rs:137`, `usage.rs:263`, `databases/duckdb/src/engines.rs:129`, and `databases/postgresql/src/call.rs:127` and `:147`. The tests `default_cache/prune.rs` cases for model selection, bad-entry kinds and symlink handling and `status.rs:22`, `:67` and `:174`. `recording.md` lines 20, 26, 62, 78, 92, 94 and 96, `threshold.md` line 45, `settings.md` line 63, ADR 0010's 2026-09-25 amendment, ADR 0049 item 3, and `README.md` line 38. Ticket 0158's commit `c09e9551` and ticket 0159's re-key.
- Keeps: Prune's order, target and alias rule over good entries. The success line's wording. `status` over a folder with no bad entry, beside the new count. The whole-command failure for an unreadable folder, a symlinked folder or a lock failure. Every default. Every entry's bytes.
- Changes: Prune skips and names a bad entry. `status` counts bad entries. One unopenable file no longer fails the folder. Pages name what the cache holds, how to turn it off or move it, who decides its answers, the measured drift, and the exit wait.
- Proof: The affected `default_cache` and `status` CLI tests, including a private prune child module; exact source and checks in `sdlc/records/0163-build.md`; offline measurement in `sdlc/records/0163-answer-drift.md`. Full integration belongs to the related-ticket batch checkpoint.
- Defers: The writable-folder warning, an entry integrity check, an off switch on DuckDB, PostgreSQL and C, SQL cache defaults, an expiry setting, the builder's `cache: false` gap, and a test for a vanished file.

## What the build taught us

The preflight saved work by identifying the held `engine/error.rs`, the literal `jev-latest` alias fixture, the current status shape and the benchmark pin. It missed the parent test file's remaining headroom and initially allowed schema and unreadable fixtures to change their digest names too. The corrected table retains each valid request digest and changes only the tested property; the blank-model case reports its actual name. The first source estimate undercounted the distinct outside-in regression rows, so the coordinator authorized the measured +199 aggregate amendment while retaining the 500-line file cap. Related prune cases moved into one private child using parent helpers. The next prep should measure current file headroom and isolate each intended failure before handing a builder a budget or fixture recipe.
