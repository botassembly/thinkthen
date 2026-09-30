# 0311: The token cap on every surface

Status: building. Lane claude-4. Branch `ticket/0311-token-cap-everywhere`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8. Issue: `sdlc/issues/2026-09-29-token-cap-contract-before-release.md`.

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
