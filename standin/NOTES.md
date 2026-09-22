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
