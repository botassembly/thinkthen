# Roadmap

Not a contract. This page lists what version one leaves out and what would bring each item in.

The rule for entry is ADR 0005: a feature enters when a demo cannot be written without it. Twelve demos were written before the code, and `demos/FINDINGS.md` records what they reached for. Nothing below was reached for.

## Held verbs

| Verb | Why it is held | What would bring it in |
| --- | --- | --- |
| `match` | `match RELATION` is `decide --jsonl` over records that hold two things. Demo 11 looked at it and chose `segment`, because the hard half of a matching job is building the candidate pairs and this tool does not build them | A demo whose result has to carry both sides' identifiers, which a one-pointer verb cannot do |
| `reduce` | It runs an adaptive search. It would ask a question, cut the material, and ask again, so the tool would be steering rather than judging | A demand for shrinking that a fixed number of independent judgments cannot serve |
| `patch` | It writes files. The tool judges and never acts | Nothing in this tool. A separate program applies an edit that this tool located |
| `state` | It keeps a durable store between runs. The tool holds no state | A demo that needs a store, and a decision about where the store lives |
| `assign` | It never calls a model. It solves a one-to-one assignment over judged pairs | A solver of its own, fed by `decide --jsonl --details` |
| `cover` | It never calls a model. It picks a covering set over judged pairs | The same solver |
| `select` | It never calls a model. It picks a subset under constraints | The same solver |

## Held options and features

| Item | Why it is held | What would bring it in |
| --- | --- | --- |
| `--from FILE` for `choose` options | Every demo typed its options. A short fixed list is what the measurement supports | A demo whose candidate list is generated and too long to type |
| `--invert` on `filter` | Every demo that wanted the other side wrote the question the other way round | A demo where the question cannot be inverted in words |
| `--output FILE`, publishing on success | It would write a file, and it would add a second success path beside the exit code | A demo that must not leave a half-written file behind on a failure |
| `--context FILE` | One verb carrying extra evidence alone would split the grammar. A user concatenates the context into standard input | A demo where the context has to stay separate from the evidence in the request |
| `--on-error continue` | It needs an error row shape, a failure count on standard error, and an exit code of its own. Stopping at the first failure needs none of those | A demo over a large file where one bad record must not end the run |
| A request cap | Demo 05 wanted a budget and found it the wrong shape, because a per-file loop spends across processes rather than within a run | A budget that holds across processes, which is a different tool |
| CSV and TSV reading | Judgments are fields on a record now, so a CSV framing buys nothing. `jq -r '@csv'` writes the output | A demo whose input is CSV that cannot be converted upstream |
| Structured questions | A question is one string. A structured question would need a grammar, and no demo wanted one | A demo whose question cannot be written as a sentence |
| Per-record option lists for `choose` | The options are fixed for the run, so one run is one measurement | A demo where each record carries its own candidate list |
| A `required` mark in the `annotate` file | The file holds questions and nothing else. A required mark is policy | A demo where a missing answer must fail the record |
| Nesting in `annotate` output | Flat top-level fields keep a chain of judgments flat. Demo 07 found nesting made the next `jq` read `.input.input.sku` | A demo whose answers collide with record fields that cannot be renamed |
| Comparison across runs | Comparing two runs belongs to a history tool outside this one. `report` measures one run | A demo that has to diff two runs and cannot do it with `jq` |
| `config set` | An editor changes a JSON file. A writer would be the first thing in this tool that writes a file the user did not name | A demand strong enough to change that rule |
| A subprocess adapter | It is the escape hatch for a vendor whose shape fits neither built-in adapter. Dynamic plugin libraries stay refused | A vendor worth supporting that serves neither `systemone` nor chat completions |
| `--none` on `choose` | The old `which` added an option meaning that no other option fits. A user adds `other` to the list and gets the same answer | A demo where the list is generated and cannot take an extra label |
