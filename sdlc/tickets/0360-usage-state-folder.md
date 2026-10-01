# 0360: Usage totals live in the state folder, and an unreadable count is never dropped quietly

Status: in progress. Lane claude-2. Branch `ticket/0360-usage-state-folder`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Asked by Ian on 2026-09-30 after a QA finding on a published checkpoint build.

## Outcome

- The count-only usage totals live in the platform state folder, under one folder named after the program:
  - Linux: `$XDG_STATE_HOME/thinkthen`, then `$HOME/.local/state/thinkthen`.
  - macOS: `$HOME/Library/Application Support/thinkthen/usage`.
  - Clearing the cache folder no longer resets them. The answer cache stays at `$XDG_CACHE_HOME/thinkthen` (`~/.cache/thinkthen`), or `~/Library/Caches/thinkthen` on macOS.
- One private `YYYY-MM.json` file per month holds all five counts: requests sent, retries, input tokens, output tokens and cache answers. The `retries-YYYY-MM.json` sidecar is gone, so the two-file conflict cannot happen.
- Before a process's first send, its counters read the usage totals once. This covers the command, every library engine built from the environment, the C door and the three SQL extensions. If the usage folder exists and cannot be read, the send is refused. The command exits 5, and the libraries return a local error. The error carries one of these exact sentences, where NAME is the generated file name (`2026-09.json` or `.lock`) or `the usage folder`:
  - `cannot read the usage totals: NAME has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again.`
  - `cannot read the usage totals: NAME has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it out of the usage folder that thinkthen status names.`
- Plans, dry runs, replay and runs answered only from the cache send nothing, so they never read the totals and never refuse.
- The read waits at most one second for another process's usage lock. If the lock stays busy, the check passes, because a busy lock says nothing about whether the files can be read. The writer keeps its own deadline and warning.
- A usage folder without a `.lock` reads as zero counts. The writer makes `.lock` before any month file, so such a folder holds no count that thinkthen wrote. Two runs that start together on a new folder no longer refuse each other.
- `thinkthen status` no longer fails on an unreadable count. It prints the whole report with the month and total counts as `unavailable` (JSON `null`) and exits 0. On standard error it writes the same kind of sentence with the full path: `thinkthen: cannot read the usage totals: PATH has invalid contents. Move it aside, and counting starts again.` (or the unsafe form). A busy lock gives `thinkthen: cannot read the usage totals: PATH is locked by another process. Try again when it finishes.`
- Old counts are not carried over. No code reads `thinkthen-usage`. Version 0.1 has no users yet. A person who wants this month's count kept can move `YYYY-MM.json` and `.lock` by hand into the new folder. The month file reads as before, with zero retries when it holds none.

## Evidence

- Starts from: main `111fd80ce`.
  - `crates/thinkthen/src/config.rs` `resolve_usage` puts usage under the cache home: `$XDG_CACHE_HOME/thinkthen-usage`, `$HOME/.cache/thinkthen-usage`, and `$HOME/Library/Caches/thinkthen-usage`. The repo has no directory crate. `config.rs` resolves the configuration, cache and usage paths by hand (ADR 0033), and its `Platform` has only Linux and macOS. Usage has no variable of its own. `THINKTHEN_CACHE` never moves it (`specification/recording.md`).
  - `engine/usage/storage.rs` keeps `YYYY-MM.json` in the old four-counter shape and retries in `retries-YYYY-MM.json`. The split served older binaries (tickets 0235 and 0248, closed issue `2026-09-28-a-newer-usage-file-breaks-an-older-builds-status.md`). `scan` refuses a month file that holds `retries` beside a sidecar with another value ("retry totals differ", `storage.rs:226-231`). It also refuses a sidecar with no month file. An older build wrote retries into the month file, and a newer one wrote the sidecar. QA hit exactly this conflict.
  - Root cause of the visible failures:
    - `cli/status.rs:126` maps any read failure to `Failure::StatusUsage`, exit 5, so status printed nothing else.
    - The usage writer runs in the background. Its first failure sets `failed`, and the process writes nothing after it (`engine/usage.rs` `Queue`).
    - The command prints one fixed warning at exit (`cli/mod.rs:111`).
    - `Engine::finish_usage` discards the result (`engine/facade/finish.rs`). So every library, the C door, the data frames and the SQL extensions stopped counting without a word.
  - `read` treats a missing `.lock` as a read failure (`storage.rs:122`), and it takes the shared lock with no deadline (`storage.rs:123`).
  - `Counters::prepare_attempt` (`engine/usage/attempt.rs`) runs before every send's final stop check. It already starts the writer and returns an engine error, so the check fits there.
  - Premise check: no limit reads the durable totals. Only `status` and tests call `usage::read`. The process request total and the estimated input admission total count in memory, per process (`specification/settings.md`, rows "Process request total" and "Estimated input admission total"). A lost count misleads `status` and the person reading it. It weakens no limit today. A future monthly limit would read these files, so it would need the refusal below.
- Keeps: the counting rules and the safety of the files.
  - Every count's meaning, the `.lock` serialization, the private modes (folder 0700, files 0600) and the write, sync and rename order.
  - The writer's one-second finish deadline, `THINKTHEN_CACHE` never moving usage, and `--no-cache` never disabling it. `EngineBuilder::new()` keeps counting in memory.
  - The fixed exit warning for a write that fails after a good start.
  - The fork-safe counters. A forked child's fresh counters check again before its first send.
  - Retained regressions: the lock, interrupt flush, fork, overflow, symlink, mode and malformed-month cases keep their subjects. Only their paths and outcomes change. The record names each deleted sidecar test and the regression that still covers its subject.
- Changes: the paths, one file, one check before the first send, and status.
  - `config.rs`: `usage_path` reads `XDG_STATE_HOME` and `HOME` as above. Its edge table covers Linux with and without `XDG_STATE_HOME`, a relative or blank home, macOS, and `XDG_CACHE_HOME` no longer moving usage.
  - `engine/usage/storage.rs` and `counts.rs`:
    - One month file in the `thinkthen.usage/1` shape with all five counters always written. A file without `retries` reads as zero, as it does on main.
    - Remove the sidecar, the contamination migration, `LegacyRow`, the `RetryWrite` stage and their tests (`engine/usage/tests/sidecar_optimization.rs` and the sidecar cases in `engine/usage/tests.rs` and `tests/backend/default_cache/usage.rs`).
    - A `retries-*.json` file is ignored like any other name that is not a month.
    - `read` takes the shared lock with a one-second bounded try. A missing `.lock` reads as zero counts.
  - `engine/usage`: one helper builds the sentence from the read failure, with the full path only when status asks for it.
  - `Counters` checks once, through a once-only cell, in `prepare_attempt` when it has a folder. The check runs before `prepare_attempt` takes the queue lock. The cell keeps its result, so after a failure every later send in that process refuses too, and a busy-lock pass stays a pass. A failure returns a new local engine error that carries the sentence. The command and the public `Error` pass it through. The check runs after the send permit and stop check, so a Ctrl-C during it can wait up to one second. That wait is accepted.
  - `cli/status.rs`: an unreadable count gives `unavailable` counts, the sentence on standard error, and exit 0. `Failure::StatusUsage` goes.
  - `sdlc/scripts/scratch.sh` `usage_home`:
    - On Linux, move `XDG_STATE_HOME` to a scratch copy that links every entry except `thinkthen`.
    - On macOS, copy `Library/Application Support` the same way, skipping `thinkthen`. Test runs then no longer see the real `config.json`, which is better isolation.
    - The decoy guard then watches the new folder.
  - Tests and binding checks that name `thinkthen-usage` or count through `XDG_CACHE_HOME` move to the state folder: the crate's backend and library tests, the C door, Python, Ruby, TypeScript, SQLite, DuckDB and PostgreSQL. A test that plants a bad usage file uses its own `HOME` and `XDG_STATE_HOME`. The other builder's binding-test files are touched only where a usage path forces it.
  - Docs:
    - `specification/recording.md` gives the new paths, the single file, and the refusal before the first send.
    - `specification/settings.md` names `XDG_STATE_HOME`.
    - `README.md` "Usage counts" gives the folder.
    - The repo `CLAUDE.md` boundary line names the state folder.
    - ADR 0034 and ADR 0113 get an amended-by note.
- Proof: an edge table, one regression for each failing path, and two outside-in cases.
  - The `config.rs` edge table above.
  - Regressions, each failing on main:
    - Status on a malformed month file gives exit 0, both count groups `unavailable`, and the pinned standard-error sentence. Main gave exit 5 and no report.
    - Status on QA's case gives exit 0 and reads the month file's own retries. The case is a month file with `retries` beside a `retries-` file with another value. Main gave exit 5 with "retry totals differ".
    - A command run over several records with `--jobs` above 1 and a malformed month file exits 5 with the pinned sentence. The loopback listener counts zero requests, which shows the refusal ends the whole run and does not become a failed row. Main exited 0 and stopped counting.
    - The same run under `--plan` exits 0 and sends nothing.
    - `EngineBuilder::from_env()` with a malformed month file returns a local error with the sentence on its first call and again on a second call, and sends nothing. Main answered, and its counts were lost silently.
    - A C door call with a malformed month file returns the local status code with the sentence, and the loopback listener counts zero requests.
    - A month file with mode 0644 gives the unsafe sentence on a run and on status.
    - A folder with no `.lock` reads as zero on status. Main gave exit 5.
    - With another process holding the lock, status gives the busy sentence and exit 0 within about a second. Main waited until the lock was released.
  - Outside in:
    - A run writes exactly one `YYYY-MM.json` with five counters, and no `retries-` file.
    - After the cache home is removed, status still shows the run's counts.
  - Checks: focused crate tests (config, usage, status, backend usage, library env), the C door tests, the touched binding and SQL checks, workspace clippy with `-D warnings`, `policy.py`, `sdlc/scripts/tickets`, and `lint` in a clean checkout. The coordinator runs the full surfaces sweep.
- Defers: carrying old counts, quiet loss after a good start, the site, and Windows.
  - Old counts are not carried over, and nothing reads `thinkthen-usage`. QA and the bench are told by inbox message.
  - A write that fails after a good check still stops counting quietly on the libraries and SQL extensions. Examples: a full disk, permissions changed mid-run, or another program writing a bad file. A cache answer that comes before any send is also not checked. Filed as a debt issue with `Pay when:` a binding gains a warning channel or a monthly limit lands.
  - `site/` pages and example outputs that show `thinkthen-usage` belong to marketing. They are noted in the debt issue for marketing to update.
  - Windows has no build. If one is added, the equivalent is `%LOCALAPPDATA%\thinkthen\usage`.
  - No lint checks that tests setting `XDG_CACHE_HOME` also set `XDG_STATE_HOME`. The gates run under `usage_home` with its decoy guard, which catches a leak into a shared folder.

## Decisions

Ian can overturn each.

1. **State, not cache.** The XDG Base Directory specification defines `XDG_STATE_HOME` for data that should persist between runs but is not portable or important enough for the data home. A spend count fits that definition, and a cache is safe to delete.
   - macOS has no state folder. Its `Library/Caches` may be purged by the system. `Library/Application Support` holds files an application makes that must persist.
   - On macOS the usage folder sits one level down, in `thinkthen/usage`. `Application Support/thinkthen` also holds the read-only configuration file, often in a folder the person made at mode 0755. The usage reader needs a private 0700 folder of its own. thinkthen creates `Application Support/thinkthen` only when it is absent, as the parent of `usage`, and never changes its mode or the configuration.
   - On Linux the state home holds no configuration, so `$XDG_STATE_HOME/thinkthen` is the usage folder itself, as Ian asked.
   - No crate is added. The `dirs` and `directories` crates give no state folder on macOS or Windows, so they would not settle the choice, and `config.rs` already resolves the other two paths by hand.
2. **One file per month.** The sidecar existed only so older binaries could read newer files. Version 0.1 has no users. One file removes the conflict QA hit and the migration code with its tests.
3. **Refuse before the first send, report in status.**
   - Each surface already has an error path, so the check sits in one place and every surface says the same sentence. A warning would need a new channel in every binding, since the libraries print nothing.
   - The check sits before the first send, so work that sends nothing is never refused over a count file.
   - The check costs one small read for each process's first send. The fix is one move.
   - This amends the contract's "counting never holds back a request" for this one case. The alternative is to keep running and warn, which needs that warning channel first.
4. **Generated names in errors, full path in status.** Engine errors keep the recorded rule of a generated file name and a fixed category. A SQL client is a different party from the server's operating-system user, so a refusal it sees names no path (ticket 0318). `status` runs as the folder's owner and already prints `usage_path`, so its sentence names the full path.

## What the build taught us

