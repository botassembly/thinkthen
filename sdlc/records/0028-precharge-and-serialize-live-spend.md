# 0028: Precharge and serialize live spend

Branch `ticket/0028-enforce-live-spend`. Built 2026-09-20.

## What landed

The live-call script now requires `--max-tokens N`, locks the shared ledger, and permanently charges the approved maximum before a paid job can start. A second invocation refuses while the first owns the lock. Recording files remain an informational measurement and can never return spending authority.

The script validates the key before repository lookup or any external command. It then removes the key from the wrapper environment. After the charge and gate PID are durable, the wrapper sends the key through a FIFO to one tracked gate. The gate releases the job only after receipt and a final signal check. Build and bookkeeping commands never receive the key.

The lock records enough safe state for manual recovery. Catchable signals keep the full charge, forward to the child, wait until the child is confirmed gone, remove safe temporary state, and preserve the first signal status. An uncertain write or uncatchable stop leaves the lock closed for recovery.

The approved project limit and recorded spend remain 476,000,000 and 429,118 tokens. Future live jobs add their declared maxima rather than measured recording totals.

## Red then green

The original script allowed an arbitrarily large job with one token remaining, raced concurrent writers, and rebuilt its spend from mutable observations. Local fake jobs now prove exact-fit charging, refusal above the remainder, permanent charges after failure or missing, repeated, deleted, and backdated usage files, and refusal of a concurrent invocation.

Input cases cover the old invocation, every numeric edge, damaged ledgers, missing jobs, blank and line-breaking keys, exact preservation of accepted key bytes, build and write failures, and a successful job that exceeds its reservation. No test opens a network connection.

Lifecycle cases cover HUP, INT, TERM, repeated signals, build and child-start boundaries, key delivery, child receipt, normal execution, and bookkeeping. The final FIFO test stops the tracked gate before it opens its reader, sends TERM and then HUP to the wrapper, and proves that the wrapper and lock remain until the gate resumes and exits. Every such case has a five-second hang bound.

## Review

The design reviewer rejected the first proposal because a refund from the recording scan recreated the original undercount. The accepted design uses a permanent conservative charge, bounded canonical numbers, a durable owner record, and fail-closed recovery. It classified the shared paid state and interruption proof as level 4 and accepted the Sol Medium route.

The code reviewer rejected four implementation passes. The findings covered missing filesystem synchronization, key exposure to helper commands, ineffective INT forwarding, an unrecorded key-bearing process, changed trailing-newline path bytes, blocking FIFO opens around signals, an orphaned FIFO helper, and an interrupted wait that could remove the lock while its gate lived. The final implementation uses a helper-free Linux FIFO handoff and one interrupt-resistant wait loop on every child path. The same reviewer accepted the final diff with no remaining finding.

## Choices made where the ticket was silent

Ian can overturn these choices.

- **The ledger charges authority rather than observed use.** A reservation never returns automatically. This favors a safe upper bound over maximum use of the approved total.
- **The FIFO uses Linux read/write-open behavior.** The wrapper opens the FIFO read/write before spawning the gate. The checked Linux shells complete that open without another endpoint, so no later wrapper open can block after a signal. Dash, Bash, and Zsh passed the probe.
- **Wrapper INT becomes child TERM.** POSIX asynchronous shells may inherit ignored INT. TERM reliably stops the job while the wrapper still returns 130.
- **Keys containing a line feed are refused.** POSIX line input cannot preserve that byte. Other environment bytes, including leading and trailing spaces, backslashes, tabs, and carriage returns, pass unchanged.

## Gates and size

The source ceiling is 16,715 measured Rust lines, up from 15,732. Most growth is the local-only ledger, concurrency, signal, recovery, key-confinement, and FIFO lifecycle proof. The production change is one shell script and its living invocations. The reviewer and implementer looked for shared wait and handoff paths; the final script uses one child-wait function and one gate.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `16715/16715` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0, 16 green how-tos and 4 expected red pages |

The coordinator ran the ladder with the key and base address unset. No live call ran.
