# 0311: The token cap on every surface

Status: landed. Lane claude-4. Branch `ticket/0311-token-cap-everywhere`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8. Issue: `sdlc/issues/2026-09-29-token-cap-contract-before-release.md`.

## Outcome

Every surface enforces the estimated input admission total from ticket 0299 with no port code. A user sets `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL` once. The command, `EngineBuilder::from_env()`, and so every port and SQL extension read it. A request that would pass the limit is refused before it is sent.

## Current state

- The core, the command flag, and the C settings key landed in ticket 0299. Ticket 0300 added prices on the same path.
- Every port and SQL extension builds its engine from `EngineBuilder::from_env()`: Python (and its Polars and pandas calls), R, Ruby, TypeScript, C and the ports over C, SQLite, DuckDB, and PostgreSQL. The Rust Polars door uses the caller's own engine.
- `from_env` reads no token limit today. No port sets one. So only the command, Rust builder users, and C JSON callers get the cap.
- Ticket 0308 showed the thin path: a variable read in `from_env` reaches every port with no port code.

## Setting

- Name: `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL`. A whole number of 0 or more. Blank is unset.
- Precedence: an explicit setter wins. The command flag, the Rust builder setter, and the C JSON key (including `null`) outrank the variable.
- Refusal: the existing Usage sentence from ticket 0299 on each surface. A malformed value is a Usage error before any send.
- Scope: unchanged from ticket 0299. One process count; each PostgreSQL backend is its own process.
- No new setter or SQL setting in any port. SQLite keeps refusing the key in its own settings JSON.

## Evidence

- Starts from: main `0b1f29812`; ticket 0299's core admission gate, command flag, and C key; ticket 0308's `from_env` reach to every port; the issue's host list.
- Keeps: the 0299 estimator, gate, refusal sentences, scope, and exit codes; the request cap; every port's constructor and SQL settings; the C JSON key and its `null`.
- Changes: `from_env` and the command edge read the variable through one parser in `engine/send_budget.rs`; the command flag outranks it; the settings page row names the variable on every surface.
- Proof: per surface with an installed toolchain, one loopback case sets the variable, makes a call, and pins the Usage refusal with zero listener arrivals. The command and Rust cases also pin the malformed sentence, and the command case shows the flag outranks the variable.
- Defers: durable library and SQL spend in `thinkthen status` (see below); a limit across processes or PostgreSQL connections; per-port setters and SQL settings; installed package and release qualification.

## Deferred: spend in `status`

The status issue (`sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`, option 1) asks library and SQL engines to persist counts to the command's usage store. That widens what the library writes, so the issue asks for an ADR first. ADR 0111 slices 2 and 3 rework the cache and usage accounting. Writing library counts now would build on code that slice will replace. It stays open for a ticket after those slices.

## Build result

- Source: `engine/send_budget.rs` holds one parser, `estimated_total`. `public/settings/environment.rs` and `cli/edge.rs` call it. `cli/asking.rs` lets the flag outrank the variable. No port or SQL extension source changed.
- `cli/edge.rs` stays at 499 nonblank lines by merging two `use` lines and passing the read value straight to the parser.
- Tests, one case per surface, each pinning the refusal and zero listener arrivals:
  - Command, `tests/backend/limits.rs`: refusal, the malformed sentence, and the flag outranking the variable with one arrival.
  - Rust `from_env`, `tests/public_env/batch.rs`: four refused calls and the malformed sentence.
  - C, `libraries/c/tests/door/settings.rs`: code 1 refusal; a JSON `null` then admits exactly one arrival.
  - Go over C, `thinkthen_test.go` run from `fixtures/run_matrix.py` against a fresh counted backend.
  - Python, `tests/test_call.py`: a scalar and a Polars column call, `UsageError`.
  - R `tests/facts.R`, Ruby `tests/test_engine_settings.rb`, TypeScript `tests/settings.test.mjs`: each port's usage error with the core sentence.
  - SQLite `tests/test_settings.py`, DuckDB `tools/settings_suite.py`, PostgreSQL `check.sh` step `token_variable_refuses_before_sending`: a statement error with the core sentence.
- Red: the command and Python cases failed with the variable read removed.
- Not run per wrapper: the other ports over C (PHP, Swift, Zig, C++, C#, JVM, Objective-C, Ada, COBOL, Dart). They pass JSON to the C constructor, which starts from `from_env`. Ticket 0299 rules out repeating the case in every wrapper.
- Pages: the settings row names the variable on every surface; `backends.md` and the changelog gain a sentence; the token-cap issue records the rollout and stays open for package qualification.
- Growth: root Rust 105,252 to 105,327 (about 15 source lines, the rest tests). Host ratchets raised to measured totals: C 4,089, Python 4,568, R 2,353, Ruby 2,324, TypeScript 1,183, Go 1,242 and 596, SQLite 2,081, DuckDB 3,904. I checked `backoff::per_minute` for reuse; its range and sentence differ, so the parsers stay separate.
- Checks: `cargo nextest run -p thinkthen` 1,246 passed; clippy clean on changed files; `policy.py`; `sdlc/scripts/settings` and its self-test; `sdlc/scripts/tickets`; SQLite `check.sh` in full; each other surface through its harness steps.
- Quick fix on the way: SQLite `check.sh` counted `catch_unwind(` calls, which ticket 0306 moved into the shared `contained` guard, so its gate stopped before any test. It now counts `contained(`.

## Code review

ACCEPT with four fixes, all made before landing: the SQLite gate matches the guard call `contained(body)`; the settings row says the command flag cannot clear a limit the variable sets; the Rust case pins `max_estimated_input_tokens_total(None)` outranking the variable with one arrival; the issue says why the variable replaces 0299's per-host SQL spellings. After merging main, `sdlc/scripts/test` passed (1,269 tests), with `policy.py`, the ticket checker, and SQLite `check.sh`.

## What the build taught us

- The 0308 lesson held: one read in `from_env` put the cap on eleven surfaces with no port code. Each port's existing error mapping carried the core Usage sentence unchanged.
- A variable is the thin form for an engine-wide setting. A per-port setter would have meant eleven constructors, eleven schemas, and eleven reviews for the same effect.
- The PostgreSQL variable lives in the server process environment. A DBA sets it for the whole server, and each backend still counts alone.
- Surface gates can go stale when a shared refactor lands. SQLite's source-shape check broke after ticket 0306 and hid every later SQLite test from its gate.
