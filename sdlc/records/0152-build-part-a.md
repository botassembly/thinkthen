# 0152 Part A: A huge deadline prints a short number

Status: built; waits for its code review. Owner: Claude.

Branch `ticket/0152-two-wording-fixes`, built in lane `thinkthen-lane-2`. The ticket is `sdlc/tickets/0152-two-wording-fixes.md`. Part B waits for ticket 0146, and the ticket stays open for it. Ian can overturn every decision the ticket lists.

## Result

- `CallOptions::deadline_seconds` formats its refused value through `shown` in `crates/thinkthen/src/public/options.rs`. The plain form stays when it holds at most 20 characters. A longer value prints in exponent form. `1e300` now refuses with `a deadline of 1e300 seconds is not -1, 0, or a positive budget of at most 4294967295 seconds`, 92 characters. Before, the number held 301 digits.
- `deadline_millis` and `deadline_after` did not change.
- `public_controls.rs` pins five new rows as written literals: `1e300`, `-1e-300`, `f64::MAX` (`1.7976931348623157e308`), and the two boundary rows `1e19` (`10000000000000000000`, 20 characters, plain) and `1e20` (`1e20`, whose plain form holds 21 characters). The code review asked for the boundary rows. The existing rows still pin `-0.5`, `-2`, NaN, the infinities, and `4294967296`.
- `libraries/python/tests/test_inputs.py` pins `UsageError a deadline of 1e300 seconds ...` in place of the 301-digit form. No other surface pins the sentence. Ruby and R check only the kind. TypeScript, C, and SQLite refuse with their own sentences.
- Item 1 of the wording issue is marked fixed. Item 16 stays open for Part B.

## Plants

Each plant ran against the built code, turned its test red, and was restored from a saved copy in the scratchpad. The restored file was touched, and the diff holds no plant text.

| Plant | Change | Test | Result |
| --- | --- | --- | --- |
| P1 | `shown` always returns the plain form, as `{value}` did | `public_controls.rs` `deadline_numbers_follow_the_host_table_and_the_last_call_wins` | RED: the `1e300` row read 301 digits |
| P1 | the same | Python `test_inputs.py::test_deadlines_follow_adr_0041`, through `libraries/python/check.sh` against the loopback backend | RED: index 6 read 301 digits |
| P2 | `shown` always returns the exponent form | `public_controls.rs`, the same test | RED: the `-0.5` row read `-5e-1`. The ticket named the `4294967296` row; the loop meets `-0.5` first |
| P3 | the cutoff reads `<= 300` | `public_controls.rs`, the same test | RED: the `1e20` row read `100000000000000000000` |
| P4 | the cutoff reads `< 20` | `public_controls.rs`, the same test | RED: the `1e19` row read `1e19` |

## Budgets

Measured with nonblank lines against `origin/main`.

- `crates/thinkthen/src`: 11 added, 1 removed, net 10. The budget is 10.
- `crates/thinkthen/tests`: 13 added. The budget is 12. The code review asked for the two boundary rows, which take the last line and one more. The coordinator directed the change.
- `libraries/python/tests`: 1 changed. The budget is 1.

## Ratchet

`sdlc/ratchet.json` rose from 69192 to 69215, 23 lines: 10 in `public/options.rs` and 13 in `public_controls.rs`. The nearest existing formatter is `core::measure::python_float_text` (`crates/thinkthen/src/core/measure.rs:339`). It was not reused, because it prints Python's spellings: `4294967296.0`, `1e+300`, and `NaN.0`. The refusal keeps Rust's plain `4294967296` and `NaN`. No other formatter in `public/` or `core/` picks the shorter form, so nothing was deleted first.

## Ladder

Run in lane 2 with `THINKTHEN_API_KEY` unset and `THINKTHEN_PRIVATE_NAMES` set, each rung once at `8912c8ff`, after the merge of `origin/main` at `ee8a815f`:

- `install`: exit 0.
- `lint`: exit 1 on the first run, for the ratchet alone (69213 against 69192). Exit 0 after the ceiling commit.
- `test`: exit 0.
- `spec`: exit 0. demos: 21 green, 0 red.
- `surfaces`: exit 0. Every landed surface passes, `libraries/python` included, and the release smoke passes.

`origin/main` then moved to `3ff5ea9d`, with changes under `site/`, `.claude/`, and `sdlc/issues/` only. The branch merged it at `accd4315`, and `lint` ran once more after that merge. After the code review's two rows, `lint` and `test` ran once more.
