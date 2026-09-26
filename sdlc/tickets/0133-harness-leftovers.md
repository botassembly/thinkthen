---
flow: build
priority: 133
opens: sdlc/scripts/heavy-lock sdlc/scripts/lint sdlc/scripts/README.md sdlc/scripts/children crates/thinkthen/src/cli/interrupt.rs crates/thinkthen/src/cli/interrupt/tests.rs crates/thinkthen/tests/backend/interrupt.rs crates/thinkthen/tests/relate_edge.rs crates/thinkthen/src/core/recording.rs databases/postgresql/src/files.rs databases/duckdb/tools/source_checks.py sdlc/ratchet.json databases/postgresql/ratchet.json databases/duckdb/ratchet.py.json sdlc/records sdlc/tickets sdlc/issues
---

# 0133: Keep the rungs' environment to an allow list and clear three harness leftovers

Status: ready. Owner: Claude.

Lane: thinkthen-lane-1

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A heavy rung and every process it starts see only the environment names the ladder needs. A developer's unrelated service key never reaches `cargo`, a test binary, or a surface check. A shared `CARGO_TARGET_DIR` no longer turns three surfaces red. The release binary holds no switch that fails the Ctrl-C handler on purpose. The children check holds no pending entry for a landed ticket.

The authority is the coordinator's brief for 0133 and the backlog `sdlc/planning/issue-backlog-2026-09-25.md`. The ticket settles `sdlc/issues/2026-09-25-the-heavy-rungs-still-pass-secret-shaped-names-to-cargo.md` whole. It settles items 6, 11, 12, and 13 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. This is the lane trial's first ticket under `sdlc/planning/worktrees.md`. Its build record gives each rung's wall time and the lane's `du -sh` afterward.

## Prior evidence

- Ticket 0127 made each test child build its environment from nothing. `heavy-lock` unsets every stray `THINKTHEN_*` name before a heavy rung, and the `lint` rung pins that unset under `env -i`. 0127 deferred the allow list for the rung itself, and its landing filed the heavy-rungs issue.
- On the Beelink, `/bin/sh` is `dash`. A probe on 2026-09-26 started `sh` under `env -i` with a name that no shell variable can hold, `FAKE.SERVICE_TOKEN`. `dash` did not pass it to its child. It does export `PWD`.
- Experiment 218, wave 2, found items 11, 12, and 13 on main. With `CARGO_TARGET_DIR` absolute, the `surfaces` rung failed Ruby, DuckDB, and PostgreSQL. With it unset, all three passed.
- A probe on 2026-09-26 ran a built `thinkthen status` under `prlimit --nproc=1:` as uid 1000. It exited 70 with `thinkthen: defect: SIGINT routing could not be activated`. The same command without the limit exited 0. The carrier thread is the first thread the command starts, so the real thread-spawn failure reaches the handler's error path.
- The rung scripts and surface checks read these names from the caller: `PATH`, `HOME`, `LANG`, `XDG_RUNTIME_DIR` for the lock path, `XDG_CACHE_HOME` for the Python toolchain cache, `SQLITE_AMALGAMATION`, `THINKTHEN_TOOLCHAINS`, `THINKTHEN_DUCKDB_CLI`, and the two lock names. `rustup` and `cargo` read `CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, and `RUSTC_WRAPPER`. A grep of every `.sh` file under the rung's reach found no other caller name that a rung needs.

## Part 1: the heavy rungs keep an allow list (the heavy-rungs issue, and item 12)

`heavy-lock` already walks the exported names with `awk` over `ENVIRON`. It prints nothing. Today it unsets each `THINKTHEN_` name but four. After this ticket it unsets every name except the allow list:

| Group | Names |
| --- | --- |
| The shell and locale | `PATH`, `HOME`, `PWD`, `LANG`, `LC_ALL`, `TMPDIR` |
| Runtime and cache folders | `XDG_RUNTIME_DIR`, `XDG_CACHE_HOME` |
| The Rust toolchain | `CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `RUSTC_WRAPPER` |
| The lock | `THINKTHEN_HEAVY_LOCK`, `THINKTHEN_HEAVY_LOCK_HELD` |
| Path overrides | `THINKTHEN_TOOLCHAINS`, `THINKTHEN_DUCKDB_CLI`, `SQLITE_AMALGAMATION` |

The unset still runs before the `exec flock` branch, so the re-run rung starts clean. A nested rung and a machine with no `flock` are covered too. `cargo`, each test binary, `mustmatch`, and each `check.sh` inherit only these names.

`CARGO_TARGET_DIR` is left off the list on purpose. Each heavy rung then builds in the lane's own `target` folders, the only place `spec` and three surface checks look. This settles item 12 with no edit to any `check.sh`. It matches `sdlc/planning/worktrees.md`: a lane keeps its own build folders. A shared target folder across lanes would thrash between tickets.

A name that no shell variable can hold, such as `FAKE.SERVICE_TOKEN`, never reaches the loop under `dash`, and `dash` does not pass it on. The `lint` row pins that. On a machine whose `/bin/sh` passes such a name on, the row fails and names it.

`sdlc/scripts/README.md` rewrites the `heavy-lock` row to name the allow list and the reason `CARGO_TARGET_DIR` is absent. The header comment in `heavy-lock` says the same in two sentences.

### The check

The `lint` rung's 0127 row becomes one row that sources `heavy-lock` under `env -i`. It sets the lock as held, the kept `THINKTHEN_` names, `HOME`, `FAKE_SERVICE_API_KEY=planted`, `THINKTHEN_SENTINEL=planted`, `CARGO_TARGET_DIR=/nonexistent/target`, and `FAKE.SERVICE_TOKEN=planted`. After the source, it `exec`s `awk` to print every name the child sees, sorted on one line. The row pins that whole line. It prints the planted names only, since the child starts from `env -i`, and never the developer's environment.

### Edge cases

| Name in the caller's shell | A rung's child sees it |
| --- | --- |
| `PATH`, `HOME`, `PWD` | Yes |
| `FAKE_SERVICE_API_KEY=planted` | No |
| `THINKTHEN_SENTINEL=planted` | No, as 0127 pinned |
| `THINKTHEN_HEAVY_LOCK`, `THINKTHEN_HEAVY_LOCK_HELD`, `THINKTHEN_TOOLCHAINS`, `THINKTHEN_DUCKDB_CLI` | Yes |
| `CARGO_TARGET_DIR`, absolute or relative | No. The rung builds in `target` |
| `FAKE.SERVICE_TOKEN=planted`, which no shell variable can hold | No, under `dash` |
| A name on the allow list the caller lacks | Absent, no error |

## Part 2: the interrupt handler loses its failure switches (item 11)

`UnixRouting::start_with` takes `failures: [bool; 4]`, `carrier` takes `readiness_fails`, and `restore` takes `injected_failure`. Only tests pass `true`. All three go. `start` does the work `start_with` did, with the real calls alone.

The four paths and their boundaries:

| Path | Real boundary | Proof after this ticket |
| --- | --- | --- |
| The carrier thread cannot spawn | `pthread_create` returns `EAGAIN` under `RLIMIT_NPROC` | A new outside-in test runs the binary under `prlimit --nproc=1:` |
| Blocking SIGINT fails | `pthread_sigmask` with `SIG_BLOCK` | None. POSIX and Linux fail it only for an invalid `how` or a bad pointer, and `nix` passes neither |
| The carrier cannot unblock SIGINT | `pthread_sigmask` with `SIG_UNBLOCK` | None, for the same reason |
| Restoring the mask fails | `pthread_sigmask` with `SIG_SETMASK` | None, for the same reason |

The three unreachable paths keep their error mapping, so a failure still reports a defect and never panics. Their unit rows go. `tests.rs` still covers the success path: the mask is blocked, a worker inherits it, and cleanup restores it. The `State` tests already map each `StartError` to its defect sentence, and they stay.

### The new test

`crates/thinkthen/tests/backend/interrupt.rs` gains one Linux test. It starts `prlimit --nproc=1:` through the 0127 child helper with `PATH` alone, a scratch `HOME`, and a fake key. `prlimit` runs `thinkthen decide "Is it accepted?"` against a loopback `Listener`, with standard input closed. The test pins:

- exit code 70
- standard error exactly `thinkthen: defect: SIGINT routing could not be activated\n`
- empty standard output
- zero connections at the listener

As root, `RLIMIT_NPROC` does not bind, so the command runs and the test fails. Its failure message says the limit does not bind this user. The rungs never run as root (`sdlc/planning/worktrees.md` rule 8), and the root-container issue already records that the test rung fails as root. The landing commit adds this test to that issue.

Ticket 0132 does not open `tests/backend/interrupt.rs`. Its branch at `7bf3d4fd` touches four other files in that folder.

## Part 3: the children check clears its pending entries (item 13)

| Entry | Change |
| --- | --- |
| `crates/thinkthen/tests/relate_edge.rs`, `relate --help` | Gains `.env_clear()`. The binary needs no name to print help |
| `databases/postgresql/src/files.rs`, `mkfifo` | Gains `.env_clear()` and `PATH` read by name, as 0127's `PATH`-only sites do. The crate is its own workspace and cannot reach the Rust helper without a `#[path]` across workspaces |
| `databases/duckdb/tools/source_checks.py`, `cargo metadata` and `cargo tree` | Both pass `env=clean_env(keep=TOOLCHAIN)`, imported from `conformance/children` as `harness.py` already does. `TOOLCHAIN` names `HOME`, `CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `CARGO_TARGET_DIR`, and `RUSTC_WRAPPER`, the list `test_deadline/child.rs` names |
| `databases/duckdb/tools/databases_suite.py`, the inner `subprocess.run` | Moves to `EXEMPT`. The line runs inside a script whose process the helper already built, as the TypeScript `fork` entry does. That reason holds as long as the outer call passes `env=child_env(...)` |

`PENDING` ends empty. The table and its stale-entry rule stay for the next owner.

## Part 4: `EntryError::Unwritable` stays, with its reason (item 6)

`serde_json::to_string_pretty` over an entry of strings and raw JSON cannot fail. The call returns a `Result` anyway. Removing the variant needs an `unwrap` or an `expect`, and the crate forbids both in product code. A two-line doc note at the variant says so. This adds no test, since a comment has no behavior.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. An allow list, not a deny pattern. A deny pattern misses the next service's secret, the reason 0127 gave for its helpers.
2. The allow list lives in `heavy-lock`, the one file every heavy rung already sources, as an extension of the 0127 loop. No new script.
3. `CARGO_TARGET_DIR` is dropped, not honored. Honoring it needs the target folder resolved in ten surface checks, one of them owned by an in-flight Quick Fix. Dropping it gives each lane its own build folders, as the lane rule wants.
4. `lint` is not a heavy rung and keeps the caller's environment. The issue names the heavy rungs.
5. The spawn failure is proved at the real boundary with `prlimit`. The three `pthread_sigmask` paths lose their test, because no boundary reaches them. Their code stays.
6. The new interrupt test fails as root rather than skipping. A skip reads as a pass.
7. The DuckDB suite's inner spawn becomes an exemption with a reason, not an allow list. Its parent is already clean.
8. `Unwritable` stays, with the reason written at the variant.

## Proof

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| The `heavy-lock` row in `lint` | The child's whole sorted name line is `HOME PATH PWD THINKTHEN_DUCKDB_CLI THINKTHEN_HEAVY_LOCK THINKTHEN_HEAVY_LOCK_HELD THINKTHEN_TOOLCHAINS` | P1: put `FAKE_SERVICE_API_KEY` on the allow list. The row names it. P2: put `CARGO_TARGET_DIR` on the allow list. The row names it |
| The full ladder on the Beelink | `install`, `lint`, `test`, `spec`, and `surfaces` pass when started with `FAKE_SERVICE_API_KEY=fake-not-a-key` and an absolute `CARGO_TARGET_DIR` under the scratch folder. That folder stays empty | None. This is the toolchain proof the issue asks for on each machine that runs rungs. The Beelink is the only one |
| `interrupt.rs`: the spawn failure | Exit 70, the whole sentence, empty output, zero connections | P3: map a spawn failure to `StartError::Restoration`. The sentence differs. P4: `expect` the spawn. The command panics with exit 101 |
| `children` on the tree | Exit 0 and `children: 0 findings`, with `PENDING` empty | P5: drop the new `.env_clear()` from `relate_edge.rs`. The check names the line. P6: drop `env=` from one `source_checks.py` call. The check names it |
| `children --self-test` | Unchanged | None. The rules do not change |

The build records each plant's red run. The plants are never committed.

The four questions, for each new or changed test:

- **The `heavy-lock` row.** It protects the rule that a rung's children see only the allow list. A regression that keeps a secret-shaped name or `CARGO_TARGET_DIR` fails it. No existing test reads what a rung's child inherits beyond `THINKTHEN_` names. It needs no hook: it sources the real file and reads a real child's environment.
- **The spawn-failure test.** It protects exit 70 and the defect sentence when the carrier thread cannot start, with nothing sent. A panic, a wrong sentence, or a command that goes on without routing fails it. The switch row that covered this path goes, and no other test reaches a spawn failure. It needs no hook: `RLIMIT_NPROC` is the operating system's own limit.
- **The `children` changes.** They extend the existing check by removing entries. No new test.

## Budgets

In nonblank lines.

- `heavy-lock`: at most 10 added.
- `lint`: at most 6 added over the row it replaces.
- `interrupt.rs` and `interrupt/tests.rs`: fewer lines than today, together.
- `tests/backend/interrupt.rs`: at most 30 added.
- The spawn sites and `children`: at most 15 changed in all.
- `recording.rs`: at most 3 added.
- `sdlc/ratchet.json` and each surface ratchet move to the measured total. The Rust ceiling should fall.
- No dependency.

## Stop rules

Stop, write down what happened, and hand back before any of these:

- A rung fails on the Beelink under the allow list and the fix needs a secret-shaped name, or more than three names beyond the table.
- A fix needs a file another in-flight builder owns: 0132's `engine/http.rs`, its backend tests, `specification/backends.md`, `specification/check.md`, or `public_controls.rs`; the flaky-test Quick Fix's `libraries/c/tests/door`, `databases/postgresql/check.sh`, or `libraries/polars/tests/throttle_equality.rs`; or 0134's public library files.
- `prlimit --nproc=1:` does not fail the carrier spawn in the test, or the command starts another thread first.
- A budget would be crossed, a dependency added, or a public API or command behavior changed.

## Scope and exclusions

Excluded, from the harness issue:

- Item 1, which ticket 0119 owns.
- Item 4's high-load proof. The brief waits while the one-minute load is above 10.
- Item 5, a command behavior change. Its echo sits in `cli/failure.rs`, which 0132 opens.
- Items 7, 8, and 9, the demo standard. They touch `sdlc/scripts/demos`, its self-test, and demo pages 19 and 27. They fit one Quick Fix together.

Also excluded: any `check.sh`, any live or paid call, and the `lint` rung's own environment.

## Complexity

Contract 1; state and timing 1; reach 2; proof 2; cost of error 2; total 8. Final level: 2. The risk is a rung that needs a name the table lacks on a machine other than the Beelink.

## Evidence

- Starts from: The heavy-rungs issue and items 6, 11, 12, and 13 of the harness issue. Ticket 0127's `heavy-lock` loop, its `lint` row, and its child helpers. Experiment 218, wave 2, for items 11 to 13. The 2026-09-26 `prlimit` probe and the `dash` probe under Prior evidence.
- Keeps: Every rung's steps and every test's meaning. The lock and its two names. The 0127 helpers and the children rules. The interrupt handler's behavior and its defect sentences. `EntryError` as it is.
- Changes: A heavy rung keeps only the allow list, and `CARGO_TARGET_DIR` is dropped. The interrupt handler loses three test-only parameters, and one outside-in test reaches the spawn failure. Four spawn sites build their environment, one moves to `EXEMPT`, and `PENDING` empties. One doc note in `recording.rs`.
- Proof: The Proof table. Each new or changed test has a planted fault that turns it red, and the full ladder runs once with a planted key and an absolute target folder.
- Defers: The `lint` rung's environment. A machine whose `/bin/sh` passes an unsettable name on. The GitHub gate, which runs only by hand. A by-hand `check.sh` run with an absolute `CARGO_TARGET_DIR` still breaks Ruby, DuckDB, and PostgreSQL, and only the rung is fixed. Tests for the three `pthread_sigmask` failure paths, which no boundary reaches. Items 1, 4, 5, 7, 8, and 9 of the harness issue.

## What Ian can overturn

Each numbered decision above. The allow list's contents. Dropping `CARGO_TARGET_DIR` in place of honoring it. Losing the tests for the three unreachable `pthread_sigmask` paths. The new test failing as root.

## Issues this closes

- `sdlc/issues/2026-09-25-the-heavy-rungs-still-pass-secret-shaped-names-to-cargo.md`. It moves to `closed/` in the landing commit.
- Items 6, 11, 12, and 13 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. The landing commit moves them to "Already fixed" with this ticket's number. The issue stays open for items 1, 4, 5, 7, 8, and 9.
- The landing commit adds the spawn-failure test to `sdlc/issues/2026-09-25-two-gate-failures-in-a-root-container.md` as a third root failure.
