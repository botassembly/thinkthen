# PostgreSQL check pins its result lines

Quick Fix `qf/postgresql-check-exact-lines`, 2026-09-27.

- Starts from: Issue `sdlc/issues/closed/2026-09-26-postgresql-check-has-passes-any-multi-line-needle.md` and source baseline `3c409cbc`. On that baseline, the actual `has` function accepted wrong output for a needle beginning with a newline, accepted one line of a two-line needle, and accepted `13` for the `3\n` needle after command substitution. `target/codex-logs/postgresql-check-red.log` holds the local reproduction.
- Keeps: The PostgreSQL extension, its SQL results, all other check steps, and the local fixture and loopback backend behavior are unchanged.
- Changes: `has` and `hasnt` now match a whole literal needle. A check step rejects partial multiline matches. The five issue-named steps pin the final or first answer line, and the total-limit step pins its complete two-line result.
- Proof: `bash -n databases/postgresql/check.sh` and a direct matcher negative proof passed. The selected local PostgreSQL run passed seven steps with zero failures: `a_multiline_needle_must_stay_whole`, `dev_zero_refuses_fast`, `a_timed_out_batch_leaves_the_session_working`, `a_changed_limit_rebuilds`, `the_total_holds_across_rows`, `a_panic_is_an_error`, and `the_fake_key_stays_in_the_environment`. The native check used `flock -o` on the shared heavy lock and a temporary PostgreSQL cluster with a loopback backend. Its log is `target/codex-logs/postgresql-check-selected.log`.
- Defers: The full install, lint, test, spec, and surfaces ladder and code review remain for landing. No paid or live backend call ran.
