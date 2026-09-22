# 0070: The PostgreSQL deadline in place of single-row cancel

Date: 2026-09-22

Status: landed on branch `surfaces`; not yet merged to main.

## Result

Single-row calls on the PostgreSQL surface stop through a deadline, not a cancel. A blocking socket cannot hear `pg_cancel_backend` or a statement timeout until the network call returns, so the deadline is the enforced tool. The setting is `thinkthen.deadline_ms` (`Userset`; `-1` none, `0` spent, positive a budget), read by `call_options()` through the contract's checked `with_deadline_millis` and carried by every single-request function. A spent budget refuses before sending: SQLSTATE `57014` with `thinkthen deadline` in the message. The physics is stated in the module doc, the README, and the setting's own description. Conformance case 27 now runs instead of skipping; `runner.py` gained the deadline arm.

## Evidence

From `databases/postgresql/NOTES.md` on branch `surfaces`:

- `ok a zero budget returns the deadline kind (57014) with nothing sent`
- `ok a 50 ms budget refused in 0.13s against the 300 ms stub`
- `73 of 74 cases ok, 1 diverged (case 17, the stand-in's recorded gap)`; `check.sh` exit 0.

Landed at `9eb3cae` ("Revoke PUBLIC, carry batch errors, and give PostgreSQL a deadline").

Open follow-ups filed in the second-review issue: PostgreSQL batches ignore the deadline, and any interrupt is still treated as a cancel.
