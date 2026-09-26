# Quick Fix qf-review-273: refuse a bad PostgreSQL throttle where it is set, and state the current key rule in ADR 0004

Status: built and checked, waiting for landing. Owner: Claude. Base: `origin/main` at `36014ad1`. Lane: `thinkthen-lane-3`. A fresh read-only Opus review is in `sdlc/records/qf-review-273-review.md`.

## Evidence it started from

- Local experiment 273, report 04, item I4. The PostgreSQL README said the throttle ran from 0 to 32. `thinkthen.throttle` accepted 0, and every later call then failed with the engine's sentence `a throttle is a whole number from 1 through 32`. `sdlc/issues/2026-09-26-architect-review-04-libraries-and-databases.md` item 2 holds it.
- Local experiment 273, report 12, item 2.3. ADR 0004's status line said "The rest stands", and its decision still said a key never crosses hosts. ADR 0010 and `specification/backends.md` replaced that rule. `sdlc/issues/2026-09-26-architect-review-12-security-and-data-boundary.md` item 3 holds it.

## Retained behavior

- -1 stays the unset throttle, so an unset setting leaves the engine's own throttle. The first explicit throttle still holds for the backend's life.
- The throttle stays a superuser setting.
- The command, the libraries, SQLite, and DuckDB are unchanged.

## Change

1. PostgreSQL registers `thinkthen.throttle` with a check (`databases/postgresql/src/ffi.rs`). The check refuses any value other than -1 and 1 through 32. The refusal reads `thinkthen usage: a throttle is a whole number from 1 through 32 (retryable: no)`, the sentence the other surfaces show. `SET` fails with SQLSTATE 22023. A configuration file's or `ALTER ROLE`'s bad value draws the same sentence as a warning when the library loads, and the setting stays -1, so calls keep working. The setting's range widened to the full integer range, so the check alone judges and 0, 33, and -2 read one sentence. The cost is that `pg_settings` shows the integer bounds as `min_val` and `max_val`. Ian can overturn that choice. The README row now says 1 to 32 and names the refusal.
2. ADR 0004's status line names ADR 0010 as the replacement for the four-value profile and the key rule. A new amendment of 2026-09-26 states the current rule. The key comes from `THINKTHEN_API_KEY` alone. It goes only to the address the user named, or to the default base when the user names none. The amendment also marks the profile paragraph as history under ADR 0032.
3. Both architect review issues record their fixes. Review found that the check script's `has` passes any needle that holds a newline. `sdlc/issues/2026-09-26-postgresql-check-has-passes-any-multi-line-needle.md` records the five older steps it weakens.

## The other surfaces

- SQLite has no such hole. `thinkthen_throttle(0)` and `thinkthen_throttle(33)` refuse at the setter call with the same sentence, and `databases/sqlite/tests/test_settings.py` pins it.
- DuckDB keeps the same shape as the old PostgreSQL behavior, and this fix cannot reach it. DuckDB has no check step for an extension setting. `SET thinkthen_throttle = 0` succeeds, and the next call refuses with the same sentence and sends nothing. `databases/duckdb/README.md` says so, and `databases/duckdb/tools/settings_suite.py` pins it. That gap stays deferred.

## Proof

`throttle_setting_range` in `databases/postgresql/check.sh` is an outside-in regression. It starts the server with `thinkthen.throttle = 0` in the configuration file, and a call must answer `t` after the warning. It then sets 0, 33, and -2 with `SET`, each after `LOAD`. Each must fail with `ERROR:  22023:` and the sentence, and the next call must answer `t`. The loopback backend must count 4 sends.

It failed before the fix. The configuration file's 0 reached the call, which read `ERROR:  thinkthen usage: a throttle is a whole number from 1 through 32 (retryable: no)` and no warning. It passed after the fix.

The four questions. It protects the rule that a bad throttle is refused where it is set and calls keep working. Removing the check or narrowing the range back to -1 through 32 fails it. No earlier step set a bad throttle. It needs no test-only hook, because it drives a real server through `psql`.

## Size

`databases/postgresql/ratchet.json` rises from 1,817 to 1,855. The growth is the guarded check and its registration in `ffi.rs`, the one module allowed unsafe code, and the refusal helper in `call.rs`. No other setting check or copy of the sentence existed in `databases/postgresql/src` to reuse. `sdlc/ratchet.json` does not change.

## Checks

With `THINKTHEN_API_KEY` unset. `sdlc/scripts/live` did not run, and no paid call was made. The rungs ran at `8a985e0a`, with the second-pass issue fix uncommitted in the worktree. That fix touched only the issue page.

- `lint` with the private-names list: exit 0.
- `test`: exit 0, 956 passed, 0 failed, 13 ignored across 37 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `databases/postgresql/check.sh` at `97cabd87`: exit 0, `postgresql: 53 passed, 0 failed`, conformance 43 passed, 0 failed, 11 not run. Later commits changed only records and issues.

## Deferred

- DuckDB accepts a bad throttle at `SET` and refuses at the next call, because DuckDB has no check step.
- The five weak `has` steps in the PostgreSQL check wait on their issue.
- The repository `CLAUDE.md` says the key goes only to the address the user named, without the default base. Ian distributes that file, so this fix leaves it alone.
