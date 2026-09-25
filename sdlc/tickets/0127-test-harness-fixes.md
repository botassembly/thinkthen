---
flow: build
priority: 127
opens: conformance/backend/src/listener.rs conformance/backend/tests conformance/children crates/thinkthen/src/test_deadline crates/thinkthen/src/cli/conformance_tests/command.rs crates/thinkthen/src/cli/interrupt/tests.rs crates/thinkthen/src/engine/host_signal_tests.rs crates/thinkthen/src/engine/recorder/identity/tests.rs crates/thinkthen/tests/backend/secrecy.rs crates/thinkthen/tests/backend/secrecy_relate.rs crates/thinkthen/tests/backend/exchange.rs crates/thinkthen/tests/backend/interrupt.rs crates/thinkthen/tests/compile_contract.rs crates/thinkthen/tests/demo_runner.rs crates/thinkthen/tests/transform.rs crates/thinkthen/tests/version.rs libraries/c/tests libraries/python/tests libraries/typescript/tests libraries/ruby/tests libraries/r/tests databases/sqlite/tests databases/duckdb/tools/harness.py databases/duckdb/tools/signal_suite.py databases/duckdb/tools/settings_suite.py databases/duckdb/tools/site_examples.py databases/postgresql/tests sdlc/scripts/children sdlc/scripts/heavy-lock sdlc/scripts/lint sdlc/scripts/test sdlc/scripts/README.md sdlc/ratchet.json databases/sqlite/ratchet.py.json databases/duckdb/ratchet.py.json databases/postgresql/ratchet.py.json libraries/c/ratchet.json libraries/python/ratchet.py.json libraries/ruby/ratchet.rb.json libraries/r/ratchet.R.json libraries/typescript/ratchet.mjs.json sdlc/issues sdlc/records sdlc/tickets
---

# 0127: Fix the test harness before the mutation audit

Status: ready. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Ticket 0119 audits the engine tests by mutation. It keeps the secrecy sweep and leans on the shared loopback backend. Both have known holes. This ticket closes them first. It also stops test children from inheriting the developer's shell, and it strips stray `THINKTHEN_*` names from the heavy rungs.

The authority is the backlog `sdlc/planning/issue-backlog-2026-09-25.md`, row 9 and Ian's ruling 7, the coordinator's brief for 0127, and the coordinator's rulings on the first review. The ticket settles items 2, 3, and 10 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. It lands the fix for item 4 and leaves that item open for its high-load proof. It settles `sdlc/issues/2026-09-25-python-test-children-inherit-the-whole-shell-environment.md`, which ticket 0122 filed.

The environment work answers an incident. On 2026-09-25 the 0122 build planted a fault that formatted a Python test child's environment into an error. The assertion diff printed unrelated service keys from the builder's shell. The child had inherited everything. A `THINKTHEN_*` variable left in a shell can also change what a child does, and so a test's result.

## Part 1: every test child gets an allow-listed environment

### The rule

Every test that starts a child process builds that child's whole environment. The child sees only the variables the test names. A test never copies the parent's environment and then removes names. Test code reads the parent's environment one name at a time.

### The helper, one per language

Each helper starts a child with an empty environment, adds `PATH` from the parent, and adds the names the call site asks it to keep. It sets `THINKTHEN_*` values only from explicit arguments. It refuses to keep a name that starts with `THINKTHEN_` or holds `KEY`, `TOKEN`, `SECRET`, `PASSWORD`, `CREDENTIAL`, or `AUTH`, in any case. The refusal is a panic or an error with this exact sentence:

```text
a test child may not keep NAME from the parent: set a THINKTHEN_ value or a fake key explicitly
```

| Language | Helper | Used by |
| --- | --- | --- |
| Rust | `crates/thinkthen/src/test_deadline/child.rs`: `command(program, keep) -> Command` | The crate's unit tests as `crate::test_deadline::child`. Integration tests, `conformance/backend/tests`, and `libraries/c/tests` include it by `#[path]`, as they already include `run.rs` and `wait.rs` |
| Python | `conformance/children/children.py`: `child_env(keep=(), **values) -> dict` | `libraries/python/tests`, `databases/sqlite/tests`, `databases/duckdb/tools`, and `databases/postgresql/tests` |
| TypeScript | `conformance/children/children.mjs`: `childEnv({ keep, values })` | `libraries/typescript/tests` |
| Ruby | `conformance/children/children.rb`: `Children.env(keep:, **values)`. Every call site also passes `unsetenv_others: true` | `libraries/ruby/tests` |
| R | `conformance/children/children.R`: `clean_env(keep = character(), values = character())` | `libraries/r/tests` |

The R helper has one form. It returns a character vector of words for `env`: first `-i`, then `shQuote(paste0(name, "=", value))` for `PATH`, each kept name, and each named value. A call site runs `system2("env", c(clean_env(keep, values), shQuote(program), shQuote(arguments)))`. The one background spawn in `interrupt.R` pastes the same words after `env` into its `system` line. The R sites read `THINKTHEN_BASE_URL` and the fake key with `Sys.getenv("NAME")` and pass them to `clean_env` as values. Today their children inherit both from `with-backend.sh`.

`conformance/` already holds the test support every surface shares. A Rust site that needs no variable at all keeps its plain `.env_clear()`. Forty-five Rust sites already clear the environment that way, and none moves.

### The sites

A scan of `origin/main` at `c8ca9a65` found these children that inherit the whole environment, or copy it and remove names. Each moves onto its language's helper or gains `.env_clear()`. The build re-runs the scan and fixes any site this list missed.

| Language | Sites |
| --- | --- |
| Rust, no variable needed | `conformance/backend/tests/binary.rs:38`, `crates/thinkthen/src/cli/conformance_tests/command.rs:178`, `cli/interrupt/tests.rs:273`, `engine/recorder/identity/tests.rs:231`, `tests/version.rs:13` |
| Rust, `PATH` only | `cli/interrupt/tests.rs:264` and `tests/backend/interrupt.rs:100` (`kill`), `engine/host_signal_tests.rs:124` (`sh`), `tests/transform.rs:318` (`mkfifo`), `tests/demo_runner.rs:33` (`sh`), `libraries/c/tests/door/main.rs:85`, `:159`, and `:171` (`cc`, `readelf`, `nm`) |
| Rust, a toolchain | `tests/compile_contract.rs:173`, `libraries/c/tests/door/main.rs:42`, `libraries/c/tests/door/bytes.rs:154` (`cargo`) |
| Python, the pandas surface | `libraries/python/tests/conftest.py:31` starts the backend with no `env`. `conftest.py:68` `child_env` copies `dict(os.environ)` less the key. Its callers in `test_release.py` and elsewhere keep their calls. `examples.py:22` copies `dict(os.environ, ...)`. `test_door.py:145` passes no `env` |
| Python, the databases | `databases/sqlite/tests/helper.py:103` `environment` copies `os.environ` less the key. Its callers keep their calls. `helper.py:56` and `databases/duckdb/tools/harness.py:37` start the backend with no `env`. `databases/postgresql/tests/examples.py:23` and `runner.py:76` start `psql` with no `env`. `databases/duckdb/tools/harness.py:85` `child_env` already builds an allow list and moves onto the helper |
| TypeScript | `libraries/typescript/tests/backend.mjs:18` starts the backend with no `env`. `backend.mjs:50` `childEnv` spreads `process.env` |
| Ruby | `libraries/ruby/tests/backend.rb:50` starts the backend with no environment. `backend.rb:128` copies `ENV.to_h` less `THINKTHEN_*`. `tests/test_public_names.rb:18` passes no environment |
| R | `libraries/r/tests/helper.R:57` and `interrupt.R:20` add names to the inherited environment. `conformance.R:28` runs `sha256sum` with it |

`cli/conformance_tests/command.rs:192` removes steering names it finds with `vars_os()`, and `:240` lists them the same way in the child. The first loop goes, since the child now starts empty. The second reads `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` by name, the two names the probe exists to rule out.

Each call site names what its child keeps. A child of the `thinkthen` binary keeps nothing. A toolchain child keeps the names its tool reads, such as `HOME`, `CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `CARGO_TARGET_DIR`, and `RUSTC_WRAPPER`. The SQLite children keep `LD_LIBRARY_PATH`, which `check.sh` sets to the pinned 3.50.0 host. A Python library child may keep `PYTHONPATH` or `VIRTUAL_ENV` if its import needs it. The build measures each list by running the site, and the record lists each one.

### The check: `sdlc/scripts/children`

A new standard-library Python script scans every tracked `.rs`, `.py`, `.mjs`, `.js`, `.ts`, `.rb`, and `.R` file. It runs from the `lint` rung twice: `children --self-test`, then `children`. It follows `sdlc/scripts/tickets` in shape. It skips comment lines.

The check has two rules.

**Spawns.** In every scanned file, each spawn form below must build its child's environment.

| Language | A spawn is | It passes when |
| --- | --- | --- |
| Rust | `Command::new(` | `.env_clear()` appears in its statement, or in the later statements that begin with the name a `let` bound it to |
| Python | `subprocess.run`, `Popen`, `call`, `check_call`, `check_output`, `asyncio.create_subprocess_exec` | its parentheses hold `env=` and not `env=None`. `os.system`, `os.popen`, `os.exec*`, and `os.spawn*` fail outright |
| TypeScript | `spawn`, `spawnSync`, `exec`, `execSync`, `execFile`, `execFileSync`, `fork` in a file that names `child_process` | its parentheses hold `env:` and not `env: undefined` |
| Ruby | `Open3.*`, `Process.spawn`, `spawn`, `system`, `IO.popen`, `exec` | its parentheses hold `unsetenv_others: true`. A backtick command and `%x` fail outright |
| R | `system(`, `system2(` | its parentheses hold `clean_env(` |

**Whole-environment reads in test code.** Test code may name the parent's environment only in these single-name forms. Every other mention of the whole map fails.

| Language | Passes | Fails, for example |
| --- | --- | --- |
| Rust | `env::var(`, `env::var_os(` | `env::vars(`, `env::vars_os(` |
| Python | `os.environ.get(`, `os.environ[`, `in os.environ`, `os.environ.pop(`, `os.environ.setdefault(`, `os.getenv(` | `dict(os.environ`, `os.environ.copy(`, `**os.environ`, `os.environ.items(`, `os.environ.update(`, `env=os.environ` |
| TypeScript | `process.env.NAME`, `process.env[` | `...process.env`, `env: process.env`, `Object.keys(process.env)` |
| Ruby | `ENV.fetch(`, `ENV[` | `ENV.to_h`, `ENV.dup`, `ENV.each`, `ENV.reject`, `ENV.key?` |
| R | `Sys.getenv("NAME")` | `Sys.getenv()`, `Sys.getenv(names)` with a vector |

Test code is any file under a `tests/` folder, under `conformance/`, or under `databases/duckdb/tools/`, and any Rust file named `tests.rs` or ending in `_tests.rs`. The pandas surface's `pop` and `setdefault` calls name one variable each, so they pass beside the coordinator's three Python forms.

A failure reads `children: PATH:LINE: this child inherits the whole environment; build it with the LANGUAGE child helper`, or `children: PATH:LINE: test code reads the whole environment; read one name at a time`.

The check catches the forms in its tables and nothing else. A spawn through another library, or a map reached through an alias, passes it. C sources and shell scripts are outside its scope.

Two tables sit at the top of the script. Each entry names a path, an owner, and a reason. The check fails on an entry whose path no longer holds a finding, so neither table can rot.

| Table | Path | Reason |
| --- | --- | --- |
| Excluded | `sdlc/scripts/` | The gate's own tools run `cargo`, `git`, and `node` in the developer's toolchain. `live` builds its own environment by design |
| Excluded | `probes/`, `site/`, `databases/duckdb/vendor/`, `libraries/r/thinkthen/tools/` | Live measurements run by hand, the site build, vendored code, and the R package's install-time tools. None is a test |
| Exempt | `libraries/typescript/tests/fork.test.mjs` | The `fork` runs inside a child the helper already built, so it inherits only the allow list |
| Pending 0123 and 0126 | `crates/thinkthen/tests/relate_edge.rs` | Its one inheriting spawn runs `relate --help`. Relate and help text are both in flight |
| Pending 0129 | `databases/postgresql/src/files.rs` | Its test module runs `mkfifo`. 0129 owns `databases/*/src` |
| Pending 0129 | `databases/duckdb/tools/databases_suite.py`, `databases/duckdb/tools/source_checks.py` | `databases_suite.py:117` starts a Python child with no `env`, and `source_checks.py:59` and `:168` start `cargo` with none. 0129 owns both files |

The self-test builds one file per row of the edge-case table below in a scratch tree and pins each whole output.

### The heavy rungs drop stray `THINKTHEN_*` names

`sdlc/scripts/heavy-lock` is sourced by `install`, `test`, `spec`, and `surfaces`. Before it takes the lock, it unsets every exported name that starts with `THINKTHEN_` except its own `THINKTHEN_HEAVY_LOCK` and `THINKTHEN_HEAVY_LOCK_HELD`. It finds the names with `awk` over `ENVIRON` and prints nothing. The unset runs before the `exec flock` branch, so the re-run rung starts without them. It also covers a nested rung and a machine with no `flock`, where no `exec` happens. `sdlc/live-test` and each test already set every `THINKTHEN_*` value they use. `sdlc/scripts/README.md` gains one clause on the row for `heavy-lock`.

The runner-level allow list for secret-shaped names stays deferred. The landing commit files it as an issue in `sdlc/issues/`.

### Edge cases for the check, the helpers, and the rungs

| Input | Expected |
| --- | --- |
| Rust `Command::new("sh").arg("-c").output()` | Refused |
| Rust `Command::new("sh").env_clear().arg("-c").output()` | Passes |
| Rust `let mut c = Command::new(x);` then `c.env_clear();` then `c.spawn()` | Passes |
| Rust `run::output(Command::new(x).arg("--version"))` | Refused |
| Rust `.env_clear().envs(std::env::vars())` in test code | Refused |
| Rust `std::env::var_os("PATH")` in test code | Passes |
| Python `subprocess.run(["x"], capture_output=True)` | Refused |
| Python `subprocess.run(["x"], env=None)` | Refused |
| Python `subprocess.run(["x"], env=child_env())` | Passes |
| Python `env = dict(os.environ)` in test code | Refused |
| Python `os.environ.get("PATH")`, `os.environ["X"]`, `"X" in os.environ` in test code | Passes |
| Python `os.environ.items()` in test code | Refused |
| TypeScript `spawn(bin, [], { stdio })` | Refused |
| TypeScript `spawn(bin, [], { env: undefined })` | Refused |
| TypeScript `env: { ...process.env, X: '1' }` in test code | Refused |
| TypeScript `process.env.THINKTHEN_TEST_BACKEND` in test code | Passes |
| Ruby `Open3.capture3(env, ruby, "-e", s)` | Refused |
| Ruby `Open3.capture3(env, ruby, "-e", s, unsetenv_others: true)` | Passes |
| Ruby `` `ls` `` and `%x(ls)` | Refused |
| Ruby `ENV.to_h` in test code | Refused |
| Ruby `ENV.fetch("HOME")` in test code | Passes |
| R `system2("sha256sum", file, stdout = TRUE)` | Refused |
| R `system2("env", c(clean_env(), shQuote("sha256sum"), shQuote(file)))` | Passes |
| R `Sys.getenv()` in test code | Refused |
| R `Sys.getenv("TT_TESTS")` in test code | Passes |
| An exclusion or pending path with no finding left | Refused as a stale entry |
| Helper `keep` of `LD_LIBRARY_PATH` set in the parent | Kept |
| Helper `keep` of a name the parent lacks | Left out, no error |
| Helper `keep` of `THINKTHEN_BASE_URL`, `OPENAI_API_KEY`, `GITHUB_TOKEN`, `db_password`, or `AWS_SECRET_ACCESS_KEY` | Refused with the exact sentence |
| Helper value `THINKTHEN_API_KEY="sk-fake"` | Set, since the test named it |
| A rung sourced with `THINKTHEN_SENTINEL`, `THINKTHEN_HEAVY_LOCK`, and `THINKTHEN_HEAVY_LOCK_HELD` set | The sentinel is gone. Both lock names stay |

## Part 2: the shared loopback backend (items 2 and 4)

All of this lives in `conformance/backend/src/listener.rs`.

1. **A stalled peek falls into the full read.** `peek_request` still peeks, so a reset can drop unread bytes. Today it falls back to `read_request(&mut BufReader::new(stream))` with `used` 0 when the peek fills its 32 KiB buffer. It now takes that same branch when `seen` has stayed the same for 2 seconds of polls. That read ends in one of two ways. A client that closed after half a request makes `read_request` meet end of file and return `None`, so the serving thread returns. A slow client finishes the request by reading, with `used` 0. No thread spins any more.
2. **A reset of a request that was read answers loudly.** When `used` is 0, a `Canned::reset()` reply cannot drop unread bytes, and a close would look like a clean end. The listener answers the drift status 500 instead, the fail-loud reply the arms already use. The body and a standard error line read `the loopback listener cannot reset a request it had to read`. It does not panic, because a panic in a serving thread only closes the stream, and the client again sees a plain close.
3. **A scripted listener counts, skips, and keeps its port.** `serve_script` counts every connection in `Counts::connections`. A connection that closes before a whole request, including one that sends zero bytes, takes no scripted reply. The listener skips it and accepts the next connection. Today it returns and closes the port. After the script runs out, the thread keeps accepting until the test process exits. It records each extra request and answers it with the drift status and `the script has no reply left for this connection`.
4. **A dropped `Listener` keeps its port.** The serving thread owns the socket, and dropping the `Listener` closes only its record channel. The thread ignores the closed channel. It keeps the port bound and answers every later connection with the drift status until the process exits. No other test in the same process can then bind that port while a client of this one may still connect. The answering listeners already hold their ports this way. One secrecy sweep opens about 520 scripted listeners, so one process holds about 520 idle threads and sockets. The build records the sweep's time before and after.
5. **The secrecy sweep names what the listener saw.** The status assertion in `crates/thinkthen/tests/backend/secrecy.rs` prints the connection count and each recorded request's line and body length. It prints no body, since the body holds the evidence.

Point 3 gives `crates/thinkthen/tests/backend/state.rs:198` its meaning. That test asserts `connections()` reads 0 on a scripted listener after a refused input. Today `serve_script` never counts, so the assertion holds whatever the command does. After this ticket, a request sent by mistake fails it.

Item 4's cause stays unproven. Points 3 and 4 remove the likely cause inside one test process, and point 5 names a stray connection if one comes from anywhere else.

The existing `a_backend_that_answers_nothing_is_exit_four` in `crates/thinkthen/tests/backend/exchange.rs` relies on an empty script closing the port. Under point 3 it would test the drift reply. The refusal it pins is already pinned by `a_refused_port_fails_before_the_first_default_retry_wait` in the same file, so the build deletes it.

### Edge cases for the listener

| Client sends | Script | Old code | New code |
| --- | --- | --- | --- |
| A whole request | `ok` | 200 | 200 |
| Half a request, then closes | `ok`, then a second client sends a whole request | The thread spins on the first connection, and the second client waits forever | The first connection ends after the stall. The second gets the 200. `connections()` reads 2 and `requests()` holds one request |
| Nothing, then closes | `ok`, then a second client sends a whole request | The listener returns and closes the port. The second client is refused | The first connection is counted and skipped. The second gets the 200 |
| A 40 KiB request | `reset` | The client reads a clean end of file | The client reads status 500 and the sentence |
| A 1 KiB request | `reset` | The client sees a reset | The client sees a reset |
| A second whole request | `ok` only | The connection is refused | The client reads status 500 and the sentence. `connections()` reads 2 |
| A whole request after the `Listener` is dropped | `ok`, already used | The connection is refused | The client reads status 500 and the sentence |

## Part 3: the secrecy sweep marks both relate entities (item 3)

`secrecy.rs` gains two markers beside `EVIDENCE`: `SECOND`, `marker-second-e41d09`, and `KIND`, `marker-kind-93c2f0`. Every relate input in `evidence` names the second entity `SECOND` in place of `Acme`. Every framing that carries a kind gives both entities the kind `KIND`, since `linked --either` pairs one kind with itself. `--lines` carries no kind. `nothing_leaked` refuses `EVIDENCE`, `SECOND`, and `KIND` in standard error for every case.

`secrecy_relate.rs` names its second entity `SECOND` in place of `Ada`. Its rule `linked=person:person` and both kinds stay `person`, and its call to `nothing_leaked` refuses the kind marker in a run that holds none.

If the stronger check finds either marker in a diagnostic on main today, that is a real leak in relate. The build stops, files it, and leaves relate's code to ticket 0123.

## Part 4: the rusqlite workaround test (item 10, Ian's ruling 7)

Ian ruled that nobody asks the rusqlite maintainers for a fix, and that a test guards the hand-extended API table in `databases/sqlite/src/ffi.rs:21` to `:30`. Ticket 0129 owns `databases/*/src`, so this ticket commits no change there.

`databases/sqlite/tests/test_interrupt.py` already loads the extension into the pinned 3.50.0 host and interrupts held calls. The extension learns of the interrupt only through the `is_interrupted` pointer the table reads. The build plants a tail of 12 and a tail of 14 in `ffi.rs`, one at a time, runs the SQLite check, and restores the file.

If both plants turn `test_interrupt.py` red, that file is the test. Its docstring records Ian's ruling and names the table it guards. The build record names, for each plant, which test went red and how: the assertion it failed, or the crash and its signal. If either plant stays green, the build stops and hands back. It adds no second test on its own.

## Decisions

Each is the agent's decision unless a line names the coordinator. Ian can overturn any of them.

1. The helpers keep a name only when the call site asks for it, and they refuse secret-shaped and `THINKTHEN_` names. A deny list alone would miss the next service's key.
2. The helpers live in `conformance/children/`, beside the shared backend, with the Rust one beside `run.rs`. A surface's tests reach them as they reach the backend binary.
3. Rust sites that need nothing keep `.env_clear()`. Moving forty-five correct sites onto a helper would churn files other tickets own for no gain.
4. The check is a regex scanner with pinned plants, and it reads the whole environment through an allow list of single-name forms. A parser per language would cost far more, and the plants pin what it catches. The coordinator ruled the allow list.
5. A stall of 2 seconds sends a peek into the full read. A shorter window would read a slow client's bytes under load, and a later reset would then answer 500.
6. A reset of a request that was read answers 500 with a sentence. The standard library cannot send a reset on a drained socket.
7. A scripted listener keeps its port until the process exits, even after the `Listener` is dropped, and answers extra connections with 500.
8. The sentinel proofs for the helpers live in `conformance/children/test.sh`, which the `test` rung runs. The surfaces' test files take no new tests.
9. The heavy rungs unset stray `THINKTHEN_*` names. The coordinator ruled this cheap step in, and the secret-shaped runner allow list out.
10. The sweep's 50-run proof runs at normal load. The brief waits while the one-minute load is above 10, so the ticket does not create load 120 on purpose, and item 4 stays open.

## Acceptance

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `sdlc/scripts/children --self-test` | Every check row of the edge-case table, whole output pinned | Drop the Ruby `unsetenv_others` rule. The Ruby refusal row reads empty |
| `sdlc/scripts/children` on the tree | Exit 0 and `children: 0 findings` | Add `Command::new("sh").arg("-c").arg("true").status()` to `tests/transform.rs`. The check names that line. Separately, restore `dict(os.environ)` in `libraries/python/tests/conftest.py`. The check names it |
| `test_deadline::child` unit test | A `sh -c 'test -z "${CARGO_PKG_NAME+x}"'` child through the helper exits 0. The test first asserts that the parent has `CARGO_PKG_NAME`, which cargo sets for every test run. A `#[should_panic]` case pins the refusal sentence for `keep = ["THINKTHEN_BASE_URL"]` | Drop `env_clear()` from the helper. The child exits 1 |
| `conformance/children/test.sh` | Runs each non-Rust helper's check under `env -i PATH=... THINKTHEN_SENTINEL=planted FAKE_SERVICE_API_KEY=planted`. Each check starts `sh -c` through its helper, which exits 0 only when neither name is set. Each pins the refusal sentence for the helper rows of the edge table. A missing `ruby` or `Rscript` prints `not run` and never `pass` | Make the Python helper start from `dict(os.environ)`. The Python check exits 1. The same plant runs once per language |
| The `heavy-lock` line in `lint` | Sources `heavy-lock` under `env -i` with the rung row's three names set and the lock marked held, then pins that the sentinel is unset and both lock names remain | Remove the unset loop. The sentinel survives |
| `libraries/python/tests/test_secrecy.py` | Its environment test also asserts that a sentinel the parent sets does not reach the child, as the 0122 issue asks | Restore `dict(os.environ)` in `conftest.py` `child_env`. The sentinel reaches the child |
| `conformance/backend/tests/listener.rs`: half request | The half-request row. The second client reads 200 within 10 seconds | Remove the stall rule. The second client times out |
| `listener.rs`: zero-byte stray | The zero-byte row. The second client reads 200 | Return from `serve_script` on a closed connection. The second client is refused |
| `listener.rs`: large reset | The 40 KiB row. Status 500 and the whole sentence | Remove the `used == 0` branch. The client reads end of file |
| `listener.rs`: script ended and dropped | The second-request row and the dropped row. Status 500, `connections()` 2, both request lines | Return from `serve_script` when the script ends. The connection is refused |
| The secrecy sweep and `secrecy_relate.rs` | Every case passes with three markers | An `eprintln!` of the second entity's name in `cli/relate`, then of its kind. Each turns a secrecy test red. Neither plant is committed |
| The secrecy sweep's message | A failing case prints the connection count and the request lines | For one case, open a stray connection to the listener before the command. The failure names 2 connections and the stray request line. Not committed |
| The sweep, 50 runs | `no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` passes 50 runs in a row under the heavy lock | None. A failure is kept and filed with the connections it names |
| rusqlite table | As Part 4 says | A tail of 12, then a tail of 14 |

Each new test answers the four questions of `CLAUDE.md`.

- **The check and its self-test.** It protects the rule that no test child inherits the shell. A new test that spawns without clearing fails it. No existing test reads the spawn sites. It needs no hook.
- **The helper tests.** They protect the allow list and the refusal. A helper edited to copy the parent fails them. No existing test sets a variable in a parent and looks for it in a child. They need no hook: the parent's own environment is the real boundary.
- **The `heavy-lock` line.** It protects the rung's unset. Removing the loop fails it. No existing check sources `heavy-lock`. It needs no hook, since the held-lock name is the file's own nested-rung path.
- **The four listener tests.** They protect a clean end for a half request, a skip for a zero-byte stray, a loud answer for an impossible reset, and an open port after the script and after a drop. Each old-code behavior above fails them. No test sends raw bytes to a scripted listener today. They use the public `Listener` and a raw `TcpStream`, with no hook.
- **The secrecy and `test_secrecy.py` changes.** They extend existing tests. No new test is added.

## Budgets and the ratchet

- `sdlc/scripts/children`: at most 260 nonblank lines, self-test included.
- `crates/thinkthen/src/test_deadline/child.rs`: at most 45 nonblank lines, its test included.
- `conformance/children/`: at most 30 nonblank lines per helper and 120 for `test.sh` and the four checks it runs.
- `conformance/backend/src/listener.rs`: at most 45 nonblank lines added.
- `conformance/backend/tests/listener.rs`: at most 130 nonblank lines.
- `secrecy.rs` and `secrecy_relate.rs`: at most 30 nonblank lines changed.
- Rust spawn sites: at most 50 nonblank lines changed in all.
- Surface helpers and spawn sites, the pandas surface included: at most 110 nonblank lines changed in all. Each surface ratchet moves to its measured total, and the commit names each change.
- `heavy-lock`: at most 6 nonblank lines added. `lint` and `test` rungs: at most 12 lines added.
- Part 4: at most 6 lines of docstring.
- `sdlc/ratchet.json` rises to the measured total in the commit that adds the code, at most 280 more than today. The commit names where the builder looked for duplication first: the spawn blocks in `tests/backend/harness/mod.rs` and `tests/support/measure.rs`, and `serve_script` beside `serve_kept`.
- No dependency.

## Stop rules

Stop, write down what happened, and hand back before any of these:

- A child needs a kept name that starts with `THINKTHEN_` or looks secret-shaped.
- More than three existing tests depend on a scripted listener closing its port. The fallback is to keep the port behavior and ship only the counts and the sweep's message.
- The kept listeners slow the secrecy sweep by more than a fifth, or exhaust file descriptors or threads.
- A rung breaks because `heavy-lock` unset a `THINKTHEN_*` name it needed.
- A secrecy marker reaches a diagnostic on main. File it for relate and leave the code.
- The 50 runs take longer than 45 minutes, or the load stays above 10. Record how many ran.
- Either Part 4 plant leaves `test_interrupt.py` green, or Part 4 needs a committed change under `databases/sqlite/src`.
- A budget would be crossed, a dependency added, a public API or command behavior changed, or a file edited that another in-flight ticket owns, beyond the overlaps named under Dependencies.
- A surface's `check.sh` needs a change.

## Scope and exclusions

Excluded: item 1 of the issue, which 0119 owns, and items 5 to 9. Any change to the `thinkthen` binary, the library, or a surface's shipped code. Any change under `databases/*/src`. The runner-level allow list for secret-shaped names. Any live or paid call.

## Dependencies and order

Lands before ticket 0119. Ticket 0122 has landed on main at `c8ca9a65`, so its test folder is in scope.

These files overlap other in-flight tickets. Whichever ticket lands second merges.

| Ticket | Overlap | Order |
| --- | --- | --- |
| 0123, relate | `tests/backend/exchange.rs`, `secrecy.rs`, `secrecy_relate.rs` | 0123 lands after 0127, or merges if it lands first |
| 0126, help text | This ticket opens named files under `crates/thinkthen/src/cli` and `crates/thinkthen/tests`, never whole folders. `tests/version.rs` is the likeliest meeting point. `relate_edge.rs` waits for 0126 | Whichever lands second merges |
| 0129, `databases/*/src` | `databases/postgresql/src/files.rs`, `databases/duckdb/tools/databases_suite.py`, and `databases/duckdb/tools/source_checks.py` stay pending for 0129. This ticket plants in `databases/sqlite/src/ffi.rs` and restores it | 0129 clears its pending entries, or a Quick Fix does after it lands |
| 0126 and 0129 | `databases/sqlite/README.md`. This ticket leaves it alone, and records the ruling in the `test_interrupt.py` docstring | None needed |

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The change raises the ceiling, so the code review names what it checked.

## Complexity

Contract 1; state and timing 2; reach 3; proof 2; cost of error 2; total 10. Final level: 2. The reach is wide but shallow. The timing risk sits in the listener's stall rule.

## Evidence

- Starts from: `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md` items 2, 3, 4, and 10. `sdlc/records/0089-code-review.md` FU3 found the two listener edges. `sdlc/records/0088-review-final.md` F2 showed a planted print of `Acme` passing all five secrecy tests. `sdlc/records/qf-heavy-lock.md` recorded the one relate failure at load 120. `experiments/207-thinkthen-db/sqlite/NOTES.md` in the workspace found the rusqlite table's 3.34 end. `sdlc/issues/2026-09-25-python-test-children-inherit-the-whole-shell-environment.md`, filed by 0122, records the inherited-key print. `databases/duckdb/tools/harness.py` `child_env` and the Rust backend harness already build a child's environment from nothing, and both are the model here.
- Keeps: Every existing test's meaning. The fake keys and loopback addresses each test sets. The reset of a request that was peeked. The answering listener's behavior. The forty-five Rust sites that already clear the environment. The secrecy sweep's cases and routes. The heavy lock's own two names.
- Changes: One helper per language and a lint check. Children that inherited now get an allow list, the pandas surface's included. The heavy rungs drop stray `THINKTHEN_*` names. The listener sends a stalled peek into the full read, skips closed connections, answers an impossible reset with 500, keeps a scripted port open, and counts its connections. The sweep marks the second relate entity and its kind, and names what the listener saw. One duplicate test in `exchange.rs` goes.
- Proof: The acceptance table. Each new test has a planted fault that turns it red, and the build records each plant's red run.
- Defers: Shell spawns and the runner-level allow list. `cargo test`, `pytest`, `node --test`, `ruby`, and `Rscript` still start with every non-`THINKTHEN_` name in the shell. Each `check.sh`, `with-backend.sh`, and `runtime.sh` still passes those names on. The fix is one `env -i` allow list at the heavy rungs and the surface checks. Every toolchain's variables must be found on the Beelink first, where ThinkThen builds. The landing commit files that issue. The four pending files wait for their owners. The C driver's `fork` inherits from a parent the helper already cleared. A reset of a request over 32 KiB. The 50-run proof at high load, which keeps item 4 open. Items 1 and 5 to 9 of the issue.

## What Ian can overturn

Each numbered decision above. The deferral of the runner-level allow list. The Part 4 reading of his ruling: an existing test that turns red on both plants counts as the added test. The two extra Python single-name forms, `pop` and `setdefault`.

## Issues this closes

- `sdlc/issues/2026-09-25-python-test-children-inherit-the-whole-shell-environment.md`. It moves to `closed/` in the landing commit.
- Items 2, 3, and 10 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. The issue stays open for items 1, 4, and 5 to 9. The landing commit moves the three items to its "Already fixed" section with this ticket's number. Item 4 records that the fix landed in 0127 and that the high-load proof is still owed.
- The landing commit files one new issue: the runner-level allow list for secret-shaped names.
