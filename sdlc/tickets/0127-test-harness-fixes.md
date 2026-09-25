---
flow: build
priority: 127
opens: conformance/backend/src conformance/backend/tests conformance/children crates/thinkthen/src/test_deadline crates/thinkthen/src/cli/conformance_tests crates/thinkthen/src/cli/interrupt crates/thinkthen/src/engine/host_signal_tests.rs crates/thinkthen/src/engine/recorder/identity crates/thinkthen/tests/backend/secrecy.rs crates/thinkthen/tests/backend/secrecy_relate.rs crates/thinkthen/tests/backend/exchange.rs crates/thinkthen/tests/backend/interrupt.rs crates/thinkthen/tests/compile_contract.rs crates/thinkthen/tests/demo_runner.rs crates/thinkthen/tests/transform.rs crates/thinkthen/tests/version.rs libraries/c/tests libraries/typescript/tests libraries/ruby/tests libraries/r/tests databases/sqlite/tests databases/sqlite/README.md databases/duckdb/tools databases/postgresql/tests sdlc/scripts/children sdlc/scripts/lint sdlc/scripts/test sdlc/ratchet.json databases/sqlite/ratchet.py.json databases/duckdb/ratchet.py.json databases/postgresql/ratchet.py.json libraries/c/ratchet.json libraries/ruby/ratchet.rb.json libraries/r/ratchet.R.json libraries/typescript/ratchet.mjs.json sdlc/issues sdlc/records sdlc/tickets
---

# 0127: Fix the test harness before the mutation audit

Status: ready. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Ticket 0119 audits the engine tests by mutation. It keeps the secrecy sweep and leans on the shared loopback backend. Both have known holes. This ticket closes them first, and it stops test children from inheriting the developer's shell.

The authority is the backlog `sdlc/planning/issue-backlog-2026-09-25.md`, row 9 and Ian's ruling 7, and the coordinator's brief for 0127. The ticket settles items 2, 3, 4, and 10 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. Step B settles the pandas branch's issue `2026-09-25-python-test-children-inherit-the-whole-shell-environment.md` after ticket 0122 lands.

The environment work answers an incident. On 2026-09-25 the 0122 build planted a fault that formatted a Python test child's environment into an error. The assertion diff printed unrelated service keys from the builder's shell. The child had inherited everything. A `THINKTHEN_*` variable left in a shell can also change what a child does, and so a test's result.

## Part 1: every test child gets an allow-listed environment

### The rule

Every test that starts a child process builds that child's whole environment. The child sees only the variables the test names. A test never copies the parent's environment and then removes names. The rule covers Rust, Python, TypeScript, Ruby, and R source, and the database surfaces' test code in those languages.

### The helper, one per language

Each helper starts a child with an empty environment, adds `PATH` from the parent, and adds the names the call site asks it to keep. It sets `THINKTHEN_*` values only from explicit arguments. It refuses to keep a name that starts with `THINKTHEN_` or holds `KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `CREDENTIAL`, or `AUTH`, in any case. The refusal is a panic or an exception with this exact sentence:

```text
a test child may not keep NAME from the parent: set a THINKTHEN_ value or a fake key explicitly
```

| Language | Helper | Used by |
| --- | --- | --- |
| Rust | `crates/thinkthen/src/test_deadline/child.rs`: `command(program, keep) -> Command` | The crate's unit tests as `crate::test_deadline::child`. Integration tests, `conformance/backend/tests`, and `libraries/c/tests` include it by `#[path]`, as they already include `run.rs` and `wait.rs` |
| Python | `conformance/children/children.py`: `child_env(keep=(), **values) -> dict` | `databases/sqlite/tests`, `databases/duckdb/tools`, `databases/postgresql/tests`, and in step B `libraries/python/tests` |
| TypeScript | `conformance/children/children.mjs`: `childEnv({ keep, values })` | `libraries/typescript/tests` |
| Ruby | `conformance/children/children.rb`: `Children.env(keep:, **values)`, always spawned with `unsetenv_others: true` | `libraries/ruby/tests` |
| R | `conformance/children/children.R`: `clean_env(keep, values)`, which returns the `env -i NAME=value ...` words a `system2` or `system` call runs its program under | `libraries/r/tests` |

`conformance/` already holds the test support every surface shares. A Rust site that needs no variable at all keeps its plain `.env_clear()`. Forty-five Rust sites already clear the environment that way, and none moves.

### The sites

A scan of `origin/main` at `08a8e754` found these children that inherit the whole environment, or copy it and remove names. Each moves onto its language's helper or gains `.env_clear()`. The build re-runs the scan and fixes any site this list missed.

| Language | Sites |
| --- | --- |
| Rust, no variable needed | `conformance/backend/tests/binary.rs:38`, `crates/thinkthen/src/cli/conformance_tests/command.rs:178`, `cli/interrupt/tests.rs:273`, `engine/recorder/identity/tests.rs:231`, `tests/version.rs:13` |
| Rust, `PATH` only | `cli/interrupt/tests.rs:264` and `tests/backend/interrupt.rs:100` (`kill`), `engine/host_signal_tests.rs:124` (`sh`), `tests/transform.rs:318` (`mkfifo`), `tests/demo_runner.rs:33` (`sh`), `libraries/c/tests/door/main.rs:85`, `:159`, and `:171` (`cc`, `readelf`, `nm`) |
| Rust, a toolchain | `tests/compile_contract.rs:173`, `libraries/c/tests/door/main.rs:42`, `libraries/c/tests/door/bytes.rs:154` (`cargo`) |
| Python | `databases/sqlite/tests/helper.py:103` `environment` copies `os.environ` less the key. Its callers keep their calls. `helper.py:56` and `databases/duckdb/tools/harness.py:37` start the loopback backend with no `env`. `databases/duckdb/tools/databases_suite.py:117` and `source_checks.py:59` and `:168` pass no `env`. `databases/postgresql/tests/examples.py:23` and `runner.py:76` start `psql` with no `env`. `databases/duckdb/tools/harness.py:85` `child_env` already builds an allow list and moves onto the helper |
| TypeScript | `libraries/typescript/tests/backend.mjs:18` starts the backend with no `env`. `backend.mjs:48` `childEnv` spreads `process.env` |
| Ruby | `libraries/ruby/tests/backend.rb:50` starts the backend with no environment. `backend.rb:128` copies `ENV.to_h` less `THINKTHEN_*`. `tests/test_public_names.rb:18` passes no environment |
| R | `libraries/r/tests/helper.R:57` and `interrupt.R:20` add names to the inherited environment. `conformance.R:28` runs `sha256sum` with it |

Each call site names what its child keeps. A child of the `thinkthen` binary keeps nothing. A toolchain child keeps the names its tool reads, such as `HOME`, `CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `CARGO_TARGET_DIR`, and `RUSTC_WRAPPER`. The SQLite children keep `LD_LIBRARY_PATH`, which `check.sh` sets to the pinned 3.50.0 host. The build measures each list by running the site, and records it.

### The check: `sdlc/scripts/children`

A new standard-library Python script scans every tracked `.rs`, `.py`, `.mjs`, `.js`, `.ts`, `.rb`, and `.R` file. It fails on a spawn that inherits, or on code that copies the whole environment. It runs from the `lint` rung twice: `children --self-test`, then `children`. It follows `sdlc/scripts/tickets` in shape. It skips comment lines.

| Language | A spawn is | It passes when | Whole-environment copies it refuses anywhere |
| --- | --- | --- | --- |
| Rust | `Command::new(` | `.env_clear()` appears in its statement, or in the later statements that begin with the name a `let` bound it to | `.envs(` over `vars()` or `vars_os()` |
| Python | `subprocess.run`, `Popen`, `call`, `check_call`, `check_output`, `asyncio.create_subprocess_exec` | its parentheses hold `env=` | `os.environ.copy(`, `dict(os.environ`, `**os.environ`, `os.environ.items(`, `env=os.environ`. `os.system`, `os.popen`, `os.exec*`, and `os.spawn*` fail outright |
| TypeScript | `spawn`, `spawnSync`, `exec`, `execSync`, `execFile`, `execFileSync`, `fork` in a file that names `child_process` | its parentheses hold `env:` | `...process.env`, `env: process.env`, `Object.assign(` over `process.env` |
| Ruby | `Open3.*`, `Process.spawn`, `spawn`, `system`, `IO.popen`, `exec` | its parentheses hold `unsetenv_others: true` | `ENV.to_h`, `ENV.to_hash`, `ENV.dup`, `ENV.clone`, `ENV.each`, `ENV.select`, `ENV.reject`, `ENV.filter` |
| R | `system(`, `system2(` | its parentheses hold `clean_env(` or the literal `env -i` | `Sys.getenv()` with no argument |

A failure reads `children: PATH:LINE: this child inherits the whole environment; build it with the LANGUAGE child helper`, or `children: PATH:LINE: this copies the whole environment into a child`.

Two tables sit at the top of the script. Each entry names a path, an owner, and a reason. The check fails on an entry whose path no longer holds a finding, so neither table can rot.

| Table | Path | Reason |
| --- | --- | --- |
| Excluded | `sdlc/scripts/` | The gate's own tools run `cargo`, `git`, and `node` in the developer's toolchain. `live` builds its own environment by design. The shell deferral below covers them |
| Excluded | `probes/`, `site/`, `databases/duckdb/vendor/`, `libraries/r/thinkthen/tools/` | Live measurements run by hand, the site build, vendored code, and the R package's install-time tools. None is a test |
| Exempt | `libraries/typescript/tests/fork.test.mjs` | The `fork` runs inside a child the helper already built, so it inherits only the allow list |
| Pending 0122 | `libraries/python/` | Ticket 0122 owns the folder until it lands. Step B clears it |
| Pending 0123 and 0126 | `crates/thinkthen/tests/relate_edge.rs` | Its one inheriting spawn runs `relate --help`. Relate and help text are both in flight |
| Pending 0129 | `databases/postgresql/src/files.rs` | Its test module runs `mkfifo`. 0129 owns `databases/*/src` |

The self-test builds one file per row of the edge-case table below in a scratch tree and pins each whole output.

### Edge cases for the check and the helpers

| Input | Expected |
| --- | --- |
| Rust `Command::new("sh").arg("-c").output()` | Refused |
| Rust `Command::new("sh").env_clear().arg("-c").output()` | Passes |
| Rust `let mut c = Command::new(x);` then `c.env_clear();` then `c.spawn()` | Passes |
| Rust `run::output(Command::new(x).arg("--version"))` | Refused |
| Rust `.env_clear().envs(std::env::vars())` | Refused |
| Python `subprocess.run(["x"], capture_output=True)` | Refused |
| Python `subprocess.run(["x"], env=child_env())` | Passes |
| Python `env = dict(os.environ)` | Refused |
| Python `os.environ.get("PATH")` | Passes |
| TypeScript `spawn(bin, [], { stdio })` | Refused |
| TypeScript `env: { ...process.env, X: '1' }` | Refused |
| Ruby `Open3.capture3(env, ruby, "-e", s)` | Refused |
| Ruby `Open3.capture3(env, ruby, "-e", s, unsetenv_others: true)` | Passes |
| R `system2("sha256sum", file, stdout = TRUE)` | Refused |
| R `system2("env", c(clean_env(), "sha256sum", file))` | Passes |
| An exclusion or pending path with no finding left | Refused as a stale entry |
| Helper `keep` of `LD_LIBRARY_PATH` set in the parent | Kept |
| Helper `keep` of a name the parent lacks | Left out, no error |
| Helper `keep` of `THINKTHEN_BASE_URL`, `OPENAI_API_KEY`, `GITHUB_TOKEN`, `db_password`, or `AWS_SECRET_ACCESS_KEY` | Refused with the exact sentence |
| Helper value `THINKTHEN_API_KEY="sk-fake"` | Set, since the test named it |

## Part 2: the shared loopback backend (items 2 and 4)

All of this lives in `conformance/backend/src/listener.rs`.

1. **A stalled peek ends in a read.** `peek_request` still peeks, so a reset can drop unread bytes. When `seen` stays the same for 2 seconds of polls, it reads the pending bytes and blocks on the next read. A zero-length read is end of file, and the serving thread returns. More bytes finish the request by reading, with `used` 0. A client that sent half a request and closed no longer keeps a thread spinning.
2. **A reset of a request that was read answers loudly.** When `used` is 0, a `Canned::reset()` reply cannot drop unread bytes, and a close would look like a clean end. The listener answers the drift status 500 instead, the fail-loud reply the arms already use. The body and a standard error line read `the loopback listener cannot reset a request it had to read`. It does not panic, because a panic in a serving thread only closes the stream, and the client again sees a plain close.
3. **A scripted listener keeps its port.** `serve_script` counts every connection in `Counts::connections`. A connection that closes before a whole request takes no scripted reply. After the script runs out, the thread keeps accepting until the test process exits. It records each extra request and answers it with the drift status and `the script has no reply left for this connection`. No other test in the same process can then bind the port while a client of this one may still connect.
4. **The secrecy sweep names what the listener saw.** The status assertion in `crates/thinkthen/tests/backend/secrecy.rs` prints the connection count and each recorded request's line and body length. It prints no body, since the body holds the evidence.

Item 4's cause stays unproven. Point 3 removes the likely cause inside one test process, and point 4 names a stray connection if one comes from anywhere else.

The existing `a_backend_that_answers_nothing_is_exit_four` in `crates/thinkthen/tests/backend/exchange.rs` relies on an empty script closing the port. Under point 3 it would test the drift reply. The refusal it pins is already pinned by `a_refused_port_fails_before_the_first_default_retry_wait` in the same file, so the build deletes it.

### Edge cases for the listener

| Client sends | Script | Old code | New code |
| --- | --- | --- | --- |
| A whole request | `ok` | 200 | 200 |
| Half a request, then closes | `ok`, then a second client sends a whole request | The thread spins on the first connection, and the second client waits forever | The first connection ends after the stall. The second gets the 200. `connections()` reads 2 and `requests()` holds one request |
| Nothing, then closes | `ok` | The listener returns and closes the port | The connection is counted and takes no reply |
| A 40 KiB request | `reset` | The client reads a clean end of file | The client reads status 500 and the sentence |
| A 1 KiB request | `reset` | The client sees a reset | The client sees a reset |
| A second whole request | `ok` only | The connection is refused | The client reads status 500 and the sentence. `connections()` reads 2 |

## Part 3: the secrecy sweep marks both relate entities (item 3)

`secrecy.rs` gains two markers beside `EVIDENCE`: `SECOND`, `marker-second-e41d09`, and `KIND`, `marker-kind-93c2f0`. Every relate input in `evidence` names the second entity `SECOND` in place of `Acme`. Every framing that carries a kind gives both entities the kind `KIND`, since `linked --either` pairs one kind with itself. `--lines` carries no kind. `nothing_leaked` refuses `EVIDENCE`, `SECOND`, and `KIND` in standard error for every case. `secrecy_relate.rs` gets the same markers in place of `Ada` and `person`, and its relation rule names `KIND`.

If the stronger check finds either marker in a diagnostic on main today, that is a real leak in relate. The build stops, files it, and leaves relate's code to ticket 0123.

## Part 4: the rusqlite workaround test (item 10, Ian's ruling 7)

Ian ruled that nobody asks the rusqlite maintainers for a fix, and that a test guards the hand-extended API table in `databases/sqlite/src/ffi.rs:21` to `:30`. Ticket 0129 owns `databases/*/src`, so this ticket commits no change there.

`databases/sqlite/tests/test_interrupt.py` already loads the extension into the pinned 3.50.0 host and interrupts held calls. The extension learns of the interrupt only through the `is_interrupted` pointer the table reads. The build plants a tail of 12 and a tail of 14 in `ffi.rs`, one at a time, runs the SQLite check, and restores the file.

- If both plants turn `test_interrupt.py` red, that file is the test. Its docstring names the table it guards, and `databases/sqlite/README.md` records the ruling and the guard in one paragraph. No second test proves the same contract.
- If either plant stays green, the build adds `databases/sqlite/tests/test_api_table.py`. It interrupts one held call on the floor host and pins that the call returns `thinkthen cancelled: the call was cancelled` within 0.1 seconds. It must turn red on both plants.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. The helpers keep a name only when the call site asks for it, and they refuse secret-shaped and `THINKTHEN_` names. A deny list alone would miss the next service's key.
2. The helpers live in `conformance/children/`, beside the shared backend, with the Rust one beside `run.rs`. A surface's tests reach them as they reach the backend binary.
3. Rust sites that need nothing keep `.env_clear()`. Moving forty-five correct sites onto a helper would churn files other tickets own for no gain.
4. The check is a regex scanner with pinned plants. A parser per language would cost far more, and the plants pin what it catches.
5. A stall of 2 seconds ends a half request. A shorter window would read a slow client's bytes under load, and a later reset would then answer 500.
6. A reset of a request that was read answers 500 with a sentence. The standard library cannot send a reset on a drained socket.
7. A scripted listener keeps its port until the process exits and answers extra connections with 500.
8. The sentinel proofs for the helpers live in `conformance/children/test.sh`, which the `test` rung runs. The surfaces' test files take no new tests.
9. The sweep's 50-run proof runs at normal load. The brief waits while the one-minute load is above 10, so the ticket does not create load 120 on purpose.

## Acceptance

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `sdlc/scripts/children --self-test` | Every row of the check's edge-case table, whole output pinned | Drop the Ruby `unsetenv_others` rule. The Ruby refusal row reads empty |
| `sdlc/scripts/children` on the tree | Exit 0 and `children: 0 inheriting spawns` | Add `Command::new("sh").arg("-c").arg("true").status()` to `tests/transform.rs`. The check names that line. Separately, restore `...process.env` in `backend.mjs`. The check names it |
| `test_deadline::child` unit test | A `sh -c 'test -z "${CARGO_PKG_NAME+x}"'` child through the helper exits 0. The test first asserts that the parent has `CARGO_PKG_NAME`, which cargo sets for every test run. A `#[should_panic]` case pins the refusal sentence for `keep = ["THINKTHEN_BASE_URL"]` | Drop `env_clear()` from the helper. The child exits 1 |
| `conformance/children/test.sh` | Runs each non-Rust helper's check under `env -i PATH=... THINKTHEN_SENTINEL=planted FAKE_SERVICE_API_KEY=planted`. Each check starts `sh -c` through its helper, which exits 0 only when neither name is set. Each pins the refusal sentence for the helper rows of the edge table. A missing `ruby` or `Rscript` prints `not run` and never `pass` | Make the Python helper start from `dict(os.environ)`. The Python check exits 1. The same plant runs once per language |
| `conformance/backend/tests/listener.rs`: half request | The half-request row above. The second client reads 200 within 10 seconds | Remove the stall rule. The second client times out |
| `listener.rs`: large reset | The 40 KiB row. Status 500 and the whole sentence | Remove the `used == 0` branch. The client reads end of file |
| `listener.rs`: script ended | The second-request row. Status 500, `connections()` 2, both request lines | Return from `serve_script` when the script ends. The connection is refused |
| The secrecy sweep and `secrecy_relate.rs` | Every case passes with three markers | An `eprintln!` of the second entity's name in `cli/relate`, then of its kind. Each turns a secrecy test red. Neither plant is committed |
| The secrecy sweep's message | A failing case prints the connection count and the request lines | For one case, open a stray connection to the listener before the command. The failure names 2 connections and the stray request line. Not committed |
| The sweep, 50 runs | `no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` passes 50 runs in a row under the heavy lock | None. A failure is kept and filed with the connections it names |
| rusqlite table | As Part 4 says | A tail of 12, then a tail of 14 |

Each new test answers the four questions of `CLAUDE.md`.

- **The check and its self-test.** It protects the rule that no test child inherits the shell. A new test that spawns without clearing fails it. No existing test reads the spawn sites. It needs no hook.
- **The helper tests.** They protect the allow list and the refusal. A helper edited to copy the parent fails them. No existing test sets a variable in a parent and looks for it in a child. They need no hook: the parent's own environment is the real boundary.
- **The three listener tests.** They protect a clean end for a half request, a loud answer for an impossible reset, and an open port after the script. Each old-code behavior above fails them. No test sends raw bytes to a scripted listener today. They use the public `Listener` and a raw `TcpStream`, with no hook.
- **The secrecy changes.** They extend two existing tests. No new test is added.

## Budgets and the ratchet

- `sdlc/scripts/children`: at most 230 nonblank lines, self-test included.
- `crates/thinkthen/src/test_deadline/child.rs`: at most 45 nonblank lines, its test included.
- `conformance/children/`: at most 30 nonblank lines per helper and 120 for `test.sh` and the four checks it runs.
- `conformance/backend/src/listener.rs`: at most 45 nonblank lines added.
- `conformance/backend/tests/listener.rs`: at most 110 nonblank lines.
- `secrecy.rs` and `secrecy_relate.rs`: at most 30 nonblank lines changed.
- Rust spawn sites: at most 50 nonblank lines changed in all.
- Surface helpers and spawn sites: at most 80 nonblank lines changed in all. Each surface ratchet moves to its measured total, and the commit names each change.
- Part 4: at most 10 lines of prose, or 40 nonblank lines if the new test is needed.
- `lint` and `test` rungs: at most 4 lines added.
- `sdlc/ratchet.json` rises to the measured total in the commit that adds the code, at most 260 more than today. The commit names where the builder looked for duplication first: the spawn blocks in `tests/backend/harness/mod.rs` and `tests/support/measure.rs`, and `serve_script` beside `serve_kept`.
- No dependency.

## Stop rules

Stop, write down what happened, and hand back before any of these:

- A child needs a kept name that starts with `THINKTHEN_` or looks secret-shaped.
- More than three existing tests depend on a scripted listener closing its port. The fallback is to keep the port behavior and ship only the counts and the sweep's message.
- A secrecy marker reaches a diagnostic on main. File it for relate and leave the code.
- The 50 runs take longer than 45 minutes, or the load stays above 10. Record how many ran.
- Part 4 needs a committed change under `databases/sqlite/src`.
- A budget would be crossed, a dependency added, a public API or command behavior changed, or a file edited that the brief gives to 0122, 0123, 0124, 0125, 0126, or 0129, beyond the two secrecy files this ticket names.
- A surface's `check.sh` needs a change.

## Steps

- **Step A** lands everything above.
- **Step B** follows ticket 0122's landing. `libraries/python/tests/conftest.py` `child_env`, and every Python surface spawn site, moves onto `conformance/children/children.py`. `test_secrecy.py`'s environment test gains the sentinel check the pandas issue asks for. The pending `libraries/python/` entry leaves the check, and that issue closes. If 0122 lands before step A, step B joins step A. Otherwise step B lands later on this ticket's branch, as a second reviewed change.

## Scope and exclusions

Excluded: item 1 of the issue, which 0119 owns, and items 5 to 9. Any change to the `thinkthen` binary, the library, or a surface's shipped code. Any change under `databases/*/src`. Any live or paid call.

## Dependencies and order

Lands before ticket 0119. Step B waits for ticket 0122. The two secrecy files may meet 0123's relate changes, so the coordinator merges whichever lands second.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The change raises the ceiling, so the code review names what it checked.

## Complexity

Contract 1; state and timing 2; reach 3; proof 2; cost of error 2; total 10. Final level: 2. The reach is wide but shallow. The timing risk sits in the listener's stall rule.

## Evidence

- Starts from: `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md` items 2, 3, 4, and 10. `sdlc/records/0089-code-review.md` FU3 found the two listener edges. `sdlc/records/0088-review-final.md` F2 showed a planted print of `Acme` passing all five secrecy tests. `sdlc/records/qf-heavy-lock.md` recorded the one relate failure at load 120. `experiments/207-thinkthen-db/sqlite/NOTES.md` in the workspace found the rusqlite table's 3.34 end. The pandas branch's issue `2026-09-25-python-test-children-inherit-the-whole-shell-environment.md` at `origin/ticket/0122-support-pandas` records the inherited-key print. `databases/duckdb/tools/harness.py` `child_env` and the Rust backend harness already build a child's environment from nothing, and both are the model here.
- Keeps: Every existing test's meaning. The fake keys and loopback addresses each test sets. The reset of a request that was peeked. The answering listener's behavior. The forty-five Rust sites that already clear the environment. The secrecy sweep's cases and routes.
- Changes: One helper per language and a lint check. Children that inherited now get an allow list. The listener ends a stalled half request, answers an impossible reset with 500, keeps a scripted port open, and counts its connections. The sweep marks the second relate entity and its kind, and names what the listener saw. One duplicate test in `exchange.rs` goes.
- Proof: The acceptance table. Each new test has a planted fault that turns it red, and the build records each plant's red run.
- Defers: Shell spawns, and the test runners themselves. `cargo test`, `pytest`, `node --test`, `ruby`, and `Rscript` still start with the shell's environment from the rungs and from each `check.sh`. An in-process test can still read a stray `THINKTHEN_*` value. The fix is one `env -i` allow list at the heavy rungs and the surface checks, and it needs its own ticket, because every toolchain's variables must be found on Yellow first. The Python surface's sites wait for step B. The three pending files wait for their owners. The C driver's `fork` inherits from a parent the helper already cleared. A reset of a request over 32 KiB. The 50-run proof at high load. Items 1 and 5 to 9 of the issue.

## What Ian can overturn

Each numbered decision above. The shell deferral, if he wants the rung-level `env -i` in this ticket. The Part 4 reading of his ruling: an existing test that turns red on the plants counts as the added test.

## Issues this closes

- Items 2, 3, 4, and 10 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. The issue stays open for items 1 and 5 to 9. The landing commit moves the four items to its "Already fixed" section, each with this ticket's number.
- `sdlc/issues/2026-09-25-python-test-children-inherit-the-whole-shell-environment.md`, at step B.
