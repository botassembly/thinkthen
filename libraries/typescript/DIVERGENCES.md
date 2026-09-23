# TypeScript divergences from the Python surface

The second review found the two surfaces returning different shapes, with
no record of which difference was decided. This file is that record: the
Python surface's public shapes are the reference (it was first, and its
names come from the contract), and every difference is either aligned in
the wave of 2026-09-22 or recorded here with its reason. Nothing is left
silent.

## Aligned in this wave

| Shape | Python | TypeScript before | Now |
| --- | --- | --- | --- |
| The audit trail's answer | `{"answer", ...}` | `{"value", ...}` | `{"answer", ...}` — the contract's own field name and every other surface's key |
| `annotate` rows | one object a record, the set's fields only | the fields plus a `record` key | the fields only; the input record is `records[index]`, the shape the conformance rows pin |

The alignment also let two dead checks live again: the conformance runner
no longer deletes the extra key, and the annotate cases (15, 74, 82, 83,
84) now run instead of being recorded as divergences — the runner was
wrapping the set without the `version` the core's parser requires, and
the score cases' `nearest_level` is now asserted against
`details.nearest`.

## Recorded divergences, with reasons

1. **`find` when nothing fits.** Python returns `None` for the whole
   result; TypeScript returns the ruled pair with a null place —
   `{index: null, unit: null, probability}` — which keeps the probability
   the contract's `Found` carries, as Ruby and R do. The settlement
   ("`find` and `rank` return the ruled pair: the place and the
   probability, everywhere") favors this shape; the Python surface owns
   any change to its `None`.
2. **An edge's kind fields.** Python's `Edge` dataclass always carries
   `source_kind` and `target_kind` (nil when a rule named none);
   TypeScript omits the keys, matching the JSON door and the Ruby hash.
   A JavaScript object with undefined fields serializes them away, so the
   wire shape and the object shape agree only when the keys are omitted.
3. **The error classes.** Python raises six classes under a
   `ThinkThenError` base; TypeScript raises one `ThinkThenError` carrying
   `kind` and `retryable`. The six kind strings are the same everywhere;
   one class with a discriminant is the JavaScript host's own shape, as
   the class hierarchy is Python's. (Ruby, this wave, gained the shared
   base its review asked for.)
4. **The cancel spelling.** Python takes `token=CancelToken`, TypeScript
   takes `signal: AbortSignal`, Ruby takes `cancel: ThinkThen::Cancel`.
   Each host's own cancellation gesture, with the same semantics: no new
   request starts, sent requests finish, and an interrupt never fires a
   token the caller shared.
5. **The deadline unit.** Python, Ruby, R, C, and Rust take seconds.
   TypeScript takes `deadlineMs` in milliseconds, the unit of
   `Date.now()`. The rules are the same on all six surfaces, because each
   one converts through the contract's `deadline_from_seconds`: `-1` means
   no deadline, zero is a spent deadline, every other negative and a NaN
   are usage errors, and a budget above 4294967295 seconds is a usage
   error. Each host also spells no deadline its own way (`None`, `nil`,
   `NULL`, `null`, or a missing key). R takes an integer as well as a
   double, so `-1L` and `5L` read as `-1` and `5`. The seventh review's
   probes (R2-10) checked every spelling on all six surfaces. One cost
   remains: a computed `end - Date.now()` that lands exactly on `-1`
   means no deadline. `index.d.ts` documents it on `deadlineMs`.
6. **The question value.** Python's `tt.question(...)` returns a Question
   object (with `.digest()`); TypeScript's returns a function carrying the
   spec, so no spread or stringify turns it back into text by accident.
   Both feed every verb.
7. **The `recognize` offsets.** Python counts code points, so
   `text[start:end]` slices the name; TypeScript converts the contract's
   code points to UTF-16 units, so `text.slice(start, end)` slices it.
   Host indexing; both are proven by the same recordings.
8. **The column forms.** Python's `decide` and `score` take a Polars
   Series or a frame column and return a column, and `annotate`,
   `recognize`, and `relate` take frames (`on=`); TypeScript has no column
   door. The Polars door is Python's Arrow work; a Node column form needs
   its own design and is not in this release.
9. **Blocking against async.** Python's verbs block the calling thread.
   TypeScript's return Promises, and each call runs on one worker of
   libuv's thread pool for its whole length. Node's event loop must not
   block, and the engine call is the same blocking call underneath.
   The pool is shared. File reads, `dns.lookup`, `crypto`, and `zlib` run
   on it too, and it holds `UV_THREADPOOL_SIZE` workers, 4 by default. A
   fifth call in flight waits for a worker, and so does that other work.
   The seventh review measured it (R2-27, R3-25): 8 decides against a
   stub answering in 500 ms took 1053 ms on a pool of 4, and an
   `fs.readFile` issued during them waited 1000 ms. On a pool of 8 the
   decides took 539 ms and the read waited 484 ms. A caller that runs
   many calls at once sets `UV_THREADPOOL_SIZE` (at most 1024) in the
   environment before Node starts, to the calls it runs at once plus the
   other pool work it needs. A bulk verb (`decideMany`, `filter`, `rank`,
   `annotate`) sends a whole column from one worker at the engine's
   width, so a column is one call and holds one worker.
   Decision: the calls stay on the pool. A thread of their own per call
   would free the pool, but it would put no bound on the threads a
   `Promise.all` over many single calls starts. The pool is that bound.
   Ian can overturn this; the other choice spawns a thread per call and
   resolves the Promise from it.
10. **The record text.** Python requires `str` records (a non-string is a
    type error); TypeScript requires strings; Ruby, this wave, serializes
    a non-string record as its JSON text. Ruby is the permissive one; the
    two typed hosts refuse rather than guess.

## Checked and the same

The verb names and their arities; `nil`/`None`/`null` as "not sure"; the
band as the host's pair; `decide_many`, `filter`, `rank`, `choose`,
`score`, `tag`, `recognize`, and `relate` field names (`strength` on a
name, `probability` on a relation or edge, `source` and `target` ends);
the failed-question marker's shape (`{failed: {kind, cause}}`, never
null); the `usage` counters (`requests`, `cache_answers`, `tokens`); and
the six kind strings on every error.
