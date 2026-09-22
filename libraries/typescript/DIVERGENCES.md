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
5. **The deadline spelling and the negative budget.** Python takes
   `deadline=` in seconds and refuses every negative (its `None` is the
   one no-deadline spelling); TypeScript takes `deadlineMs=` in
   milliseconds, refuses every negative including the contract's `-1`
   sentinel, and spells no deadline as `null` or by leaving the key out.
   Ruby takes `deadline=` in seconds and still honors `-1` as the
   contract's sentinel, as its wave-1 test pins. A computed
   `end - Date.now()` landing on `-1` must never disable a deadline, so
   the millisecond hosts refuse it; Ruby's seconds spelling and its
   documented sentinel are the reason it does not, and a ruling could
   align it.
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
9. **Blocking against async.** Python's verbs block the calling thread;
   TypeScript's return Promises and run on libuv worker threads. Node's
   event loop must not block, and the engine call is the same blocking
   call underneath.
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
