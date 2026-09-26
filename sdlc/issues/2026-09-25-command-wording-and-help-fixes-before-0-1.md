# Command wording and help fixes before 0.1

Status: Open. Only item 1 remains.

Ian's ruling, 2026-09-25, on item 6: `check` shows the model, the provider, the URL, and all its outputs. When no model name is given, it prints "unspecified". It shows the model the user asked for and the model each reply names. Option 2, an optional model in every request, is not ruled.

This issue merges nine issues. They were `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, and `2026-09-25-command-wording-and-help-fixes-before-0-1.md`. Each one fixes a sentence a user reads: a refusal, a help line, or a printed report. None changes what the engine answers. They share one goal. The words must match the behavior before the 0.1.0 release. All evidence below was checked against main at `a95474be` on 2026-09-25.

## 1. A huge deadline prints hundreds of digits

What the user sees. `CallOptions::deadline_seconds` refuses a budget past 4,294,967,295 seconds. Its sentence formats the raw `f64` with `{value}` (`crates/thinkthen/src/public/options.rs:129`). Rust's `Display` for `f64` never uses an exponent. A deadline of `1e300` prints a 301-digit number. The milliseconds sentence at `options.rs:156` has the same shape. Every surface that passes a user's deadline through prints the same sentence. The Python test pins the long form: `libraries/python/tests/test_inputs.py:84` expects `f"UsageError a deadline of {10**300} seconds {budget}"`.

What the ruling says. ADR 0041 sets the cap and the refusal. It does not ask for the full digits.

The fix. Print the value in a short form when it passes the cap, such as `{value:e}` (`1e300`), or name only the cap. Keep NaN and infinity as they read now. Update the Python test and any engine test that pins the long form.

Done when: a `1e300` deadline refuses with a sentence under 120 characters, and the Python and engine tests pin that sentence.

## 2. The help lists admin commands before the ten functions

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

## 3. `relate --either` with a question file names the wrong cause

Fixed by ticket 0123, landed 2026-09-25 from branch `ticket/0123-relate-fits-the-backend`.

## 4. relate help lists `--jobs`, and relate refuses it

Fixed by ticket 0123, landed 2026-09-25 from branch `ticket/0123-relate-fits-the-backend`.

## 5. The engine's WidthActive message still says width

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

## 6. check prints the model it sent, not the model that answered

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

## 7. recognize `--dry-run` help promises the request and shows no size

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

## 8. recognize help carries no cost sentence

Ticket 0123 fixed the relate half. Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

## 9. `--jobs N` opens one connection per request in flight

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

## Already fixed

- `thinkthen --version` prints the version. Ian's 2026-09-24 ruling asked for it. `crates/thinkthen/src/cli/args.rs:23-25` adds the flag, and `crates/thinkthen/src/cli/mod.rs:46-47` prints `CARGO_PKG_VERSION`. The crate and the Python library both carry `0.0.1`, which the ruling allows until launch. The command order part of that issue stays open as item 2.
- Connection reuse across a batch, the code answer that issue 9 proposed, already exists. The shared pool in `crates/thinkthen/src/engine/http.rs:54-59` reuses connections between requests. The missing sentence stays open as item 9.
