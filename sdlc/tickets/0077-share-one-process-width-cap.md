---
flow: build
priority: 77
opens: crates/thinkthen/src/engine/http.rs crates/thinkthen/src/engine/mod.rs crates/thinkthen/src/engine/request.rs crates/thinkthen/src/engine/schedule.rs crates/thinkthen/src/engine/annotate_schedule.rs crates/thinkthen/src/engine/workers.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/find.rs crates/thinkthen/src/cli/schedule.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/tests/backend sdlc/scripts/policy.py sdlc/ratchet.json sdlc/planning/adr/0017-libraries-over-one-bound-core.md sdlc/planning/libraries
---

# 0077: Share one process width cap

Status: ready; implementation waits for tickets 0082, 0083, and 0076 to land

## Outcome and authority

Every live HTTP attempt in one process shares one width cap. Ordinary record scheduling, grouped `annotate`, a single judgment, aggregate `find`, split chunks, `recognize`, `relate`, any request path added by tickets 0082 and 0083, explicit engines, and module-level convenience calls cannot multiply the selected concurrency by using separate schedulers or clients.

Ian ruled the selection contract exactly. An engine built with no explicit width does not set the process cap. The first engine built with an explicit width sets it. Engines with no explicit width follow the active cap and never conflict. A later explicit engine with the same width succeeds. Only a later explicit engine with a different width fails locally before any request. Its diagnostic names the active width and says to match it or drop the argument.

This ticket records the design now and does not move implementation ahead of the launch work. The current build queue places 0079 through 0083 and 0087 before the ordered 0076 through 0078 controls sequence. Ticket 0076 lands the deadline state first. Ticket 0077 then composes the gate wait with that state without reopening the deadline contract. ADR 0017 owns one gate for the process, and the current queue supplies the later width-selection ruling where ADR 0017's older settings table still reads as though every engine defaults its own width to 4.

## Current facts

The command currently turns an absent `--jobs` into `4` at its edge. Ordinary and grouped schedulers each create `jobs` scoped workers. Single-document judgments and `find` bypass both schedulers. Each command path builds its own `http::Client`, and `Client::post_observed` sends initial attempts and retries without a shared permit. One process can therefore multiply width when later library callers create several engines or mix scalar and bulk calls.

The accepted measurements explain the risk. One hundred independent calls reached 85 concurrent requests when each batch owned its own gate. A shared gate held the same workload at its selected width. The command contract accepts widths from 1 through 32 and uses 4 when the user omits `--jobs`. The vendor guidance cited by `specification/records.md` says useful public-endpoint concurrency is about eight. This ticket does not change the accepted range or add a pacer. It prevents independent paths from multiplying a width that the caller selected to fit the vendor.

## Design

### Selection and effective width

Keep explicitness through the command edge. `--jobs N` contributes `Some(N)`. An omitted argument contributes `None`; no edge may replace it with `Some(4)` before process-width registration. The command still schedules an unbound batch at the existing fallback width 4, so command behavior and help remain unchanged.

The process-width state has two distinct facts:

- The selected explicit width starts unset. An implicit engine leaves it unset. The first explicit width atomically changes unset to that value. The same explicit value is idempotent. A different explicit value returns a typed local usage failure.
- The effective width is the selected explicit width when one exists and the existing fallback 4 otherwise. An implicit engine reads the effective width at call start. An implicit engine built before activation therefore follows the active width on its next call. It never claims that 4 as an explicit process decision.

An already-started implicit call may have created four scoped workers before a first explicit engine activates another width. Those workers still pass through the shared attempt gate. Activation does not revoke an attempt that already acquired a permit. Every attempt that starts after activation observes the selected cap. A lower first explicit width blocks new acquisitions until older attempts drain below it. An upper first explicit width becomes available to calls that begin or dispatch afterward. No caller may clear or replace an active explicit width inside the same process.

The exact conflicting-width diagnostic is:

`thinkthen: width 4 is already active for this process; use width 4 or drop the width argument`

The number is the active width. The failure occurs during engine construction or command setup, before key lookup, cache or recording mutation, client connection, input dispatch, request accounting, or HTTP transport for that caller. It maps to the existing usage failure kind and exit 2 in the command. It contains no backend, key, evidence, or engine identity.

### One attempt gate

Put one process-shared gate at the transport attempt boundary. A scheduler's worker count bounds that call's memory and local work. It does not serve as the process cap. Every initial attempt and retry acquires the same gate after cache or replay has shown that a live send is needed and before ticket 0073's attempt-start cancellation check, request accounting, and transport call. A cancelled waiter sends nothing. Hold the permit only while the blocking HTTP attempt is in flight. Release it before decoding, recording, a retry wait, cache work, ordered output, or host callbacks. A retry acquires a new permit.

The placement covers every request path because `engine::request` reaches live transport through `Client::post_observed`. Before implementation, inventory the current post-0083 tree and name every live path in the ticket review: direct judgments, record scheduling, grouped `annotate`, `find`, split chunks, `recognize`, `relate`, retries, and any request path added by tickets 0082 and 0083. Every path must converge on the same private send boundary. Keep the low-level send function private and extend the policy check so production code outside the gated HTTP module cannot call `ureq` or create another live-send door.

The gate limits concurrent attempts across all clients in the linked engine image. It does not serialize cached answers, replay, request preparation, local validation, or completed-result rendering. It promises a cap and cancellation-aware waiting. It does not promise fairness between callers or reserve lanes for scalar work.

### Engines and convenience calls

The later public builder passes `Option<Width>` into the same private registration path. `Engine::from_env()` and every builder that omits width pass `None`. Module-level convenience functions delegate to one lazy default engine built with `None`. They must not materialize the fallback as an explicit width. Otherwise the first convenience call would silently freeze the process at 4 and reject a later `Engine::builder().width(8)`.

The command follows the same rule. An omitted `--jobs` uses the unbound fallback. An explicit `--jobs 4` activates 4. This distinction changes no output or throughput for one command process, but it preserves the library contract before the public API freezes.

### Process, host, and fork boundaries

One operating-system process owns one cap for one linked engine image. Host threads, async tasks that enter the blocking engine, database worker threads inside that process, explicit engines, and the lazy convenience engine share it. Separate command processes and PostgreSQL backend processes have independent caps. This ticket does not coordinate limits across processes.

Ticket 0078 owns complete fork recovery. It must compare the current process ID before touching an inherited width wait primitive, connection pool, cache coordinator, or other request-path lock. On a mismatch it publishes fresh child state without taking an inherited lock and abandons the inherited state. The child starts with no selected explicit width. Its first child call then contributes that engine's explicit width or remains implicit. Parent state does not change. Ticket 0077 must keep width registration and permit state in one replaceable process-state component and must not hide another global lock or condition variable that 0078 cannot replace.

Ticket 0077 does not claim warm-child or busy-child safety by itself. The public Rust API and real surface integration remain blocked until 0078 proves the gate and pool rebuild together. The C planning page also permits a host to load two copies of the library. Two independently loaded native images cannot share a Rust static. The surface integration ticket must either prevent duplicate engine images in one host process or provide a cross-image process primitive before it repeats an absolute one-process claim. This ticket proves one linked engine image and records that boundary plainly.

## Scope and exclusions

Allowed: one private process-width component; preservation of explicit versus omitted width; cancellation-aware permit acquisition; one gated HTTP-attempt path; ordinary and grouped scheduler plumbing needed to read the effective width; the exact conflict mapping; mechanical enforcement against another live-send door; deterministic unit and loopback tests; the ADR 0017 width-selection correction; directly affected library planning sentences; exact ratchet, ticket, queue, and record updates.

Production may touch at most ten Rust files and add at most 550 nonblank Rust lines. Focused tests may add at most 750 nonblank Rust lines. Reuse ticket 0073's 50 ms cancellation poll and existing scoped workers. Add no dependency, async runtime, resident worker thread, pacer, second scheduler, generalized resource pool, or host callback.

Excluded: changing the 1 through 32 accepted command range, changing the fallback of 4, rate-per-minute control, HTTP/2, one connection for several simultaneous HTTP/1.1 requests, deadlines, public Rust types, full fork recovery, connection-pool rebuild, host signal ownership, cache or recording semantics, counter meanings, retry classification, output order, memory-window rules, workflows, surfaces implementation, site, paid calls, and publication. Do not change 0087's implementation position or begin 0077 implementation before 0087 lands.

## Deterministic acceptance

- A state test builds any number of implicit engines and proves the selected explicit width remains unset. Their calls use effective width 4 until an explicit width activates another value. Calls begun after activation use that active value.
- Synchronized transition tests hold four fallback permits while an explicit width of 1 activates, queue implicit waiters on both sides of the activation point, and release attempts one at a time. No new acquisition starts above 1 after the activation linearization point, stale fallback waiters do not pass, and every waiter wakes or cancels. A matching upshift test activates width 8 while fallback work is held and proves newly available capacity wakes waiters without exceeding 8. Barriers identify the activation point; timing does not choose the result.
- A table over widths 1, 4, 8, and 32 proves the first explicit width activates once, the same explicit width remains accepted, and a different explicit width returns the exact diagnostic with the winner's value. A synchronized two-thread race between different first explicit widths proves exactly one value wins and the loser names it. The assertion does not choose the winner by timing.
- A compiled command with omitted `--jobs` does not activate 4. A compiled command with explicit `--jobs 4` does. A later conflicting explicit construction fails before input dispatch, key lookup, connection, request accounting, and request. Count each boundary rather than inferring it from exit status.
- One synchronized loopback listener holds replies and counts active sockets. Concurrent direct judgments, ordinary records, grouped `annotate` work, aggregate `find` work, split chunks, `recognize`, `relate`, retries, and every live path added by tickets 0082 and 0083 share one cap. The listener observes exactly the active width and receives no next request until one held reply is released. The final maximum never exceeds the cap. Use channels and barriers for each phase; use only a generous outer timeout to detect deadlock.
- A retry releases its permit after the failed attempt, performs no wait while holding capacity, and reacquires before the next attempt. A cache hit and replay acquire no permit. Existing send and usage counts remain exact.
- Cancellation while waiting for width returns the existing cancelled cause within the shared polling protocol, sends nothing for that waiter, and leaves no permit consumed. Attempts already holding permits follow ticket 0073's existing finish rule. All scoped workers still join.
- A process-state seam proves PID inspection precedes access to every replaceable width wait state. Ticket 0077 makes no child-success claim. Ticket 0078 must add warm-parent and busy-parent fork tests against the real loopback wire before the public API opens.
- A future-facing constructor test pins `None` for the lazy convenience engine and omitted builder width. If the public functions are still absent, place the private seam and its test now, then make tickets 0084 through 0086 consume that seam without translating `None` to 4.
- Existing `--jobs` order, bounded-memory, annotate queue, cancellation, retry, cache, secrecy, and exact-output tests remain green. Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass before code review. The coordinator then runs `sdlc/scripts/install`, `lint`, `test`, and `spec` sequentially. No gate or proof opens a non-loopback socket.

## Dependencies and follow-through

Tickets 0055, 0064, 0072, 0073, and 0074 provide the private engine boundary, bounded retries, fast refusal, cancellation, and command SIGINT behavior this gate must preserve. Tickets 0079 through 0083 and 0087 must land before implementation by Ian's launch-first queue. Ticket 0076 is the direct prerequisite because a whole-call deadline must bound the new gate wait. Ticket 0078 must compose fork recovery and pool replacement with this process state before tickets 0084 through 0086 expose the public Rust API and packages.

The separate `surfaces` branch proves rehearsal behavior over a stand-in. Its earlier settings-keyed state allowed a narrow engine beside a wide engine. Ian's later one-process ruling supersedes that behavior. Real surface integration must remove per-settings width gates, preserve settings differences unrelated to width, and run the combined-path proof against the production engine.

## Complexity and routing

Contract 2; State/timing 4; Reach 4; Proof 4; Cost of error 4; Total 18. Minimum and final level: 4. One process-global concurrency decision crosses every paid request path and must hand state safely to fork recovery. A mistake can exceed the selected vendor-facing concurrency, deadlock cancellation or a forked host, or reject a valid later engine. Route implementation to `gpt-5.6-sol` with medium reasoning. Use separate Sol design and code reviewers. Stop and redesign if proof requires a second transport door, cross-process coordination, or a dependency.

## Source-backed blockers

- Implementation order is blocked until 0082, 0083, and 0076 land. Ticket 0087 has landed.
- Public convenience-call proof cannot finish on this ticket because the public Rust API is scheduled in 0084 through 0086. This ticket must land the private `Option<Width>` seam that those tickets consume.
- Public fork-safe claims remain blocked on 0078. ADR 0017 requires a PID check before inherited locks and a fresh pool and gate in the child.
- An absolute cap across two separately loaded copies remains unresolved. `sdlc/planning/libraries/c.md` permits that host shape, while a Rust process static covers one linked image. Draft ADR 0047 (ticket 0093) states one cap per loaded copy and defers a cross-image cap past 0.1; Ian can overturn it.

## What Ian can overturn

Ian can overturn the fallback width, accepted range, exact conflict sentence, and duplicate-image boundary. His one-process selection ruling controls this ticket unless he replaces it: implicit engines never set the cap, the first explicit width sets it, implicit engines follow it, and only a later different explicit width fails.
