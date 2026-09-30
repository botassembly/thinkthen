# The process-cap batching test races two records

Status: closed 2026-09-30. Replaced by ticket 0317, which runs the case at `--jobs 1`.

## What happens

`crates/thinkthen/tests/backend/batching/ceiling.rs` `a_later_ordinary_command_request_is_refused_by_the_process_cap` sends two records at the default `--jobs 4` with `--max-requests-total 1`. It expects record 1 to take the one send and record 2 to be refused. Record 2 sometimes reserves first, and the command prints `stopped at record 1; 0 records finished`. One run in eight failed this way on 2026-09-30 while ticket 0308 was being built. The address is loopback, so the pacer is not involved.

## What would fix it

Run the case at `--jobs 1`, or assert whichever record the budget refused. A fix belongs to whoever owns the test rung next.
