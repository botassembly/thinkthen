# The Rust surface

Landed 2026-09-21. The crate `thinkthen`: an `Engine` value from the
environment, every verb in the ruled shape, blocking calls returning
`Result`, and the stand-in behind one dependency line. Run `./check.sh`
for the null suite, the conformance slice, and the wire suite when the
stub is up on 8213. Findings and quirks are in `NOTES.md`; the slide's
one-character finding is filed there for the slide owner.

The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the library:

```rust
use thinkthen::{Answer, Engine, Question};

let tt = Engine::from_env()?;
let refund = Question::decide("...").band(0.2, 0.8)?;
match tt.decide(&refund, &ticket.body)? {
    Answer::Yes => refunds.push(ticket),
    Answer::No => {}
    Answer::Unsure => review.push(ticket),
}
let complaints = tt.filter("Is this a complaint?", &reviews)?;
```

The engine itself, with no binding in between. Blocking calls, and no async
runtime comes with it. `Answer::Unsure` is a checked arm.

## The Polars Series door, behind a feature flag

`polars = ["dep:polars"]` is off by default: the core surface compiles
with the pinned 1.93.1 toolchain and no Polars. With the feature on, the
crate needs the 1.95 toolchain Polars requires (experiment 228 records
why: `sysinfo 0.39` refuses 1.93), and `Engine` gains five methods:

- `decide_column(question, &Series)` — the whole column crosses once
  through the batch spine at the process width gate; the answers come
  back as a boolean column whose nulls are "not sure".
- `choose_column`, `score_column`, `tag_column` — per-text answers as a
  text, number, and list column. The stand-in's contract carries no bulk
  form for these three, so they run one call a record; `decide` is the
  batch spine.
- `annotate_frame(&QuestionSet, &DataFrame, on)` — the caller's columns
  come back unchanged with one new column a question, in the set's name
  order; a failed member widens that column to text carrying the ruled
  marker, the same widening the other frame doors use.

A text column crosses zero-copy: each row is a `&str` read out of the
producer's own UTF-8 buffer. The equality expectation: **a column
crosses at the same width as a slice** — one crossing, 32 in flight, the
answers in input order, the same request count. The tests behind the
feature prove the answers, the order, and the request-count equality
against the slice form; experiment 213 measured the width itself at 32
in flight for both.

```rust
use polars::prelude::{DataFrame, Series};
let df = DataFrame::new(vec![/* ... */])?;
let out = tt.annotate_frame(&set, &df, "body")?;
let answers = tt.decide_column("Is this a complaint?", &df.column("body")?.as_materialized_series())?;
```
