# 0361: loopback test servers close each reply, and tests hold the input pause

Status: ready. Batch C1 in `../planning/issue-priorities-2026-09-30.md`. Starts after ticket 0356 lands, because both edit binding test fixtures and their Python ratchets. Branch `ticket/0361-test-servers-and-input-pause`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Pays Debt 030, `../issues/2026-09-30-piped-batching-tests-race-the-50-ms-input-pause.md`. Extends Debt 020's workaround, `../issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`.

## Outcome

Every Python loopback server that a test, check or probe starts sends `Connection: close` on every reply, so ureq never reuses a connection the server is about to close. Command tests that pipe records through the test harness no longer race the 50 ms input pause: a debug-only setting, `THINKTHEN_TEST_INPUT_PAUSE_MS`, sets the pause, and the harness sets it to 10 seconds. The product's pause stays 50 ms, and a release binary ignores the setting.

## Evidence

- Starts from: main after 0356 lands.
  - Debt 020: Python's `http.server` replies over HTTP/1.0 without `Connection: close`. ureq-proto 0.6.4 pools such a connection, and a second send on one engine can fail as "the backend did not answer". A scratch DuckDB case failed 10 of 480 runs without the header and 0 of 480 with it. `databases/duckdb/tools/verbs_budget.py` already sends the header.
  - `git grep -l BaseHTTPRequestHandler` finds 20 code files. The servers are `databases/duckdb/tools/verbs_budget.py` (already done), `databases/sqlite/tests/conditional_backend.py`, `libraries/{ada,cobol,objective-c}/checks/backend.py`, `libraries/{cpp,dart}/…/fixture.py` (cpp fixtures, dart checks, dart flutter), `libraries/{csharp,jvm}/tests/backend.py`, `libraries/{go,php}/fixtures/backend.py`, `libraries/swift/Tests/fixtures/backend.py`, `libraries/zig/Tests/backend.py`, three servers in `libraries/python/tests/` (`test_call.py`, `test_door.py`, `test_secrecy.py`), the two probes `probes/{annotate-batching,tag-score}/measure.py`, and `sdlc/scripts/installer-test`.
  - Each binding's fixture lives in its own folder and is started by that binding's own check. No Python module is shared between bindings today.
  - Debt 030: `engine/pipeline/run.rs` closes the open request after `PAUSE` (50 ms) with no new input. The CLI's `Reader` host (`engine/pipeline.rs`) reads input on its own thread, so a stall of that thread under load can close a batch early. About 20 tests under `crates/thinkthen/tests/backend/` pin exact batch sizes or request counts through the harness's `spawn` or `spawn_file`, for example `batching.rs` `order_holds_across_jobs` (`[6] + [10; 30]`) and `batching/too_large.rs` (`[5, 3, 2]`). Feeding from a file (0352's `spawn_file`) removes the test's own blocked write but not the reader's stall.
  - `THINKTHEN_TEST_RETRY_WAIT_MS` is the precedent: `cli/edge.rs` `test_only` reads it only in a build with debug assertions, help never shows it, and the harness's `command()` sets it on every spawn.
  - Every harness spawn ends its input: `spawn` writes and drops the pipe, and `spawn_file` reads a file. End of input closes the open request at once, so a long pause cannot hang a harness test. `a_pause_sends_the_open_batch` builds its own command and holds its pipe open.
- Keeps: the 50 ms pause and every other batching rule in the product, in both debug and release builds when the setting is unset. `a_pause_sends_the_open_batch` and its three cache modes, still on the product default. Every fixture's status codes, bodies and request counting. `verbs_budget.py` unchanged. `test_call.py`'s HTTP/1.1 listener keeps its protocol version; it only gains the header.
- Changes: one header override per Python server, one pause setting for the reader host, and the records.
  - Each Python handler class without the header gains one override, so every reply, including `send_error`, says it closes:
    ```python
    def end_headers(self):
        self.send_header("Connection", "close")
        super().end_headers()
    ```
    Each override carries a comment naming Debt 020's issue. A shared helper module would need a path import across binding folders. That would couple bindings that share nothing now, so the edit is copied per file. Each touched binding's `ratchet.py.json` rises to its measured total.
  - `engine/pipeline.rs`: `Host` gains `fn pause(&self) -> Duration`, defaulting to `run::PAUSE`. `Reader` carries a pause and returns it; `reader()` takes it. `run::Bounds` gains `pause`, set from `host.pause()` in `Engine::ask_all`, and `Run` uses it where it reads `PAUSE` today. Only the reader-thread host can stall, so the setting reaches nothing else, and the engine's `Settings` and the library, door and SQL hosts stay as they are.
  - `cli/edge.rs`: `Environment` reads `THINKTHEN_TEST_INPUT_PAUSE_MS` through `test_only` and gives `input_pause()`, the value or `PAUSE`. `cli/asking/judged.rs` and `cli/annotate/asker.rs` pass it to `reader()`.
  - `tests/backend/harness/mod.rs` `command()` sets `THINKTHEN_TEST_INPUT_PAUSE_MS=10000`. Ten seconds outlasts any stall, and a mistake shows as a slow test, not a hang.
  - `a_pause_sends_the_open_batch` moves from `batching.rs` (475 nonblank lines) to a new `tests/backend/batching/pause.rs`, beside the new test.
  - `sdlc/planning/rust-standards.md` names three hidden test-only variables. `sdlc/ratchet.json` rises to the measured source total.
  - Debt 030 moves to `sdlc/issues/closed/` with `Paid:` and a `Resolution:` line. Debt 020's status says the fixtures are done; it stays open for the upstream fix and Ian's recorded default.
- Proof: one new command test, the kept pause test, and the touched surface checks.
  - `batching/pause.rs` `a_test_pause_holds_the_open_batch_until_input_ends`: with the setting at 10,000 ms, write three records, keep the pipe open 300 ms, and count zero requests at the listener. Then close the pipe and pin one request of three records and the three rows. Passing never depends on timing: no request can go out while the pause holds. On today's main the count after 300 ms is one, so the test fails before the change.
  - `a_pause_sends_the_open_batch` still passes unchanged, without the setting, proving the default pause still sends.
  - `decide_edge/help.rs` adds the new name to its help-never-shows check.
  - Python: each touched surface check runs once, one at a time under the load rule: SQLite, Ada, COBOL, C++, C#, Dart, Go, JVM, Objective-C, PHP, Python, Swift and Zig. The two probes run their self-tests in `spec`, so the build runs those two scripts' self-tests directly. `installer-test` runs once.
  - Rust: `cargo nextest` on `backend` batching tests and the help test, workspace clippy with `-D warnings`, `policy.py`, `tickets`, and `lint`.
- Defers: one library test and the product side of Debt 020.
  - `tests/public_batches/interactive.rs` `batch_one_returns_before_the_next_held_input_while_max_waits_for_close` races the pause the other way, through the library's caller iterator, which this setting does not reach. It has not failed. It gets an issue if it fails once.
  - Debt 020's product fix waits on upstream ureq-proto or a ticket for `engine/http.rs`, and Ian's resend choice stays open.

## What the build taught us

(Added before landing.)
