# standin NOTES

## 2026-09-21 — R1: the recognize and relate replay

The stand-in answers both functions from the recordings, never by inventing
an answer.

- `src/replay.rs` embeds `data/recognize-replay.json` (generated from
  `experiments/225-recognize-harvest-package` by
  `../conformance/tools/build_recognize_cases.py`) and implements the
  matching rules: a text the recordings do not hold is a usage error naming
  the text; a rule the recordings do not hold for that text is a usage error
  naming the rule and the covered set; a rule's named end must be one of the
  asked kinds; relate refuses a kind field and named rule ends because the
  recordings carry no kinds; the record limit refuses past 255.
- The recordings carry the number on a name under Jev's own field name; the
  replay maps it to the ruled field `strength` (settled 2026-09-21; the R1
  record's interim `number` is history).
- Two relate arms (R03 per-subject, R04 pairs) share one text; the replay
  prefers the pairs form, the ruled method, and the per-subject case is
  pinned in the conformance file and skipped in the replay test, recorded in
  `../conformance/DIVERGENCES.md`.
- The trait methods `recognize_opts` and `relate_opts` call
  `Error::guard(options)` first and then the replay; `relate_opts` also runs
  `guard_relate_records` so the limit holds even when a caller enters
  directly.

Commands and output:

```
$ cargo test --test recognize_replay
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test
lib: 4 passed; wire: 3 passed; recognize_replay: 7 passed
```

The seven: every conformance case replays exactly (41 recognize cases and
R01/R02/R04; R03 skips as marked); the door JSON carries `source`, `target`,
`probability`, and `number` and never `from`/`to`/`confidence`; the deck's
0.9-bar relate call returns the two sound edges; the 255 refusal; missing
texts and unrecorded rules name themselves; the any-kind end replays on the
C36 case; relate refuses kinds it cannot honour.

`cargo check --all-targets` is clean with no warnings.

The check script's R-surface slice now fails on the 45 new cases because
that runner parses every case's question before it switches on the verb;
the guard is the R surface lane's one-line fix, filed in
`../conformance/DIVERGENCES.md`.

## 2026-09-21 — the adversarial-review fix wave, core lane

Rulings 1, 3, 5, 7, and 10, and the core minors. Before and after, with the
command that shows it.

**Ruling 1, the question file says source and target.** `relation_from_value`
requires `source`/`target` and refuses `from`/`to` with a usage error naming
the ruled spelling; the contract's file tests and the two test files that
built rules from `from`/`to` read the ruled keys; the generator's question
rules and the validator's rule-key checks follow. Before: `{"name":"works_for",
"from":"person","to":"organization"}` parsed. After:

```
$ cargo test --release -p thinkthen-contract
13 passed (the_old_relation_spelling_is_refused is new)
```

**Ruling 3, Ctrl-C on a fast backend.** The wait loop ran the poll only in
the `RecvTimeoutError::Timeout` arm, so a busy channel starved it. The loop
now runs the poll from the answer arms too, gated by one `TICK` since the
last poll. Before, by the supervisor's probe: SIGINT one second into a
three-million-record null batch raised at 8.48 s, after the batch. The Rust
regression test's own before/after (temporarily dropping the busy-arm ticks):

```
the_poll_runs_on_a_busy_channel  FAILED in 2.87 s (the whole batch ran)
the_poll_runs_on_a_busy_channel  ok     in 0.24 s (cancelled after one tick)
```

And end to end through the Python shim, the new
`libraries/python/tests/test_cancel_fast.py`:

```
elapsed 0.772
OK  the interrupt landed at 0.772 s, within a tick of the signal
```

(The parent's SIGINT leaves at 1.0 s from spawn, about 0.72 s from the
child's batch start after the import.)

**Ruling 5, the counter counts what left the machine.** `post()` counts a
send when it left and counts a sent retry again; a refusal, a failed
connect, or a name that did not resolve counts nothing (`left_the_machine`).
The two contradicting comments are gone. New tests, one process each:

```
tests/refused.rs  a_refused_send_counts_nothing  ok (3.00 s; delta 0)
tests/billing.rs  a_retried_send_counts_twice    ok (1.41 s; delta 2, responder read both)
```

**Ruling 7, the generator.** `ruled_entity` writes `strength`; question
rules write `source`/`target`; every relate case carries its `form`. Rerun:

```
wrote 72 cases (41 recognize, 4 relate); replay table 41 + 4 rows
OK: 72 cases validated: schema, grammar, digests, wire contract, offline replay
```

The validator gained the case-id uniqueness check.

**Ruling 10, the two fork preconditions**, in the crate docs: the settings
are read once on first use and must be read before any fork, and a forked
child keeps the inherited pool's file descriptors until it exits.

**Core minors.** `replay.rs`'s docs no longer call `confidence` or `number`
ruled names; `conformance/DIVERGENCES.md`'s interim-number bullet is
rewritten as history pointing at the rename commits.

## 2026-09-22 — the review fix wave, the stand-in's three pieces

Findings 7 and 8, group 4's retry waits, and the connector door the review
asked for. Each fix carries the test that failed before it.

**The connector door, and finding 8 (the ignored address and width).** The
knobs now resolve per engine value: `ResolvedConfig::resolve` reads the
`EngineConfig` first, the environment second, and the built-in defaults
last, and `BlockingEngine` carries the result. `StandinConnector`
implements the contract's `Connector` and returns `Arc<dyn Engine>`. The
process state is keyed by pid and transport shape (address, width,
timeout), so an engine value with its own config gets its own pool and
gate instead of silently inheriting the environment's. `Settings` stays
the config's alias, so no surface changed.

`tests/connector.rs` proves the precedence end to end with no environment
of its own: the environment names a refused address (`127.0.0.1:1`) and
width 8, the config names a live listener and width 1, four concurrent
calls all answer and the listener's peak concurrency is exactly 1.

**Finding 7 (the fake failure in product code).** The annotate
partial-failure fixture now fires only when the test-only
`ENGINE_SYNTHETIC_PARTIAL` opt-in is set. Unset — every production
process — the fixture record answers like any other input.
`the_fixture_text_answers_on_the_default_path` fails against the old code
(`topic answered, not failed` was the panic: the old code returned the
Failed marker), and `the_partial_failure_marker_is_the_ruled_shape` still
pins the marker through the opt-in. Cross-side consequence for the merge:
every conformance runner that replays
`74-annotate-preserves-good-answers` must export
`ENGINE_SYNTHETIC_PARTIAL=1` for that case, or the case needs revising in
the phase H conformance lane.

**Group 4 (retry waits that ignore a stop).** Both backoff sleeps are now
`sleep_checked`: slices of at most 100 ms, each one checking the cancel
token and the deadline. Before the fix,
`a_cancel_lands_inside_the_retry_backoff` measured the full backoff —
`the cancel cut the 1 s backoff short, wall was 1.000808042s` — and after
it the cancel lands inside 150 ms. A deadline shorter than the backoff
returns the deadline kind with the budget in its message.

Commands and output:

```
$ cargo test
lib: 9 passed; backoff: 2 passed; billing: 1 passed; connector: 1 passed;
recognize_replay: 7 passed; refused: 1 passed; wire: 3 passed (skipped
offline); doc tests: 1 passed

$ cargo test --test backoff   # with the checked sleep disabled
thread 'a_cancel_lands_inside_the_retry_backoff' panicked:
the cancel cut the 1 s backoff short, wall was 1.000808042s
test result: FAILED. 1 passed; 1 failed
```

The fixture text now appears only in tests, the conformance case, and
notes: `grep -rn "order 4471: charged twice"` over the worktree finds it
in `conformance/conformance.json`, this crate's gated fixture and its
test, six surface test files, and NOTES files — no surface's `src/` and no
database's `src/`.

## 2026-09-22 — the second review wave: state per settings value, a compile-time fixture door, retry classification, and the settled names

Four changes, each with the test that failed before it (commands below):

- **One state per settings value.** `STATES` is a fixed table of atomic
  pointers keyed by pid, address, width, and timeout: a lookup is one
  atomic load and an `Arc` clone per slot, a miss publishes through a
  compare-and-swap, a fork rebuilds into empty or stale slots, and a
  replacement retires rather than frees, for the reason the single slot
  always gave. Before, `a_narrow_engine_keeps_its_width_beside_a_wide_one`
  measured a peak of 3 concurrent calls on a width-1 engine while a
  second, wider engine ran beside it, and `the_file_descriptors_stay_flat`
  saw 10 → 14 descriptors over forty calls. After, peak 1 and flat.
- **The fixture door is compile time.** The synthesized annotate failure
  sits behind the `synthetic-partial` cargo feature, off by default;
  `ENGINE_SYNTHETIC_PARTIAL` is read by nothing. Before, a temporary
  host-level test with the variable set failed (`failed_questions` was 1,
  not 0). After, a default build answers the record for real, and the
  in-lib test `the_env_variable_arms_nothing` pins it.
- **A refused connection fails at once.** `post` classifies a send
  failure before the retry loop; only a genuinely retryable failure earns
  the backoff. Before, `a_refused_send_fails_at_once` measured 7.00 s
  (1 s + 2 s + 4 s of backoff). After, under 0.9 s, non-retryable, and
  the counter stays put. The retry-wait tests now generate their retry
  with a local 503, a genuinely retryable answer.
- **The settled environment names.** `THINKTHEN_NULL`,
  `THINKTHEN_BASE_URL`, `THINKTHEN_TIMEOUT_SECS`, `THINKTHEN_MAX_RETRIES`,
  and `THINKTHEN_WIDTH` win; the older `ENGINE_*` names keep working,
  deprecated. Before, `the_prefixed_names_win_and_the_old_ones_still_work`
  measured width 1 with `THINKTHEN_WIDTH=4` set. After, 4, and the
  deprecated name alone still sets 2.

The contract's parser wave changed annotate's columns to file order, so
`every_verb_answers_on_null` now expects `spam` before `kind`. The C
gate's `ENGINE_SYNTHETIC_PARTIAL=1` exports are gone: `libraries/c/check.sh`
builds the door with `--features synthetic-partial` for the door shape
test and case 74, then rebuilds the production shape.

Commands and output:

```
$ cargo test
lib: 8 passed; backoff: 2 passed; billing: 1 passed; connector: 1 passed;
prefixed_env: 1 passed; recognize_replay: 7 passed; refused: 1 passed;
settings_state: 2 passed; wire: 3 passed (skipped offline); doc tests: 1

$ cargo test --features synthetic-partial
lib: 8 passed (the marker test runs, the env test is out); the rest as above

$ git checkout c81679b^ -- src/lib.rs && cargo test --test settings_state
a_narrow_engine_keeps_its_width_beside_a_wide_one: left 3, right 1
the_file_descriptors_stay_flat: descriptors went 10 to 14

$ git checkout a06b70d^ -- src/lib.rs && cargo test --test refused
no backoff waits for a refusal, wall was 7.00242021s

$ git checkout a06b70d^ -- src/lib.rs && cargo test --test prefixed_env
THINKTHEN_WIDTH=4 beat ENGINE_WIDTH=1: left 1, right 4
```

## 2026-09-23 — review 5: the settings table under one lock

The lock-free table did not hold. Under ThreadSanitizer its own
`churn_probe` reported nine data races, each the tombstone write in
`vacate` against a reader's plain load of the same box, and the C door
crashed in two of fifty-four runs of the reviewer's `churn.c`. The table
is now a fixed array of `Option<Arc<Inner>>` behind one short lock. A
reader takes the lock, clones a counted `Arc`, and lets go; nothing ever
reads a state it does not hold a count on. The lock word stores the
holder's pid, so a child forked while a parent thread held it takes it
over instead of waiting forever. The tombstone, the retired stack, the
grace, and the never-freed boxes are gone: an evicted state is dropped
after the lock is released, and its pool closes when its last caller
finishes. A state inherited from a fork's parent is leaked, as before.

Evidence: ThreadSanitizer on `churn_probe`, 9 races before and 0 in three
runs after. `churn_probe` and `scale_probe` now talk to a loopback backend
that keeps its connections, so each state holds a real socket: the old
table left 6,995 and 605 descriptors open, and the new one stays within
three per slot.

## 2026-09-23 — review 5: a signal resumes the read

A host may install a handler without `SA_RESTART` (a child-exit handler,
a profiler timer), and the kernel may deliver the signal to the engine's
worker thread. The first no-resend fix turned the interrupted read into
the cancelled kind, so a delivered and billed call failed with code 5
although nobody cancelled. The verifier's `wsig.c` probe showed it at the
C door: code 5 at 50, 300 and 1,000 ms.

Every pool's connector chain now ends in `Resuming`, which wraps ureq's
default chain. An interrupted read consumed nothing, so the read starts
again on the same connection with the time it has left. The request is
never sent again. The read stops on an interruption only when the call's
own token fired. `post` holds that token in a thread-local for each send
and returns the cancelled kind then, for the head and the body alike.
Writes are not wrapped: they reach TCP through `write_all`, which
already resumes. An interruption that still surfaces with no fired token
is the backend kind and not retryable.

Known gap, found by the second-agent review: over TLS the wrapper sits
above rustls, and rustls 0.23 reads again after an interruption itself,
with the full timeout. On an HTTPS base the token is not checked on an
interruption, and a signal that repeats faster than the timeout can hold
the read past the deadline. The request is still never sent again. The
lever is a TCP-level wrapper under rustls that never returns an
interrupted read, since rustls would loop on one.

Evidence: `wsig.c` against the fixed library answered 0 with one body at
50, 300 and 1,000 ms. `tests/signal_worker.rs` failed before the change
with `Err(Error { kind: Cancelled, ... "a signal interrupted the send;
..." })` and passes after. `tests/signal_resend.rs` now expects the
caller-thread signal to answer with one body. The pool uses ureq's
`unversioned` transport API, which may change between minor versions;
the lock file pins 3.4.2.

## 2026-09-23 — wave 7: no request detaches a thread, and a refused thread start is an error

Every pool now resolves through the stand-in's own `Lookup` resolver
(R4-1, R7-1). ureq 3.4.2's default resolver starts a lookup thread for
every request that carries a timeout, an IP literal included, and drops
the handle. Dropping it calls `pthread_detach` on a thread that has just
sent its answer and is usually exiting. Both wave-7 churn cores fault
inside that call, on the read of the thread's record after the stack
holding it was unmapped. `Lookup` parses a numeric address on the
calling thread and starts no thread. A name still gets a lookup thread,
so the timeout still bounds a slow lookup, and the thread is joined when
it answers. Only a lookup that outlives its timeout is detached, and that
thread is still blocked in the lookup, not exiting.

Decision, which Ian can overturn: names keep a joined lookup thread
rather than a blocking lookup on the calling thread. A blocking lookup
would let a hung resolver hold a call past its deadline.

The batch path no longer panics when a thread cannot start (R4-12). A
refused first worker or a refused feeder fails the call with the defect
kind, marked retryable, and the message `the engine could not start a
thread: <the OS error>`. A refused later worker leaves the workers that
started to answer every record.

Decision, which Ian can overturn: the defect kind with `retryable` true.
No kind names a local resource, and the door already mapped the old
panic to the defect kind.

Evidence: `tests/thread_starts.rs` counts every `pthread_create` in its
process. At 14f12f5 a batch of 1,000 requests at width 1 started 1,003
threads, and a refused first worker panicked with `a worker thread
starts`. With the change the batch starts at most 3 and each refused
start answers as above. The churn counts are in the branch issue
`sdlc/issues/2026-09-23-the-c-door-churn-still-crashes-in-a-new-threads-start.md`.
