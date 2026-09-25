# The literal lines from the experiments

Status: Closed on 2026-09-25 as reference. No work is owed. Records the experiment bench lines as they ran; the ruled spellings shipped. Earlier status: Open

The talk and the site copy these lines, so each is complete enough to paste: imports or setup included where the experiment's own file shows them. Every line ran as written in the experiment folders, against the loopback stub, and the source file is named under each. The question text is the refund question the benches used; the wording varies by language because the benches were written independently, and that is faithful to what ran.

> The ruled C bulk spelling is `thinkthen_decide_many` (ADR 0017, pick 8). The C section below shows the literal line as it ran on 2026-09-21 under the older spelling `thinkthen_filter_many`; the ruled name lands with the contract.

## Python

From `experiments/205-thinkthen-libs/python/bench_width.py`:

```python
import thinkthen as tt

Q = tt.question(decide="Does the writer ask for a refund?", threshold=0.9)
kept = tt.filter(Q, records, jobs=32)
```

## Rust

From `experiments/205-thinkthen-libs/shared/engine/examples/bench.rs`:

```rust
let question = standin_engine::Question::from_json(QUESTION).expect("the question parses");
let records: Vec<String> = (0..1_000)
    .map(|place| {
        if place % 3 == 0 { format!("record {place} wants a refund") }
        else { format!("plain record {place}") }
    })
    .collect();
let kept = question.filter(&records, 32).expect("the filter runs");
```

## Ruby

From `experiments/205-thinkthen-libs/ruby/bench.rb`:

```ruby
require "thinkthen"

kept = ThinkThen.filter(question, records, jobs: 32)
```

## R

From `experiments/205-thinkthen-libs/r/bench_width.R`:

```r
library(thinkthen)

q <- tt_question(decide = "Does the writer ask for a refund?", threshold = 0.5)
records <- rep(c("i want a refund now", "good morning", "maybe later", "see you", "refund, please"), 200L)
kept <- tt_filter(q, records, jobs = 32)
```

## JavaScript and TypeScript

From `experiments/205-thinkthen-libs/javascript/bench/width.mjs`:

```js
import * as tt from "thinkthen";

for await (const record of tt.filter(question, records, { jobs: 32 })) kept.push(record);
```

## C

From `experiments/205-thinkthen-libs/c/bench.c`, the bulk form beside the JSON door:

```c
#include <thinkthen.h>

int rc = thinkthen_filter_many(QUESTION, strlen(QUESTION), recs, lens,
                               (size_t)n, 32, &err, &kept, &klen);
```

The JSON door beside it, as the same file runs it:

```c
char *answer = thinkthen_call(request, strlen(request), &len);
```

## DuckDB

From the 207 run, the shape the width and interrupt benches used (`experiments/207-thinkthen-db/duckdb/NOTES.md`):

```sql
INSTALL thinkthen FROM community;
LOAD thinkthen;

SELECT count(thinkthen_decide('The reviewer asks for a refund.', body)) FROM range(2048);
```

## SQLite

From the 207 container run (`experiments/207-thinkthen-db/sqlite/NOTES.md`):

```sql
.load ./libthinkthen0.so

CREATE TABLE t (text TEXT);
SELECT thinkthen_warm('The message asks for a refund.', text) FROM t;
SELECT count(*) FROM t WHERE thinkthen_decide('The message asks for a refund.', text);
```

## PostgreSQL

From the 207 width run (`experiments/207-thinkthen-db/postgres/NOTES.md`):

```sql
CREATE EXTENSION thinkthen;

SELECT thinkthen_warm('The reviewer asks for a refund.', body) FROM reviews;
SELECT id, body FROM reviews WHERE thinkthen_decide('The reviewer asks for a refund.', body);
```

## What is not here

The lines above are the experiment surfaces, not the ruled shapes. The ADR 0017 picks change some of the spellings — `decide_many` beside `filter` on Python, TypeScript, and Ruby, `ThinkThen.decide_many` with no question mark on Ruby, the Rust call over the blocking engine — and the real surfaces ship those spellings. When the real engine repeats the benches, this page gains the ruled lines beside the experimental ones, and the talk and the site quote the ruled ones.
