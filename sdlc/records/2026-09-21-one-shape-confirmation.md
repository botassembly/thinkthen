# The one-shape confirmation, 2026-09-21

The hand-items lane (Ian's hand-off, item 2) ran each of the ten picks from `sdlc/issues/2026-09-21-one-shape-for-nine-surfaces-as-the-slides-show-it.md` against what branch `surfaces` ships today, after the settle wave (`84a8558`). Each entry is CONFIRMED with its evidence or DIFFERS with the recorded reason and owner. The commands and outputs behind the Polars mirror are at the end.

1. **The same names everywhere. CONFIRMED.** `functions.toml` holds the fourteen ruled names — the eight verbs, `decide_many`, `question`, `details`, `usage`, `recognize`, `relate` — and `python3 scripts/check_public_names.py` prints "every surface's public names are ruled or documented." The prefixes are the host's habit: `tt.` (`libraries/python/thinkthen/__init__.py`), `ThinkThen.` (`libraries/ruby/lib/thinkthen.rb`), `tt_` (`libraries/r/thinkthen/R/thinkthen.R:149`), `thinkthen::` (`libraries/rust/src/lib.rs:84`), `thinkthen_` (`contract/include/thinkthen.h:83`), SQL (`databases/*/src`). Ruby's `decide?` is gone: zero matches in the Ruby source.

2. **The first argument is the question, as text or as a built question. CONFIRMED.** Python's `decide(question, text)` accepts both (`libraries/python/thinkthen/__init__.py:59`, with `tt.question(...)` building the other form). Rust's `IntoQuestion` covers both spellings (`libraries/rust/src/lib.rs:372` with `impl IntoQuestion for &Question` at `:382` and `for &str` at `:388`). The contract's question builder replaces the hand-formatted JSON every shim wrote in 205; Ruby (`lib/thinkthen.rb:108`), R (`R/thinkthen.R:149`), TypeScript, and C all route through it.

3. **"Not sure" is the host's own empty value. CONFIRMED.** Python `None`, TypeScript `null` (`index.d.ts:2, 69`), Ruby `nil` (`lib/thinkthen.rb:10`), R `NA` (`R/thinkthen.R:3`), SQL `NULL`; Rust and C take the enum arms (`Answer::Unsure`, `THINKTHEN_UNSURE` at `contract/include/thinkthen.h:51`). The planning pages' band `Outcome` types are gone, per the pick; the teaching mitigation is in the band examples and the conformance case.

4. **The public word is "unsure", the specification's is "unresolved". CONFIRMED.** The enum serializes as `unsure` with its test (`contract/src/lib.rs:2111`), and the C constant follows (`THINKTHEN_UNSURE 2`). The specification keeps `unresolved` in its own grammar (`specification/annotate.md:22, 47`).

5. **A band is the host's pair. CONFIRMED.** Rust `.band(0.2, 0.8)`, Python `threshold=(0.2, 0.8)`, TypeScript `[0.2, 0.8]` (`index.d.ts:10`), Ruby `0.2..0.8` (`lib/thinkthen.rb:6`), R the pair (`R/thinkthen.R`, threshold vector), and the `"0.2:0.8"` string stays in question files and on the command line only.

6. **`score` returns a number, and the nearest level's name is in `details`. CONFIRMED.** The contract's `Details` now carries `nearest: String` (`contract/src/lib.rs:362-363`), score documents the specification's position (`:352-356`), and the settle wave wired it through Python, R, C's audit JSON, and all three databases (`8a33578`, `2deb4ae`, `d77d844`, `9686899`). The two false doc sentences the audit found are corrected.

7. **Rust is blocking. CONFIRMED.** Zero `async` or `.await` matches in `libraries/rust/src/lib.rs`; calls return `Result`, and experiment 211's 9.666 s matches the async bench.

8. **Bulk is the same verbs over the host's container. CONFIRMED.** `filter`, `rank`, and `annotate` take containers and cross once; `decide_many` is present on Python, TypeScript, and Ruby as ruled (`functions.toml` row, `__init__.py`, `index.d.ts`, `lib/thinkthen.rb`), with C's `thinkthen_decide_many` admitted in the surface check; R keeps the vectorized `decide`; SQL keeps `thinkthen_warm`.

9. **SQL names a question file as `'@refund.json'`. CONFIRMED.** The `@` prefix parses in every SQL door (`databases/duckdb/src/lib.rs:142, 160, 693`, `relate.rs:277`; SQLite and PostgreSQL through the same contract parser), and where the file may be read from stays the database ADR's question.

10. **DuckDB's `thinkthen_warm` exists. CONFIRMED.** Registered in all three databases (`databases/duckdb/src/warm.rs`, `databases/sqlite/src/lib.rs`, `databases/postgresql/src/lib.rs`), closing the page's open item per experiment 207.

## The Polars mirror

The rule: what goes in decides what comes out, no new names, no separate module. Run as drawn, in `libraries/python` against the stand-in on the null backend:

```
$ ENGINE_NULL=1 .venv/bin/python
>>> import thinkthen as tt, polars as pl
>>> df = pl.DataFrame({"body": ["I was charged twice. Can you fix this?"]})
>>> r1 = tt.decide("Does the customer ask for a refund?", df["body"])
>>> print(type(r1).__name__, r1.to_list())
Series [False]
>>> r2 = tt.score("How urgent is this?", df["body"], ["Routine.", "Soon.", "Immediate."])
>>> print(type(r2).__name__, r2.to_list())
Series [0.99]
>>> df3 = tt.annotate("tests/fixture/form.json", df, on="body")
>>> print(df3.columns)
['body', 'team', 'urgency', 'wants_refund']
>>> dfr = pl.DataFrame({"body": ["Maria Chen joined Northwind Freight in Chicago last spring."]})
>>> rec = tt.recognize(dfr, on="body", kinds=["person", "organization", "place"])
>>> print(type(rec).__name__, rec.columns, rec.height)
DataFrame ['row', 'text', 'kind', 'start', 'end', 'strength'] 3
>>> print(rec.row(0))
(1, 'Maria Chen', 'person', 0, 10, 0.98)
>>> edges = tt.relate(pl.DataFrame({"body": records_from_case_69}), on="body", relations=[{"name": "caused_by", "source": "*", "target": "*"}, {"name": "same_as", "source": "*", "target": "*", "either": true}])
>>> print(type(edges).__name__, edges.columns, edges.height)
DataFrame ['name', 'source', 'target', 'probability'] 4
>>> print(edges.row(0))
('caused_by', 1, 2, 0.59)
```

A column in comes back a column out, under the same verb names — `tt.decide`, `tt.score` — and a frame in comes back a frame out under `tt.annotate`, `tt.recognize`, and `tt.relate`. No Polars module exists, no name in `functions.toml` is Polars-specific, and the plain-list form of every verb is unchanged beside it. Two notes from the runs: `decide` and `score` over a column return Polars `Series` objects (the settle wave's column forms); `relate` answered only the recorded case's records, because the stand-in answers from its recordings — the first unrecorded text refuses by name, which is the stand-in's designed behavior, not a shape gap. The walk is a code-point walk (`text[start:end]`), pinned by the emoji test.

## The rest of the hand-off

**The fork proof per language (item 6 in Ian's hand-off).** A fork after the first call answers in the child, on the null backend, each with a watchdog so a hang is a failure:

| Host | Proof | Result |
| --- | --- | --- |
| Python | the ten-round fork-during-batch test (211, carried) | passing before this lane |
| Rust | the engine's own `libc::fork` test, `standin/tests/wire.rs:163` (`a_forked_child_answers_on_the_wire`) | cited, not duplicated |
| C | `libraries/c/tests/fork.rs`, forked after the first door call under a 10 s alarm | `1 passed` |
| TypeScript | `libraries/typescript/tests/fork.test.mjs`: `child_process.fork()` and a `worker_threads` worker made after the first call, 10 s watchdog | `2 passed` |
| Ruby | `libraries/ruby/tests/test_fork.rb`, `Process.fork` after the first call, 10 s `Timeout` | passing in the builder container |
| R | `libraries/r/fork_check.R`, `mclapply` children forked after the first call, `timeout 60` in `check.sh` | passing |

Each new proof is wired into its surface's `check.sh` in the null section, so the gate carries it from now on.

**The jobs sentence (item 7).** One sentence landed in all nine surface READMEs and in the contract's `Settings.width` doc: the width is the number of requests in flight, and each in-flight request holds its own connection — 1,000 records at width 32 measured 33 pooled connections. The evidence is the width bench's stub counters, recorded in the lane notes.

**The blocker proposals (items 4 and 5).** Written in final form at item 9 of `MERGE-NOTE-INPUT.md`: SQLite — the direct link with the load-time floor, and the recommendation not to file upstream now; DuckDB — the vendored bind-callback patch as our road, the C++ fork left to the build team, the upstream pull request proposed only after the patch runs in our tree. Both marked `Ian decides the outward act`.
