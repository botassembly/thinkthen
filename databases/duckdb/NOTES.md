# NOTES

Running log. Commands and output as they happened.

## 2026-09-21 — the DuckDB surface lands

The recipe is experiment 207's, read from `experiments/207-thinkthen-db/duckdb/`: the extension template on the `duckdb` crate `~1.10505.0` with `loadable-extension`, `vscalar`, and `vscalar-arrow`; `TARGET_DUCKDB_VERSION=v1.5.5` in the Makefile (the version-pin trap: the machine CLI is v1.1.3 and cannot load what the unstable C API builds). The stock v1.5.5 CLI fetched at user level into this folder (removal: `rm -rf duckdb-bin`; nothing outside the folder, no rc change):

```
$ curl -sSL -o duckdb.zip https://github.com/duckdb/duckdb/releases/download/v1.5.5/duckdb_cli-linux-amd64.zip
$ ./duckdb-bin/duckdb --version
v1.5.5 (Variegata) d8cdaa33fd
```

`make configure && make release` produced `build/release/thinkthen.duckdb_extension` (4.7 MB). The crate binds `thinkthen-contract` and `thinkthen-standin` by path and calls only the contract's doors; `crates/`, `contract/`, and `standin/` untouched.

**First smoke, null backend:**

```
$ ENGINE_NULL=1 ./duckdb-bin/duckdb -unsigned -c "LOAD '...'; SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today') AS yes, thinkthen_decide('Is this a complaint?', 'thanks for the help') AS no, thinkthen_probability('Is this a complaint?', 'I demand a refund today');"
│ true    │ false   │   0.97 │
```

**Two bugs the suites caught.** The details struct's string children leaked stale memory into unwritten rows (`'level': Is this a complaint?hipping label`) — the write path set no NULL where a field did not apply; every child now writes or nulls its row. And `thinkthen_details` over a score question failed outright ("the answer carries no probability"), because a score reply carries a level distribution and no yes-probability: the audit door is decide-shaped. The ruled shape taken: one struct for every verb, probability/answer/sends NULL on the non-decide kinds, the question's own model and digest carrying the audit, the score's nearest level in `level`. Both fixes verified in the null suite.

**Wire, stub on 8217** (`experiments/205-thinkthen-libs/shared/target/release/stub-backend`, `STUB_PORT=8217`):

```
SELECT thinkthen_decide('@tools_fixture.json', 'I demand a refund today');  -> true   (the @file carries threshold 0.9)
SELECT thinkthen_warm('Is this a complaint?', t) FROM (SELECT unnest(['I demand a refund today','refund my shipping']) t);  -> 2
SELECT thinkthen_decide('Is this a complaint?', NULL);  -> NULL
dead address (port 9):  Invalid Input Error: thinkthen backend: io: Connection refused ... the address refused the connection
malformed evidence:     Invalid Input Error: thinkthen backend: HTTP 422: ...
```

The stub's counters after the wire suite: `{"connections":9,"max_in_flight":4,"requests":7}` — three sends for the WHERE, four for the warm, nothing extra for the repeats.

**The slide, as drawn.** `tools/run_slide.sh` extracts the DuckDB block from the marketing `surfaces.md` verbatim into `tools/slide.sql` (nothing retyped), builds the `tickets.parquet` fixture with the same CLI, and runs the block in the stock v1.5.5 CLI with the LOAD riding an init file. Offline:

```
WHERE thinkthen_decide('Is this a complaint?', body);   -> rows 1 and 2 (the refund bodies)
the choose/score statement ordered all four rows, team column NULL on every row
```

**Finding for the slide's owner, reported not hidden.** The sample's `thinkthen_choose('Which team owns this?', body, ['billing','shipping','account'])` reads NULL against the offline stand-in — its rules lift an option only when the option's own text carries a keyword, and none of the three does — and the statement cannot run on the loopback stub at all, because the stub answers one probability per request and a choice question needs a distribution (`thinkthen backend: the answer to q1 is not the shape the question asked for`). This is the recorded stub limitation behind the five shaped-to-contract conformance exchanges. The sample's shape is proven; its values await a backend that distinguishes options. Same class as the Ruby score finding the orchestrator filed in the marketing repo.

**Conformance slice, offline:** 16 ok, 3 diverge, 0 failed. The divergences, with reasons: `09-usage-filter-band` (SQL spells filter as WHERE, and a band under WHERE reads as NULL by rule 6; the refusal belongs to the surface check, not a function), `17-usage-and-cache` (the stand-in has no disk cache; the named gap the other surfaces carry too), `18-cancel-mid-batch` (the CLI's Ctrl-C owns cancel here; the SIGINT-at-LOAD chaining is the planning page's proven behavior).

**Full check, `./check.sh`:** build, 17/17 null suite, conformance slice, the slide, and the wire suite (WHERE over parquet on the wire, warm, counters) — all green with the stub up.

**Installs and removals this lane made:** the v1.5.5 CLI binary into `duckdb-bin/` (`rm -rf duckdb-bin` removes it), `make configure`'s python venv inside `configure/` (removed with `make clean_all` or `rm -rf configure`), cargo build outputs in `target/` and `build/`. No container was needed. No sudo, no key, no paid call, nothing published.

**Runtime-installer guard, checked after the CLI fetch and again at close:**

```
$ grep -c deno ~/.zshrc
0
```

**What was not run (unchecked):** the community-repository signing path (unsigned local LOAD only); `thinkthen_usage` under concurrent CLI processes; the 2,048-row width bench (207's D1 stands; the chunk path here is the same grouping); macOS packaging (the rehearsal's lane owns it).
