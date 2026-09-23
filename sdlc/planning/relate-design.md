# `relate`: the design for the command and the nine surfaces

Status: Proposed by the marketing side on 2026-09-21, which holds the product and library shape job by Ian's word. Ian ruled the same day: "Let's have them build relate at the same time." `relate` is the tenth function and is built with `recognize`. The build team reviews this page before any ticket. It authorizes no build and no paid run.

Read `recognize-design.md` first. `relate` reuses its relation rule unchanged, and step three of `recognize` is `relate` run over the names found in a text. The engine holds one pair-asking path for both.

## The one line

**Find records that clash, repeat, or rely on each other.**

Beta, like the relations in `recognize`, and every page says so in the same sentence.

## The command

```
thinkthen relate [OPTIONS] [RELATION]...
thinkthen relate [OPTIONS] @links.json
```

The relations sit where `tag` puts its labels. A bare name is a one-way relation between any two records.

```
thinkthen relate caused_by --either same_as --lines < alerts.txt
thinkthen relate covers=test:requirement --jsonl --kind-field /type < items.jsonl
```

| Option | Does |
| --- | --- |
| `RELATION` as `NAME` or `NAME=FROM:TO` | A one-way rule. `NAME` alone means `NAME=*:*`. `FROM` and `TO` are a kind or `*` |
| `--either NAME` or `--either NAME=KIND:KIND` | A rule that reads the same both ways. Asked once per pair |
| `--kind-field POINTER` | Where each JSON record keeps its kind. Without it every record is kind `*`, and a rule that names a kind is a usage error |
| `--threshold T` | The bar an edge must reach. Default 0.5 |
| `--details`, `--input`, `--lines`, `--jsonl`, `--csv`, `--tsv`, `--field`, `--dry-run`, `--cache`, `--replay`, `--no-cache` | As on every function |

A question file carries the same rules with a `reads` phrase for each, exactly as in `recognize-design.md`, under the key `relate`.

Exit codes: 0 when the run finished, including no edges. 2, 4, 5, and 70 as everywhere.

## How it asks

Ian ruled the method on 2026-09-21: it is organized around choices.

- The rules and the kinds give the list of legal pairs. A record is never paired with itself.
- Each unordered pair is one pick-one question. The options are the relations that pair allows, each way round where the rule is one-way, plus "no relation". The user never writes "no relation".
- All the records cross once, and as many pairs as the backend's question limit allows ride in each request.
- `--dry-run` prints the pair count and the request count. Pairs grow with the square of the records. `relate` refuses more than 255 records, the `find` limit, with exit 2.

## What comes back

One JSON object per edge, one per line, so the output pipes:

```
{"name":"caused_by","source":1,"target":4,"probability":0.94}
{"name":"caused_by","source":2,"target":4,"probability":0.94}
```

- `source` and `target` are record numbers, counted from 1 in input order. `source` is the subject and `target` is the object: record 1 was caused by record 4.
- `--details` adds both records' text and every option's probability.
- An `--either` edge prints once, with the lower record number in `source`.
- `probability` is the probability of the picked option. It is a plain probability, so the vocabulary's word holds.

**One thing to reconcile.** `recognize-design.md` calls the number on a relation `confidence`, because the recognize specification discounts it by the margin. One engine path should print one number under one name. The suggestion: relations print `probability` in both functions, and the margin discount stays on names only.

## The first real output

`experiments/225-relate-demo/`, run on 2026-09-21 through the live guard. Four made-up alerts, two rules, six pairs, one request, 1,365 input tokens. At the default bar the model returned four `caused_by` edges. Two were sound, at 0.94 each. One tied a late export to a full disk at 0.84, which the text does not support. One split between `caused_by` at 0.59 and `same_as` at 0.31. A bar of 0.9 kept the two sound edges. The deck slide shows both runs, because the number and the bar are the product.

## The libraries and the databases

| Surface | The call | Returns |
| --- | --- | --- |
| Python | `tt.relate(alerts, relations=["caused_by"], either=["same_as"])` | A list of edges. `tt.relate(df, on="body", ...)` returns a DataFrame of edges with the source indexes, ready for `networkx.from_pandas_edgelist` |
| TypeScript | `await tt.relate(alerts, { relations: ["caused_by"], either: ["same_as"], signal })` | `Edge[]` |
| Ruby | `ThinkThen.relate(alerts, relations: %w[caused_by], either: %w[same_as])` | An array of structs |
| R | `tt_relate(alerts$body, relations = "caused_by", either = "same_as")` | A data frame of edges, ready for `igraph::graph_from_data_frame` |
| Rust | `tt.relate(&Relate::new().relation("caused_by", Kind::Any, Kind::Any)?.either("same_as", Kind::Any)?, &alerts)?` | `Vec<Edge>` |
| C | `thinkthen_relate(tt, spec_json, texts, lens, count, &out_json, &out_len)` | The edges as a JSON string |
| DuckDB | `SELECT * FROM thinkthen_relate((SELECT id, body FROM alerts), ['caused_by'])` | Rows `(name, from_id, to_id, probability)` |
| SQLite | `thinkthen_relate('alerts', 'id', 'body', 'caused_by')`, table-valued | The same rows |
| PostgreSQL | `thinkthen_relate('SELECT id, body FROM alerts', ARRAY['caused_by'])`, set-returning | The same rows |

`relate` is the one function a database cannot run row by row, because it needs every record at once. Each engine therefore takes a table or a query. Edges as rows are what a recursive query walks, and the manual shows one.

## What is open for the build team

1. The `probability` and `confidence` question above.
2. A pick-one question allows one relation per pair. The experiment brief measures how often that loses a true second relation.
3. The one-question-per-subject form for a relation where a subject has one object. It costs one question per record. The brief measures it against pairs, and `find --in` is the same form.
4. The record limit of 255 is a guess taken from `find`.

## One tool, ten functions

`--details` prints the standard result object, `thinkthen.result/1`, with this function's value in `value` and the same `question`, `answer`, and `meta` keys as the other functions. The question file grammar, the exit-code table, the cache, and the recording are the same. A caller that handles one function's result handles this one. A question that fails inside a request that otherwise succeeded is marked on that answer and counted in `meta`. It never prints `null`, because `null` means "not sure".

## Ruled 2026-09-21: `relate` reads records, and names the user already has are records

The recognize team's closing note says `relate` works "over records or provided entities". The product side rules one form. `relate` reads records. A user who already has names passes each name as a record, with its kind in the field `--kind-field` points at, and the relation rules apply to those kinds. No second input form exists.

One case is different: names inside one text, where the sentence around them decides the relation. That is `recognize` given names the user already found. It stays in the backlog, in `sdlc/issues/2026-09-21-candidates-for-a-tenth-function-relate-and-find-in.md`. Ian can overturn this.

## Ruled 2026-09-21: a relation's ends are `source` and `target`, everywhere

`from` is a reserved word in Python and in SQL, so four of the nine surfaces could never say it. The library team chose `source` and `target` for every host. The product side extends that to the command's own JSON and to the question file, so no door converts anything and a user sees one pair of words on every surface. The rule on the command line is unchanged, `--relation NAME=FROM:TO`, because it names no field. This section overrides any older line on this page that says `from`, `to`, `head`, or `tail`. Ian can overturn it.

## Direction 2026-09-23: rebuild the graph from strings, and relate plans its own questions

Ian's direction on 2026-09-23. The user hands relate a list of entities, each a name and a kind, the same shape recognize returns: `John Lennon, person`, `Octopus's Garden, song`, `Help!, album`, `Help!, song`. The user also names the relations: `sung_by=song:person`, `appears_on=song:album`. Relate returns the graph as edges. The user never picks a method, and relate plans the questions per relation.

- A relation between two different kinds asks one `choose` per source entity over the legal targets plus "none". Every option above the cut becomes an edge, so a duet yields two `sung_by` edges.
- A relation within one kind (duplicates, contradicts, causes) asks method H.
- One entity name may carry two kinds (`Help!` the song and `Help!` the album). An entity is its name plus its kind.
- Recognize's relation step runs the same planner over the names it found. `recognize | relate` and relate over a user's own list give one output shape.

The open question is whether the kind shape alone picks the method, or whether a rule needs a `one` or `many` marker. Choose splits one probability across its picks, so a source with three true targets may fall under the cut (experiment 237, duplicate clusters). The Beatles graph test answers it: about 200 official songs, the four Beatles, and the albums, with sourced truth from `experiments/238-beatles-real-data/data/songs.tsv`. That test runs the planner against method H alone. Ticket 0081's design review waits for it. The edge shape does not change.

## Ruled 2026-09-23, revised after experiment 237: method H for every relation

Ian passed this to the build team on 2026-09-23, after the bake-off in `sdlc/issues/2026-09-23-relate-methods-bake-off.md` (`2a8d43f`). Every relation, and recognize's relation step, uses method H. H asks one yes/no per pair per relation, and the wording the questions share rides once per request. Each direction of a one-way relation is its own yes/no. The three-way choice for one-way relations is withdrawn. The default cut stays 0.5, and requests are split under the size budget. Across seven sets H found 89 of 89 true links with 11 false ones, at the fewest tokens on every set. Matching one list against another is `choose` per record, not relate. The edge shape ruled below and in ticket 0081 does not change. This section overrides the method in the section that follows.

## Ruled 2026-09-23: yes/no per both-ways relation, a three-way choice per one-way relation

Ian ruled on 2026-09-23 and overturned the pick-one method of 2026-09-21. A relation that reads the same both ways, such as `same_as`, gets one yes/no question per pair. A relation with a direction gets one three-way choice per pair: source to target, target to source, or neither. A pair can hold several relations, and each direction keeps its own probability. The evidence is `experiments/225-recognize-harvest-package/relate/measurements/VERDICTS.md`, arm (a). Twelve pairs held two true relations each. Pick-one found 11 of 24 at 2,449 tokens, and yes/no found 22 of 24 at 1,714 tokens. With several rules, yes/no asks one question per rule and stays cheaper up to about three rules. This section overrides the method sentence above.

Two more answers to the build team the same day. Recognition policy knobs stay out of this round, and only `--threshold` and `--relation-threshold` ship. Splitting is in this round and required. A request over the vendor's size budget of about 150,000 characters is split into requests that fit. Without the split, relate fails above about 25 records, and recognize fails on a long paragraph. Splitting is a correctness fix and needs no paid measurement. Dry-run sizes and replay prove it. Packing to save money stays out of this round. A request under the budget keeps its historical bytes, and no cost claim changes.

## What Ian can overturn

All of it. He has ruled: `relate` is built with `recognize`, and it asks by yes/no for a both-ways relation and by a three-way choice for a one-way relation (2026-09-23).
