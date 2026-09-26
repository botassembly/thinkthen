---
flow: build
priority: 163
opens: crates/thinkthen/src/engine/cache_prune.rs crates/thinkthen/src/engine/error.rs crates/thinkthen/src/cli/status.rs crates/thinkthen/src/cli/cache.rs crates/thinkthen/tests/cache_bad_entry.rs crates/thinkthen/tests/status.rs specification/recording.md specification/threshold.md specification/settings.md sdlc/planning/adr/0010-one-wire-shape-two-variables-and-a-smaller-version-one.md SECURITY.md libraries/c/README.md libraries/polars/README.md libraries/python/README.md libraries/r/README.md libraries/ruby/README.md libraries/rust/README.md libraries/typescript/README.md databases/duckdb/README.md databases/postgresql/README.md databases/sqlite/README.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0163: The cache folder survives a bad entry, and the pages say what it holds

Status: ready for review. Written 2026-09-26 by Claude, the queue owner's planner. A fresh read-only review must accept it before it builds. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

One malformed or foreign entry no longer stops `cache prune` or `status` for a whole folder. Both carry on and name the bad entry by its file name. Every library and database README, the settings table and `SECURITY.md` say that the answer cache is on by default and that each entry holds the judged text. `specification/recording.md` and `SECURITY.md` say that whoever can write a cache or recording folder decides the answers read from it. `recording.md` and `threshold.md` state answer drift as a measurement over the recordings on disk shows it. `recording.md` says that a command's exit waits for the usage lock. `recording.md`'s two sentences on recorded failures agree, as ticket 0158 leaves them.

Six issues from local experiment 273 block 0.1 by the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. The folder-writers issue blocks 0.1 only for its documentation half. The recording-failure issue owes only its sentence, and ticket 0158 rewrites line 59, so this ticket leaves that line to 0158. Ian can overturn each design choice.

## What happens today

Read from `origin/main` `ebd28382`.

- `engine/cache_prune.rs:200-240`, `scan`, returns `Error::CacheEntry` at the first digest-named entry that is not a regular file, changes identity while read, fails to parse, or carries the wrong name. Both callers, `inspect` at line 66 and `run_at` at line 92, pass it up. `cli/status.rs:123-128` calls `inspect` and maps any error to `Failure::StatusState`, so `status` fails whole. `recording.md` line 94 says "A scan refusal deletes nothing". The messages name no file. Report 08, finding 3 (`sdlc/issues/2026-09-26-one-bad-cache-entry-breaks-status-and-prune.md`).
- No `libraries/*/README.md` or `databases/*/README.md` says that a cache entry holds the judged text. `README.md` line 38 says it for the command. The settings table's "Answer cache" row (line 63) gives the default and not the contents. In PostgreSQL the folder belongs to the server's operating-system user, and every role that can call the functions shares it. Report 12, finding 2.1 (`sdlc/issues/2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so.md`).
- `engine/recorder.rs:137` checks folder privacy only for the platform default. Entries carry no integrity check. Report 12, finding 2.2, changed `"noul": 0.93` to `0.01` in one entry, and the next run printed `false` with `requests_sent: 0`. `recording.md` line 62 treats every entry field as untrusted text for printing and says nothing about answers (`sdlc/issues/2026-09-26-folder-writers-decide-the-answers.md`).
- `recording.md` line 78 and `threshold.md` line 45 say borderline answers moved "by up to 0.08" and that experiment 259 saw gaps "up to 0.09". ADR 0010's amendment of 2026-09-25 holds the same numbers. Report 09, finding 3, counted the Beatles Bench recordings, all answered by `jev-1.13.0`: 4,075 of 5,290 repeated digests differed, 263 by more than 0.1, and the largest gap was 0.45. One digest replays as 0.21, 0.25 and 0.66 for one label, so it crosses the default cut of 0.5 (`sdlc/issues/2026-09-26-the-spec-understates-answer-drift.md`).
- `recording.md` line 26 says "Counting never holds back a request." `engine/usage.rs:263` takes the usage lock with `File::lock`, which has no timeout, and the command's exit waits for the usage writer. ADR 0049 item 3 accepts the wait. Report 07, finding I2, held the lock for 12 seconds, and `decide` printed `true` at once and exited after 11.73 seconds (`sdlc/issues/2026-09-26-recording-page-omits-the-exit-wait-on-the-usage-lock.md`).
- `recording.md` line 59 says "A failure is never recorded", and line 84 says a recorded partial reply replays with exit 6. Ticket 0158 adds a sentence after line 59 and one after line 84 (`sdlc/issues/2026-09-26-recording-page-says-a-failure-is-never-recorded.md`).

## Design

### A bad entry is skipped and named

`scan` no longer stops at a bad entry. It sets the entry aside and keeps going. A bad entry is a digest-named object that is not a regular file, changes identity while read, fails to parse, names another schema, or carries a name that does not match its digest. `scan` returns the good entries and the names of the bad ones, sorted. The file name matched the digest pattern, so it is 64 hex characters and `.json`, and printing it prints no entry text.

- `cache prune` never deletes, follows or opens a bad entry again. It trims the good entries as today. It prints the success line, then one standard error line per bad entry: ``thinkthen: cache prune: left `NAME` in place; it is not a valid entry``. It exits 0. A bad entry's bytes do not count toward the target.
- `status` counts good entries as today and adds `cache_bad_entries N`, and `status --json` adds `cache.bad_entries`. It no longer fails when an entry is bad. It still fails for an unreadable folder, as today.
- `recording.md` line 94's "A scan refusal deletes nothing" becomes the rule above, and the `status` paragraph names the new field. The addition keeps `thinkthen.status/1`, by the rule ticket 0160 writes for results.

### Every surface says where judged text goes

Each library and database README gains one paragraph under its cache setting: "The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. `cache prune` is the only thing that removes entries. Turn it off with SETTING." SETTING is that surface's own spelling from `specification/settings.md`. `databases/postgresql/README.md` adds: "The folder belongs to the server's operating-system user. Every role that can call these functions shares it, so row text leaves the database's own access control, row-level security included. Set `thinkthen.cache` to an empty folder per database, or turn the cache off, when roles must not share answers." The settings table's "Answer cache" row says "Each entry holds the judged text" in its "What it does" cell. `SECURITY.md` gains one paragraph that names the cache and its contents, and the trust rule below.

### Whoever writes a folder decides its answers

`recording.md`, after line 62, gains: "A cache or recording entry is trusted for its answer. Whoever can write a folder that `--cache`, `--record`, `--replay` or `THINKTHEN_CACHE` names decides the answers read from it, and a changed entry replays with no sign. Keep such a folder private to the people whose answers it holds. Do not restore a shared cache across a trust boundary." `SECURITY.md` says the same in one sentence. The warning on writable folders is not in this ticket.

### Drift is a measurement

The builder measures drift over two recording sets with no network: every tracked `thinkthen.recording/1` entry in this repository, and the Beatles Bench recordings at the commit `site/examples/beatles/BENCH` names. A digest counts as repeated when two entries in different folders hold it. For each set, the record counts repeated digests, those whose replies differ, those that differ by more than 0.1, the largest gap, and how many cross the default cut of 0.5. The record keeps the short script it ran.

- `recording.md` line 78 keeps experiment 212's sentence, replaces the experiment 259 sentence with the new counts and names the record, and adds: "A borderline answer can cross the cut from one call to the next. The not-sure band in [threshold.md](threshold.md) marks where a second look pays."
- `threshold.md` line 45 cites the same record in place of experiment 259's 0.09.
- ADR 0010 gains a dated amendment that points to the record. The 2026-09-25 amendment stays as history.

### The exit waits for the usage lock

`recording.md` line 26, after "Counting never holds back a request", adds: "The command's exit waits for its usage write, and that write waits as long as another process holds the usage lock. ADR 0049 item 3 accepts the wait."

### The two recorded-failure sentences agree

After ticket 0158 lands, the builder reads `recording.md` lines 59 and 84 as 0158 left them. When they agree and say how a user asks a failed question again, the issue closes with 0158's commit and this ticket changes nothing there. When they do not agree, this ticket edits line 84's paragraph and leaves line 59 as 0158 wrote it.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **Prune and status skip a bad entry and name it.** They do not move it aside, because moving is a write the user did not ask for.
2. **Prune exits 0 when it skipped an entry.** It trimmed what it could, and a nightly job should not fail while the cache grows. The standard error lines name each skipped entry.
3. **The SQL defaults stay as they are.** The cache stays on for the three SQL extensions, and each README says what that means. Turning it off by default on three surfaces is a larger change, and the issue keeps it as a later choice.
4. **The trust rule is documentation only.** The writable-folder warning waits, by the issue's own placement.
5. **Drift is measured over the recordings already on disk.** No paid call.

## Edge cases

| Input | Expected |
| --- | --- |
| A folder with 3 valid entries and one digest-named file of invalid JSON, `cache prune --max-size 0` | 3 removed. The bad file stays. One standard error line names it. Exit 0 |
| The same with an entry naming `thinkthen.recording/2` | The same |
| The same with a digest-named symlink | It is not followed or removed, and it is named. Exit 0 |
| An entry whose name does not match its digest | Named and left |
| `status` over that folder | `cache_entries 3`, `cache_bad_entries 1`. Exit 0 |
| `status --json` over it | `cache.bad_entries` is 1 |
| A folder with no bad entry | Output as today, with `cache_bad_entries 0` in `status` |
| An unreadable folder | `status` and `prune` fail as today |
| `--answered-by-other-than` over a folder with a bad entry | The alias check reads only good entries |

## Proof

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `a_bad_entry_is_named_and_left`, new in `tests/cache_bad_entry.rs` | Edge rows 1 to 4 and 9. Each row builds a real folder, runs the binary, and pins standard output, each standard error line, the exit code and the files left | (a) Stop at the bad entry as today: row 1 exits nonzero and removes nothing. (b) Delete the bad entry: row 1's file is gone. (c) Follow the symlink: row 3 reads its target |
| `status_counts_a_bad_entry`, same file | Edge rows 5 to 8 | (d) Fail status as today: row 5 exits nonzero. (e) Count the bad entry as good: row 5 prints `cache_entries 4` |

The page changes carry no test of their own. `mustmatch` pages already hold the `status` output that changes, and they update with it. Each new number names its record.

The four questions:

- **What behavior does it protect?** Prune as the one bound on the cache, and `status` as a read-only report, with one bad file present.
- **What credible regression fails it?** A scan that stops at the first bad file again, a prune that deletes or follows a file it could not read, and a bad file counted as good.
- **Why does no existing test catch it?** No prune test plants a bad entry. `tests/status.rs::an_unsafe_cache_entry_uses_the_status_failure_without_leaking_local_bytes` pins today's whole-status failure on a symlink entry. This ticket changes that rule, so that test changes to pin exit 0, `cache_bad_entries 1` and the untouched target, and it keeps its check that no local bytes leak.
- **Does it need a test-only hook?** No. The folder, the files and the symlink are real.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `engine/cache_prune.rs` and `engine/error.rs`: at most 35 net together.
- `cli/status.rs` and `cli/cache.rs`: at most 15 net together.
- `tests/cache_bad_entry.rs`: at most 170, new. `tests/status.rs`: at most 5 net.
- Pages, READMEs, `SECURITY.md` and the ADR amendment: at most 60 net.
- `sdlc/ratchet.json` moves to the measured total, at most 50 above main. The commit says what grew.
- No dependency. No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if prune would delete, open again or follow an entry it set aside.
3. Stop if a printed line would carry anything from inside an entry.
4. Stop if the drift measurement needs a network call, or if the Beatles Bench recordings at the named commit cannot be read without one. Record what was measured and report.
5. Stop if any plant stays green.
6. Stop if ticket 0148, 0155 or 0158 has not landed.
7. Never run `sdlc/scripts/live`.

## Build order

It builds after tickets 0148, 0155 and 0158 land. 0148 opens every library and database README, 0155 opens `cli/status.rs`, `recording.md` and the database READMEs, and 0158 rewrites `recording.md` line 59. It merges with ticket H4 on the settings table's "Answer cache" row, in the order the coordinator sets. It may build beside ticket 0162.

## Scope and exclusions

Excluded: a warning or refusal for a writable folder, an integrity check on entries, turning the SQL caches off by default, an expiry setting, the dot-prefixed partials that review 11 found, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 1; state and timing 1; reach 2; proof 1; cost of error 2; total 7. Final level: 2. The risk is a prune that touches a file it could not trust, which rows 1 and 3 guard.

## Deferred gaps

- The writable-folder warning and a keyed check on entries. `sdlc/issues/2026-09-26-folder-writers-decide-the-answers.md` keeps them.
- Turning the cache off by default for the SQL extensions, and an expiry setting. `sdlc/issues/2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so.md` keeps them.
- `Engine::builder()` ignoring the configuration file's `cache: false`, in the severity 3 roll-up.

## What Ian can overturn

- Decision 2: prune exits 0 when it skipped an entry.
- Decision 3: the SQL caches stay on by default for 0.1.
- Decision 4: the folder trust rule ships as documentation only.

## Closes

`sdlc/issues/2026-09-26-one-bad-cache-entry-breaks-status-and-prune.md`, `sdlc/issues/2026-09-26-the-spec-understates-answer-drift.md`, `sdlc/issues/2026-09-26-recording-page-omits-the-exit-wait-on-the-usage-lock.md`, and `sdlc/issues/2026-09-26-recording-page-says-a-failure-is-never-recorded.md`. The documentation items of `sdlc/issues/2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so.md` and `sdlc/issues/2026-09-26-folder-writers-decide-the-answers.md`, which then keep only their deferred items. Item 3 of `sdlc/issues/2026-09-26-architect-review-08-cache.md`.

## Evidence

- Starts from: Local experiment 273, report 08 finding 3, report 12 findings 2.1 and 2.2, report 09 finding 3, report 07 finding I2, and reports 03, 06, 08 and 11 on the recorded failure, as the six issues record them. The code at `origin/main` `ebd28382`: `cache_prune.rs:66`, `:92` and `:200-240`, `cli/status.rs:123-128`, `recorder.rs:137`, `usage.rs:263`. `recording.md` lines 26, 59, 62, 78, 84 and 94, `threshold.md` line 45, ADR 0010's 2026-09-25 amendment, ADR 0049 item 3, and `README.md` line 38.
- Keeps: Prune's order, target and alias rule over good entries. `status` over a folder with no bad entry, beside the new count. Every default. Every entry's bytes.
- Changes: Prune and `status` skip and name a bad entry. `status` counts bad entries. Pages name what the cache holds, who decides its answers, the measured drift, and the exit wait.
- Proof: Two outside-in tests with five plants, an offline drift record, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: The writable-folder warning, an entry integrity check, SQL cache defaults, an expiry setting, and the builder's `cache: false` gap.
