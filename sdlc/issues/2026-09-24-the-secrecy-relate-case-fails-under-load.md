# The secrecy relate case fails under load

Status: Open. Filed by Quick Fix `qf-heavy-lock` on 2026-09-24 (`sdlc/records/qf-heavy-lock.md`).

`secrecy::no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` failed once at `tests/backend/secrecy.rs:235` for case `a-record-run-relate---lines []`. The command exited 4 with "the backend refused the connection" where the case expects exit 0. The other 357 tests in the `backend` binary passed.

Observed on 2026-09-24 on the Beelink at `34c13660`, a change to shell scripts and prose only. The one-minute load was 120. Several processes from another session each held more than one core.

The case likely needs its loopback listener to accept before the command connects, or a bounded wait for the listener. Nobody has reproduced it on an idle machine.
