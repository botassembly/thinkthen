# The live guard grew past its job

Status: Accepted into the prospective plan; no repair ticket exists yet

Found by Fable's adversarial review of `2c32524..4eadc02` and independently checked on 2026-09-20. Reviewers used throwaway repositories, dummy keys, and no network.

## Verdict

The foundation repair tranche was justified. Tickets 0024 through 0027 fixed an interactive hang, unbounded paid dispatch, invalid scores and labels, replay corruption, and misleading comparisons. Those are product defects.

Ticket 0034 took the wrong shape. The guard is cooperative because any process that holds `THINKTHEN_API_KEY` can bypass it. Its scripts grew from 449 to 996 lines, and 1,634 Rust proof lines test those Python scripts. The extra gate, socket, pending state, process identities, recovery, historical-launcher scan, and migration do not earn their cost for a local $20 allowance. Review rigor improved this oversized design instead of challenging the design itself. Ian can overturn this judgment.

## Confirmed faults

- A directory-sync failure can leave the charge visible while reporting failure. Recovery and retry can charge again.
- Cancellation after charge but before job start leaves pending state that blocks work until recovery.
- A fresh clone reports a held or unavailable lock when setup is missing.
- One missing or old registered worktree blocks all paid jobs without naming the path or cure.
- A failed state write can leave a temporary file containing process identities.
- Failed activation has no supported recovery.
- Production environment variables can inject test faults or wait forever.
- Reused process or session identifiers can falsely block recovery while the unrelated process remains alive. A repository move is refused with no implemented transfer.

The migration preserved all directories and branch references. It ran before GitHub turned green; the red check came from shallow history rather than a demonstrated runtime failure. Switching a detached worktree back to its preserved old branch recreates the global refusal. Current documentation gives a returning agent no practical cure.

## Test findings

The live tests require two historical commits, fail in shallow or squashed source, lack a Linux gate, and ship inside the product crate. Two assertions do not prove the ordering or grace properties their names claim. Some assertion paths leak processes. One cleanup reads a process-group identifier from mutable state and can signal a reused group.

## Other corrections

- Ticket 0033 does not cover `-.5`, `-1e-1`, or `-inf`; clap still prints its own confusing sentence.
- The durable record should say 493 distributions in recordings plus four standalone fixtures, not 497 recorded distributions.
- ADR 0022 is a 20,743-character same-day amendment to an agent-decided proposal. The repository rule requires a whole rewrite.
- ADRs 0019 through 0026 do not require Ian's acceptance. Workspace policy lets agents make reversible decisions and record that Ian can overturn them.
- `__pycache__` is not ignored, but normal execution does not prove that it will recur. The simplified design removes the Python helper.

## Decision

Replace the guard once through deletion. Keep one shared lock, validated totals, durable conservative precharge, and direct job execution. Define the ambiguous post-rename sync case instead of promising certainty the filesystem cannot give. Delete the recovery state machine and its test harness. This drops wrapper-owned interruption and recovery and its post-job charge line; direct execution gives signals and exit status to the job. Begin local `annotate` work immediately; its paid probe and live recordings wait. The prospective plan holds the remaining order.
