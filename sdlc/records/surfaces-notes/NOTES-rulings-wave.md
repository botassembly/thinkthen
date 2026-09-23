# The rulings fix wave, 2026-09-21

The wave's verification outputs, pasted as they ran. The rulings themselves
are `sdlc/issues/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`;
the cross-side handoff is `MERGE-NOTE.md`; the review that produced it all
is `sdlc/issues/2026-09-21-adversarial-review-of-the-surfaces-and-experiments.md`.

## Ruling 4: the public-name check

```
$ python3 scripts/check_public_names.py
ok python: 25 names, all ruled or documented
ok typescript: 15 names, all ruled or documented
ok ruby: 29 names, all ruled or documented
ok r: 13 names, all ruled or documented
ok rust: 20 names, all ruled or documented
ok c: 10 names, all ruled or documented
ok duckdb: 12 names, all ruled or documented
ok sqlite: 10 names, all ruled or documented
ok postgresql: 13 names, all ruled or documented
every surface's public names are ruled or documented
```

The injection proof: `reset_usage` added back to the Python `__all__`, and
the check fails naming it (exit 1):

```
$ python3 scripts/check_public_names.py   # with reset_usage injected
     FAIL python: 26 public names vs 25 expected
  extra:   reset_usage
  a new public name needs a ruling and a line here
```

The injection was removed after the proof. The check caught real gaps when
first run: Python's two stream helpers left `__all__`; R's
`exportPattern("^tt_")` exported twenty internals (`tt_call`, `tt_raise`,
the `tt_*_one`/`tt_*_column` wrappers) — the NAMESPACE now names the
thirteen public functions explicitly.

## Ruling 6: the rung

```
$ sh sdlc/scripts/surfaces           # offline half, then the full check
... generated files match functions.toml (14 functions)
... every surface's public names are ruled or documented
... OK: 72 cases validated: schema, grammar, digests, wire contract, offline replay
... all landed checks green
```

The skip path, with cargo absent from PATH (exit 0):

```
$ env PATH=/tmp/rt /bin/sh sdlc/scripts/surfaces
... the offline checks above ran
surfaces: cargo is not on PATH; the offline checks above ran, the full surfaces check skipped
```

## The full run with stubs on every port the surfaces expect

Stubs on 8211 through 8219 and 8231, `STUB_DELAY_MS=300`, then:

```
$ sh sdlc/scripts/surfaces
... every surface section ran with its wire suite ...
== postgres surface: wire suite against the stub on 8219
wire green: decide answers on the wire, usage counts sends and tokens
all landed checks green
```

Pasted output is in `/tmp/full-rung.log` at the time of writing; the
verdict line is the done bar.

## 2026-09-23: the stand-in's state table, review finding 15 (refs surfaces-review-3)

Commit: the state-table fix on `surfaces` (this wave). Probes pasted from the runs in this lane's transcript.

**The leak probe, fail-then-pass.** Pre-fix mimic (eviction leaks the box on purpose, `STATE_SLOTS` back to 16):

```
$ cargo test --lib retirement
thread 'tests::retirement_closes_states_rather_than_leaking' panicked at src/lib.rs:1854:9:
test result: FAILED. 0 passed; 1 failed; ...
$ # fix applied (retire + grace + sweep):
$ cargo test --lib retirement
test result: ok. 1 passed; 0 failed; ... finished in 0.08s
```

The test churns 100 settings values past 64 slots and counts `Inner` drops: the mimic drops nothing (the reviewer's "40 values, open files 4 to 44" leak); the fix drops every evicted state once the 50 ms test grace passes and a sweep runs.

**The width probe.** `width_one_holds_through_churn` holds a width-1 engine against a never-answering listener while 300 other settings values churn the table, and asserts the wire's own peak (`WIRE_PEAK`, counted at the send in `post`) stays exactly one after the first publish settles. It passed post-fix in every solo run, and it failed the pre-fix mimic earlier in this lane (`left: 2` pasted at that point in the transcript) — but that discrimination did not reproduce against the final code's structure within this lane's budget, so the probe ships with this caveat, not a clean fail-then-pass claim. It CAN fail: under the full lib suite it is intermittently red (about 2 of 25 runs) — a residual duplicate-state window under heavy contention that the home-claim plus retract-and-retry dedup narrows but does not provably close. That residual is OPEN: owner, the next surfaces pass; the structural defects (the permanent leak; eviction of busy states; blind home-slot replacement) are fixed and the leak proof above is deterministic.

**What landed in the fix:** one state per settings value behind atomic slots; evictions pick a vanished pid's idle state first, then the least-recently-used idle one (a logical lookup clock), and a busy or caller-held state only when every slot is; the home slot is claimed first so two publishers of one settings value race one compare-and-swap; every publish passes a full dedup scan whose retraction retires behind a five-second grace and retries the lookup; retired pools close once no request holds them, keeping file descriptors flat; the fork rule holds throughout — no lock is taken on the rebuild path that any request path holds.

## One deadline spelling (review finding 21) — landed as ADR 0041

Status: ACCEPTED as `sdlc/planning/adr/0041-one-deadline-spelling-across-every-surface.md`; the text below is the working record the ADR superseded.

**Decision.** Exactly one spelling of "no deadline" exists across every surface: the sentinel −1, the C header's `THINKTHEN_NO_DEADLINE`. Every other negative value is refused with the usage kind. Zero is a spent deadline, legal, and the call returns the deadline kind having sent nothing. The contract owns the conversion and the sentinel (`deadline_from_seconds` / `deadline_from_millis`, `NO_DEADLINE = -1.0`) and its tests pin all of this.

**Adoption state.** C, Ruby, R, and the contract already follow the rule (review 3 confirmed each). Python and Node currently REFUSE −1; they adopt "−1 means none" through the contract's converter in the next surfaces pass. Node additionally stops coercing `true`, `"5"`, and `[]` into numbers, and documents that a computed budget must clamp to zero (`Math.max(0, end - now)`) because −1 is reserved. DuckDB gains a deadline door when its phase lands.

**Why −1 and not a separate explicit-none argument.** The C ABI is flat scalars; a second boolean argument would touch every signature the deck draws, and −1 is already the shipped, tested spelling on four surfaces. The hazard — a computed budget landing on −1 by accident — is answered by the clamp rule, which is the semantically correct statement anyway: a deadline that already passed is zero, not none.

## 2026-09-23: the one skip-table reader and the corrected reasons (refs surfaces-review-3)

`conformance/skiptable.py` is the single reader: `lookup SURFACE CASE_ID [--verb V] [--kind K] [--form F] [--record R]` prints `RUN`, `SKIP<TAB>why`, or `DIVERGE<TAB>why`; `validate` checks the table's structure (22 entries valid at commit time). The table's five selector kinds — `id`, `verb`, `kind`, `form`, `record` — are exactly the fields the nine private readers disagreed on; this reader names them all, entries with no surface list apply to every surface, and an `id` selector wins over facet selectors.

The two false reasons ("the stand-in ignores a pre-fired token") are corrected in the data: the stand-in refuses a pre-fired token where the surfaces accept one, and DIVERGENCES.md carries that as the real-engine requirement.

Adoption contract for the surface lanes (phase 2): each runner replaces its private skip/divergence logic with one call — `python3 ../../conformance/skiptable.py lookup <surface> <case-id> --verb <verb> --kind <kind> --form <form> --record <record>` as its facets apply — and moves any skip still living only in the runner's own code into the table. Until adoption lands, the reader and the private lists coexist; the reader is the source of truth for the entries the table already carries.

**Open, exactly stated:** the nine runners still carry their private paths (SQLite, DuckDB, C, Ruby, R, PostgreSQL were named by the review); adoption is surface-folder work and was outside this lane's contract/standin-only scope.
