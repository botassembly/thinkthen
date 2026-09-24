---
flow: build
priority: 78
opens: crates/thinkthen/Cargo.toml crates/thinkthen/src/main.rs crates/thinkthen/src/cli crates/thinkthen/src/engine crates/thinkthen/tests/backend sdlc/scripts/policy.py sdlc/ratchet.json sdlc/planning/adr/0017-libraries-over-one-bound-core.md sdlc/planning/libraries sdlc/planning/databases
---

# 0078: Recover after fork and release host signals

Status: draft, design not reviewed. Owner: Claude. Was marked ready; implementation waits for tickets 0082, 0083, 0076, and 0077 to land

## Outcome and authority

Every engine call first proves that its private runtime state belongs to the current operating-system process. A child never touches an inherited mutex, condition variable, width waiter, connection pool, recorder coordinator, cache handle, counter lock, or worker queue. On a process-ID mismatch it publishes fresh child state through an atomic replacement path, then uses a fresh pool, an unset child width selection and gate, fresh process counters, and fresh recorder and cache coordination. The parent continues with its original state.

The engine library installs no process-wide signal handler. The command retains ticket 0061's exact Unix behavior under `RLIMIT_FSIZE`: a recording or cache write that reaches the file-size limit returns the fixed recording-storage diagnostic and exit 5 through normal cleanup. That `SIGXFSZ` policy moves to the command edge. An embedded Python, Ruby, R, C, database, or Rust host keeps the signal disposition it selected.

ADR 0017 section 2 fixes the PID-before-lock rule, fresh child pool and gate, lock-free replacement path, and leak-instead-of-teardown rule for inherited runtime state. The C plan forbids a signal handler in an embedded library. The settled surface ledger leaves each host's interrupt channel with that host. Tickets 0076 and 0077 provide the whole-call deadline and one process-width component that this ticket makes fork-safe. This is the final private-control ticket before tickets 0084 through 0086 may expose public engine calls and packages.

## Current facts

The post-0074 tree has no process-state root. Each command path builds a `ureq::Agent`; the recorder retains a `Mutex<Option<FolderGate>>`; persistent counters retain a `Mutex<Option<PathBuf>>`; ordinary workers share a queue receiver behind a mutex; grouped annotation holds another queue; and cache and usage paths take operating-system locks. A child forked while another parent thread owns any inherited userspace lock can wait forever because the owning thread does not exist in the child. A warm inherited HTTP agent can also retain sockets and internal pool synchronization that belong to the parent.

The schedulers create scoped workers and queues for one call and join them before return. They hold no resident thread between calls. That remains the design. Fork recovery must create a new child call and new child workers. It must never resume an inherited parent queue or count inherited in-flight work.

The current recorder calls `signal_hook::flag::register(SIGXFSZ, ...)` once from engine code before a writing operation. Registration changes process-wide signal policy for the rest of the process. That was correct for the standalone command in ticket 0061 and conflicts with the library and C plans. A host may deliberately keep the default action, ignore the signal, chain its own handler, or translate it through its runtime. Engine recording cannot silently add another permanent handler.

The durable answer cache and count-only usage files are shared across processes by design. Fresh child state does not mean a new folder or erased totals. It means new userspace coordination and newly opened file handles around the same configured paths. The child starts its in-memory counters at zero and contributes only child sends, tokens, and cache answers to the durable aggregate. It never copies the parent's pre-fork process totals into a second contribution.

## Design

### One PID guard before runtime state

Keep immutable engine settings outside the replaceable runtime state: resolved backend and model settings, timeout and retry settings, cache and recording paths, usage path, and an engine's explicit-or-omitted width choice. Put every fork-unsafe mutable resource below one private `ProcessState`: ticket 0077's selected-width state and attempt gate, every reusable HTTP pool, process counters and their persistent-update mutex, and recorder/cache coordinators that retain a userspace lock or open handle between calls.

Every direct and bulk call enters one `current_process_state` choke point before key lookup, cache or recording preparation, counters, width registration or acquisition, pool access, input dispatch, scheduler construction, or worker creation. Read the current PID and the published owner PID using atomics before loading the state slot. When they match, acquire the published state and continue. When they differ, no code may inspect, lock, clone, drop, or call through the inherited state first.

The replacement protocol uses only atomics until fresh child state exists. A PID-tagged rebuilding marker elects one child thread. The winner constructs complete fresh state from immutable settings, atomically replaces the slot, deliberately leaks the inherited state in the child, then publishes the child PID with release ordering. Other child callers wait on the atomic rebuilding marker with the existing bounded poll and observe cancellation or ticket 0076's deadline. They never wait on an inherited mutex. A child forked again while replacement is in progress can replace the inherited marker because it names another PID.

Use one safe atomic-`Arc` slot implementation. The repository continues to forbid unsafe code. This ticket may add the narrowly scoped `arc-swap` dependency if the post-0077 tree has no equivalent reviewed safe primitive. The PID word and slot must be statically initialized without a lazy mutex that can itself be inherited mid-initialization. Stop for redesign if the selected primitive takes a userspace lock before the PID mismatch is known, runs inherited-state destruction during replacement, or cannot publish the complete state before the child PID.

Retired inherited state is leaked only in the forked child. The parent owns and eventually drops its state normally. A same-process width change or ordinary engine drop does not leak. The ticket records the bounded cost: one retired runtime state per fork generation that later uses the engine. No background reaper, at-fork callback, process scan, or inherited teardown enters.

### Fresh child meaning

The child width selection starts unset. Its first call applies ticket 0077 exactly: an explicit engine width sets the child process cap, an implicit engine leaves the selection unset and uses fallback 4, a matching later explicit width succeeds, and only a conflicting later explicit width fails. Parent selection, permits, and waiters do not cross. The fresh gate has no permit held.

The child opens a new HTTP pool sized from its child effective width. It reuses no inherited idle socket, resolver state, connection object, or pool synchronization. A warm parent therefore gives the child no transport shortcut. A busy parent gives the child no inherited capacity debt. The parent request count and active sockets remain unchanged by child rebuilding.

The child creates new process counters at zero and a new persistent-update mutex around the configured usage path. Durable monthly files remain cross-process and keep their operating-system locking. The child adds only operations that begin in the child. A pre-fork parent count is visible in the durable total once, not copied into the child's process snapshot or added again.

The child creates new recorder and cache coordinators from immutable folder settings. It does not inspect or lock an inherited recorder mutex or reuse an inherited `FolderGate`. New child operations open their own directory and digest locks and follow the existing second-read rule. Stored entries, backend identity, privacy checks, immutable-entry behavior, and cache-answer meaning do not change. Restructure any process-retained recorder handle that would keep an inherited filesystem lock alive after replacement; request-owned operating-system locks remain request-owned and are never treated as reusable process state.

Each child call creates new scoped worker queues after state recovery. No scheduler, input request, pending annotation group, completed parent result, cancellation observation, or deadline instant transfers from the parent. Caller-supplied cancellation remains caller-supplied; this ticket does not silently reset an explicit token. A new call computes its own ticket 0076 deadline from its own budget.

### Host signal ownership

Remove `SIGXFSZ` registration and its `OnceLock` from the recorder and from every no-default-features library build. Make `signal-hook` a CLI-only dependency unless another CLI-only path still requires it. The private engine reports ordinary write, sync, and install errors but never installs, replaces, ignores, chains, or restores a process signal disposition.

The command owns its existing Unix policy. Before any command path can create a recording or cache write permit, and before input, key lookup, or transport, the CLI installs the same safe process-once `SIGXFSZ` handler. Setup failure remains the fixed local recording failure and sends nothing. Version, help, read-only status, cache inspection, prune, and replay-only work do not need to claim the signal. The existing file-size-limit subprocess must still exit 5 with the exact safe diagnostic, remove its temporary entry when cleanup succeeds, and never die with signal 25.

An embedded host receives no substitute promise. With the engine built without `cli`, a host's default, ignored, blocked, or custom `SIGXFSZ` action remains its action before, during, and after recording. If that policy lets a write return an error, the engine maps it to the local recording failure and performs normal cleanup. If the host keeps the default terminating action, the process may terminate. The library documentation states that file-size signal policy belongs to the host. This preserves command behavior without making an embedded engine owner of unrelated process policy.

## Scope and budgets

Allowed: one PID-guarded replaceable private process-state component; fresh child width state and gate, HTTP pool, counters, recorder/cache coordination, and scoped queues; immutable-settings reconstruction; leak-on-child-replacement; one safe atomic-slot dependency if required; moving `SIGXFSZ` setup to the CLI edge; CLI-only dependency wiring; policy enforcement; deterministic Unix subprocess tests; non-Unix no-op compilation; exact ratchet; and later implementation updates to ADR 0017, directly affected library/database plans, queue, ticket, and record.

Before implementation, rebase onto the tree after 0083, 0076, and 0077 and inventory every call entry and every mutable process resource. The inventory must name direct judgments, ordinary records, grouped `annotate`, aggregate `find`, split requests, `recognize`, `relate`, retries, width waits, deadline waits, pools, counters, recorder/cache coordinators, and any path added through 0083. It must classify each item as immutable settings, replaceable process state, or call-local state. No unclassified lock or pool may remain.

Production may touch at most fourteen Rust files and add at most 650 nonblank Rust lines. Focused tests may add at most 950 nonblank Rust lines. One new dependency is the ceiling, and it must serve only safe atomic state replacement. Reuse the existing cancellation/deadline poll, ticket 0077's process-width component, current blocking client, scoped workers, recorder paths, and operating-system file locks.

Excluded: public Rust types or functions; language or database binding implementation; public option spelling; changes to width selection, fallback 4, or range 1 through 32; deadline, cancellation, retry, request-accounting, cache-entry, recording-byte, backend-identity, output-order, or error-kind semantics; a host interrupt callback; ownership of SIGINT, SIGTERM, or any signal other than the command's existing `SIGXFSZ` policy; `pthread_atfork`; an async runtime; resident threads; a background cleanup task; cross-process width coordination; duplicate native-image coordination; closing arbitrary host file descriptors; workflows; site work; paid calls; publication; and release packaging.

## Deterministic acceptance

- A private call-entry observer proves PID comparison occurs before width registration or waiting, pool access, recorder/cache coordination, counters, input dispatch, queue construction, key lookup, request accounting, connection, or send. Each boundary has a counter or phase signal. No test infers ordering from an error alone.
- A warm-parent Unix subprocess makes one complete loopback call, keeps the engine value alive, forks, and makes a child call through the same value. The child uses a newly accepted connection, fresh pool identity, fresh gate identity, and fresh counter identity; it answers once within a watchdog bound. The parent then answers again from its original pool and retains its original counters and width selection. Channels establish every phase. Wall time only detects a hang.
- A host signal is not a cancel (error-index row R6-3). A test installs a no-op `SIGUSR1` handler without `SA_RESTART` and delivers it to engine workers during a held send and a retry wait. The call never returns `Cancelled`. The expected outcome is its normal answer and send count. If the transport reports the interruption instead, the test pins that outcome and an issue goes to 0089's `Other` transport class. Amended 2026-09-24.
- A busy-parent Unix subprocess holds every width permit and every loopback reply, then forks from a separate host thread. Before any parent reply is released, the child calls through the inherited engine and completes against a separate child listener at its own selected width. The child neither waits on the parent gate nor reuses a parent socket. The parent later completes in order. Repeat enough synchronized rounds to cover the inherited-lock window; no sleep chooses the interleaving.
- Child races start two post-fork callers together. Exactly one fresh state is published, both use it, no inherited lock is touched, and conflicting explicit widths follow ticket 0077's exact winner-and-diagnostic rule. A second-generation child forked while the first child is rebuilding also recovers instead of waiting on the inherited rebuilding marker.
- Warm-cache proof fills one answer before fork. The child reopens fresh cache coordination, returns the stored answer with zero sends, and increments only its fresh cache-answer counter. The parent retains its own process snapshot. A busy-cache proof holds parent coordination at synchronized recorder-mutex, folder-gate, and digest-lock phases and proves a child request for independent work never touches those inherited userspace locks. Same-digest work retains the existing operating-system lock and second-read semantics; the test releases the parent and observes one installed answer rather than inventing a fork-specific cache rule.
- Counter proof gives the parent nonzero process counts, forks, and observes a zero child process snapshot before the child call. Parent and child then each add known deltas to one durable usage folder. The final files equal the pre-fork parent contribution plus each post-fork delta exactly once. No copied parent total appears in the child contribution.
- Queue proof forks while ordinary and grouped schedulers each have held work. A child call creates only fresh scoped workers and queues, returns all child workers joined, and consumes no parent input, pending group, or result. Parent ordering, stopped-at metadata, cancellation, and deadline behavior remain unchanged after release.
- The existing command `RLIMIT_FSIZE` subprocess still exits 5, prints exactly the fixed recording-storage line, sends the already-counted request no extra time, and leaves no final or temporary entry after successful cleanup. Signal setup failure still happens before key lookup and transport.
- A no-default-features Unix host harness installs its own `SIGXFSZ` action, performs private recording work under a file-size limit, and proves the same action remains installed and receives the signal. A returning host action lets the engine report the local storage failure and cleanup. A separate child with the default action proves the engine does not swallow or replace it. Building the library without `cli` does not include the command signal-registration path.
- Existing 0074 SIGINT subprocesses, 0076 deadlines, 0077 width races, retries, cache and recording durability, usage totals, secrecy, exact output, and worker-join tests remain green. Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass before code review. The coordinator then runs `sdlc/scripts/install`, `lint`, `test`, and `spec` sequentially. No proof opens a non-loopback socket or uses a paid service.

## Dependencies and order

Tickets 0055, 0061 through 0065, 0072 through 0074 provide the private engine boundary, durable recording and cache locks, counters, retries, cancellation, and command SIGINT behavior this ticket preserves. Tickets 0079 through 0083 complete the request tree that the fork inventory must cover. Ticket 0076 supplies the whole-call deadline used while child callers wait for one fresh publication. Ticket 0077 supplies the replaceable width selection and gate and requires this ticket to make its private process state fork-safe.

Implementation starts only after 0082, 0083, 0076, and 0077 land. Rebase and rewrite the inventory against that exact tree before a red test. Ticket 0078 then lands as the final private-control ticket. Tickets 0084 through 0086 remain blocked until its warm-parent, busy-parent, cache, counter, queue, and host-signal proofs pass against production engine paths.

The separate `surfaces` branch supplies host-specific integration cases but no production state implementation. Its stand-in fork checks and Node child-process checks do not satisfy this ticket. Real Python and Ruby prefork checks, PostgreSQL backend checks, DuckDB signal coexistence, and installed packages remain for surface integration after the public Rust API exists.

## Public API blockers

- No public `Engine`, builder, module-level convenience function, C door, or package may claim fork safety until every call enters the PID guard before any inherited mutable resource.
- Tickets 0084 through 0086 must consume immutable settings plus the private current-process-state seam. They may not cache a raw pool, gate, recorder, or counter reference outside the replaceable state.
- Public usage snapshots must define process scope after fork: a child begins at zero, while durable command totals remain cross-process aggregates. This ticket proves the private behavior but does not freeze public type names.
- Public recording documentation must state host ownership of `SIGXFSZ`. The command retains its stronger exit-5 promise. No binding may reinstall the command's handler to imitate it.
- One Rust image provides one process state. Two separately loaded native images do not share this atomic slot or width gate. Draft ADR 0047 (ticket 0093) qualifies the claim per loaded copy.
- Unix gets the fork guarantee. Non-Unix builds keep equivalent ordinary call behavior and no signal setup, but this ticket makes no fork claim where the platform has no matching process model.

## Complexity and routing

- Contract score: 2
- State and timing score: 4
- Reach score: 4
- Proof score: 4
- Cost of error score: 4
- Total: 18
- Minimum level floor: level 4 for fork recovery across inherited concurrency and process-wide signal ownership
- Final level: 4
- Reasons: one pre-lock PID decision controls every request path, pool, width waiter, counter, cache coordinator, and scheduler after a multithreaded fork. A mistake can deadlock an embedded host, reuse a parent connection, duplicate accounting, violate the selected width, or take over a host signal. Warm and busy subprocess proof must establish the ordering on real local wire and filesystem paths. The public surface remains closed, but the private control is a release boundary for every later surface.
- Selected implementation: `sol-implementer` (`gpt-5.6-sol`, medium reasoning). Use `swe2-researcher` only for the post-0083 resource inventory and reproducer preparation. Independent design and code review use separate `sol-reviewer` sessions with `gpt-5.6-sol`, medium reasoning. Resume only the code reviewer for remediation review.

Stop and redesign if the post-0083 tree contains a live-send door outside the guarded state, the safe atomic slot touches an inherited lock, a child must destroy inherited state, cache correctness requires closing arbitrary host descriptors, signal safety requires an engine-owned process handler, or the change needs a public API decision.

## Review

- Design review: pending.
- Code review: pending.

## What Ian can overturn

Ian can overturn the child counter reset, leak-per-fork trade-off, one-dependency allowance, exact budgets, and whether the command installs `SIGXFSZ` for read-only paths. ADR 0017's PID-before-inherited-lock rule, fresh child pool and width gate, and host ownership of embedded signal policy control unless he replaces them. Ticket 0077's one-process width selection remains its own ruling.
