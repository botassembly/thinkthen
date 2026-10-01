# A process that forks after its first call hangs, in Python and in Ruby

Status: Closed on 2026-09-25 after a check against main. Ticket 0096 landed fork recovery (merge 9527d661, sdlc/records/0096-land-fork-recovery.md). Earlier status: Open. Found in the library experiments. It belongs in the rewrite of ADR 0017 before any library ships.

A survey read local experiment 205 in the workspace on 2026-09-20. Five of the six language experiments are finished, and C is not. The Python and Ruby builders each tested a fork after the first call, and both hung. The Python child hung past 30 seconds. The Ruby child hung forever against the wire, and looked fine against the null backend. A fork test that uses the null backend alone proves nothing. JavaScript has no fork in its model, and the R experiment reports no fork test.

## Why it happens

The stand-in engine keeps one process-wide runtime with worker threads. A fork copies the calling thread alone. The child inherits a pool whose workers do not exist, and a lock one of them held stays locked. The Python builder found that `os.register_at_fork` cannot repair it, because the engine offers no way to reset that state.

The real engine is a blocking client with a set of threads and no async runtime, read from `Cargo.toml` on 2026-09-20. The same hazard applies to any process-wide pool of threads or connections. It is unchecked against the real engine.

## Who forks

Python's `multiprocessing` with the fork start method, a web server that preloads the app and then forks workers, Ruby's forking servers, R's `parallel::mclapply`, and every PostgreSQL backend. The data audience does this daily. `sdlc/planning/databases/postgres.md` already rules that nothing is built before the fork.

## My recommendation

Fix it once, in the engine. The engine records the process ID when it builds its pool. Each call compares the current process ID with the recorded one and builds a fresh pool when they differ. It never touches the inherited one. Every surface then gets the fix with no code of its own, and that fits the rule that no rule lives in a binding. Each library's conformance run adds one case: fork after a call, call again in the child against the stub's wire, and require an answer.

The library documentation says nothing about forking until that case is green. The marketing deck plan carries the same rule.

## Smaller findings from the same survey

- Ruby's interrupt lands only when the call returns. Python, JavaScript, and R stop promptly. The chunked bulk form is what makes Python's interrupt prompt: one single crossing was blind for 8 seconds.
- An R binary built on the host failed on a clean image with a C library mismatch. A binary built on the target's own base image installed with no Rust toolchain. The release matrix for R needs that.
- Python's async bridge had to start a second runtime because the engine hides its own. With a blocking engine the question changes shape, and the ADR should say how `async` callers are served.
- A dead stub with default retries turned a bench run into a hang of many minutes. A bench harness needs a short retry budget.
- Every finished binding reached the engine's full width through its bulk form, 32 requests in flight, with the same wall clock as pure Rust. One call through a binding cost between 2 and 45 millionths of a second. The machine was busy, and the JavaScript and Ruby builders call their per-call numbers noisy.

The full harvest of experiment 205 waits for the C experiment and for the experiment's own `FINDINGS.md`, which is an empty template today.

## What Ian can overturn

The recommendation. The process ID check is one design among several, and the ADR decides.
