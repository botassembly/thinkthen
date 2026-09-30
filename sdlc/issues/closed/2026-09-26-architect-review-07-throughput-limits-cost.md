Status: closed 2026-09-30. Items 1, 2, 3 and 5 fixed by tickets 0308, 0225, 0315 and qf-command-edges-and-prune. Item 4 stays in the severity 3 roll-up.

# Architect review 07: throughput, rate limits, retries and cost

A fresh reviewer tested concurrency, rate limits, retries and cost reporting as an architect who sizes a batch job. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 2 severity 2 and 12 severity 3 issues. The full detail sits in the architect review report 273, 07.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. There is no rate limit, so the default can exceed the vendor's documented rate, and a throttled run stops instead of slowing down (severity 2)

Evidence. The throttle counts only in-flight attempts (`engine/mod.rs:339-362`). Live, the default `--jobs 4` sent 1,519 requests a minute, and `--jobs 16` sent 4,320 a minute. On loopback, `--jobs 3` with 100 ms replies sent 1,285 a minute. That breaks the spec's "`--jobs 3` stays under" advice (`records.md:131`), which the site's reference page repeats. When 429s persist, a record makes 3 sends over about 3 s, and then the whole run stops at exit 4. Reproduced: 12 × 429 after 20 good records gave "stopped at record 21; 20 records finished". Concurrent processes add their rates together.

What an integrator hits. A long batch job at the defaults runs fine until the vendor starts enforcing the limit, and then it dies partway through. The only documented control is a concurrency number whose effect on rate changes with record length, network and vendor load.

Direction. Add a requests-per-minute pacer beside the concurrency cap, defaulted from the backend's published limit. Size retries for a sustained limit, not a short blip. Reword the `--jobs 3` advice as "about 3 at roughly 200 ms replies". Ticket 0155's gate helps within one process only.

## 2. A command's exit can wait with no bound on another process's usage lock, and the spec does not say so (severity 2)

Evidence. `cli/mod.rs:90` calls `finish()`, which waits for the writer (`engine/usage.rs:121-128`). The writer blocks in `File::lock` with no timeout (`engine/usage.rs:263`). Reproduced: `decide` printed `true` at once and exited after 11.73 s while an outside process held `.lock` for 12 s. `recording.md:26` says only that "counting never holds back a request". ADR 0049 item 3 admits the wait. The spec does not.

What an integrator hits. `if thinkthen decide ...` carries its answer in the exit code, and that exit stalls as long as any process holds the lock. A suspended (Ctrl-Z) ThinkThen that is mid-write, or a slow or network home filesystem, can hold it. This is a hang in the worst case. The reviewer rated it 2 because it needs a stuck lock holder.

Direction. Give the final wait a short bound and a warning, and say in the spec that exit waits for the usage write.

## 3. Retries move in lockstep with no jitter (severity 3, carried here for two reviews)

Reviews 01 (issue 5) and 07 (I5) both found this. On loopback with 429 on every request, the run died at exit 4 after 3.03 s and 12 requests, and all four workers retried within 2 ms of each other at 1 s and 3 s (`engine/http.rs:118-160`). Eight resends landed within 3 ms at +1.1 s. Every worker backs off on the same schedule and returns together, so the burst repeats. Direction: share one backoff across workers once any 429 arrives and add jitter, or build 0155's gate so that the gate reopens once.

## 4. One slow or retrying record stalls the whole run (severity 3, carried here for two reviews)

Reviews 01 (issue 4) and 07 (I7) both found this. `engine/schedule.rs:124-130` counts dispatched but unprinted rows against `jobs`. A 3 s `Retry-After` at `--jobs 16` sent nothing for 3 s, and one 4 s reply turned a 0.2 s run into 4.2 s. In review 01, the server held one reply for 8 s over 41 records at `--jobs 4`, and it received only 4 requests in the first 8 s. The stall bound is `--timeout` for each of up to three attempts, plus retry waits. Direction: document it beside `--jobs`, and consider a larger reorder window or an unordered mode that tags each row with its input position.

## 5. The release binary honors `THINKTHEN_TEST_RETRY_WAIT_MS` (severity 3, carried here for two reviews)

Reviews 06 (I-12) and 07 (I13) both found this. The variable is read at `cli/edge.rs:65`. `sdlc/scripts/settings:29,113` exempts the `THINKTHEN_TEST_` prefix, so no settings row exists. Set to 0, it sent 51 attempts to a 503 server in 50 ms. Set to 1, it made six attempts in 58 ms. `THINKTHEN_TEST_SIGINT_ACK` is also read in product code. Direction: compile test hooks out of release builds.

## Severity 3 titles

- A `Retry-After` longer than `--timeout` or 60 s is cut short.
- `Retry-After: 0` or `retry-after-ms: 0` resends at once with no floor: 11 sends in 15 ms.
- There is no jitter. Carried above as item 3.
- Every retry opens a new connection, because the error body is never drained (`engine/http.rs:249-262`).
- One slow or retrying record stalls the whole run. Carried above as item 4.
- `--dry-run` cannot estimate cost: it gives no request count, token estimate or price.
- There is no run total, and filtered-out spend is invisible. See `2026-09-26-every-surface-should-give-back-run-facts.md`.
- Retries cannot be told apart from first sends. See ticket 0155.
- The libraries and SQL surfaces cannot set the timeout or the retry count. See `2026-09-26-settings-some-surfaces-cannot-reach.md` item 1.
- Nothing caps concurrency or spend across processes or connections, and `--jobs` has no environment or config tier.
- The shipped binary reads `THINKTHEN_TEST_RETRY_WAIT_MS`. Carried above as item 5.
- The final 429 message does not say retries happened. Carried in the architect review 06 file with the other refusal phrases.
