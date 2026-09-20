# ADR 0022: A live job spends its reservation before it starts

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

The live script guards paid development calls with one repository ledger. It currently checks only whether earlier spend has reached the limit. It neither reserves room for the next job nor prevents two jobs from reading and replacing the ledger together. Its later file scan is not authoritative: a repeated request can be paid and leave an immutable recording unchanged, and the plan already holds 3,207 tokens that had to be added by hand after a job wrote no usage record.

## Decision

- The invocation is `sdlc/scripts/live --max-tokens N JOB [ARG...]`. `N` is a positive canonical decimal no greater than 999,999,999. It is the most input-token budget the job is authorized to consume.
- The ledger accepts exactly one positive `limit_tokens` and one `spent_tokens`, each a canonical decimal no greater than 999,999,999, with spend no greater than limit. Blank lines and comment lines are allowed. Other data, duplicate fields, leading zeroes, and out-of-range values are refused.
- One atomic lock directory at `sdlc/live-tokens.lock` is held from the ledger read until the job ends. Another invocation refuses before building or running anything.
- The script refuses when `N` is greater than the remaining budget. An exact fit is allowed. Before any job process starts, it atomically adds the whole reservation to `spent_tokens`. That charge is never refunded automatically. A crash, signal, missing recording, repeated immutable entry, deleted file, or accounting failure therefore cannot reopen spent authority.
- The newer-JSON scan remains an informational measurement. It prints what it found and warns when that number exceeds the reservation. It never reduces or increases the durable charge. A failed job retains its own nonzero status. A successful job whose measured use exceeds its reservation exits 1.
- The lock contains a small owner file with the wrapper PID, job path, reservation, prior spend, charged total, and state. It holds no key. The script never repeats its contents in a diagnostic.
- On HUP, INT, or TERM, the wrapper forwards the signal to the job shell, waits until that child stops, reports any measurement it can make, removes its temporary files and lock, and exits 129, 130, or 143. If the child remains alive, the wrapper keeps waiting and the lock stays held.
- An uncatchable stop can leave the lock. Recovery first reads the owner file and verifies that the ledger's `spent_tokens` is at least its `charged_total` when the state is `charged`. If the state is earlier or the ledger is lower, recovery leaves the lock and repairs the ledger to the recorded charged total. The person then checks the recorded wrapper PID with `ps -p PID -o pid=,command=` and removes the exact empty lock with `rm sdlc/live-tokens.lock/owner && rmdir sdlc/live-tokens.lock`. The charge remains.

## Failure order

Input, job-path, ledger, and key refusals happen before a build or job. A held lock also refuses before either. Build failure releases an uncharged lock. Failure to prepare or atomically install the conservative charge leaves the lock when the ledger state is uncertain and starts no job. After the precharge succeeds, no later bookkeeping failure can understate authorized spend.

The job's nonzero status wins over an over-reservation warning or informational scan failure. An otherwise successful job returns 1 for either condition. Signal status wins after the child stops. Every path keeps the conservative charge.

## Limit of the guarantee

The ledger now limits the sum of maxima authorized through this script. It does not know the vendor's token count before a response and cannot stop one job that violates its declared maximum. Such a breach is reported after the irreversible call. Job authors choose a conservative maximum, and no automatic refund depends on recordings or job-controlled files.

The fixed numeric ceiling keeps every subtraction and addition inside signed 32-bit shell arithmetic while exceeding the current 476,000,000-token project limit. Raising the project limit beyond that ceiling requires changing this decision and its boundary tests.
