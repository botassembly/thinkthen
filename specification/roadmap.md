# Roadmap

Not a contract. This page lists what version one leaves out and what would bring each item in.

The rule for entry is ADR 0005: a feature enters when a demo cannot be written without it. Twelve demos were written before the code, and `demos/FINDINGS.md` records what they reached for. Nothing in the two tables below was reached for. The last section holds what ADR 0010 took out of version one after demos had already been written for it.

`match` left this page. A file of pairs goes through `annotate`, which puts both entities in one record and asks several questions of it.

## Held verbs

| Verb | Why it is held | What would bring it in |
| --- | --- | --- |
| `reduce` | It runs an adaptive search. It would ask a question, cut the material, and ask again, so the tool would be steering rather than judging | A demand for shrinking that a fixed number of independent judgments cannot serve |
| `patch` | It writes files. The tool judges and never acts | Nothing in this tool. A separate program applies an edit that this tool located |
| `state` | It keeps a durable store between runs. The tool holds no state | A demo that needs a store, and a decision about where the store lives |
| `assign` | It never calls a model. It solves a one-to-one assignment over judged pairs | A solver of its own, fed by `decide --jsonl --details` |
| `cover` | It never calls a model. It picks a covering set over judged pairs | The same solver |
| `select` | It never calls a model. It picks a subset under constraints | The same solver |

## Held options and features

| Item | Why it is held | What would bring it in |
| --- | --- | --- |
| `--from FILE` for `choose` options | On one document the shell already reads a list into the arguments with command substitution. In record mode `--options POINTER` reads the list from the record | A demo whose candidate list is neither in the record nor available to the shell |
| `--invert` on `filter` | Every demo that wanted the other side wrote the question the other way round | A demo where the question cannot be inverted in words |
| `--output FILE`, publishing on success | It would write a file, and it would add a second success path beside the exit code | A demo that must not leave a half-written file behind on a failure |
| `--context FILE` | One verb carrying extra evidence alone would split the grammar. A user concatenates the context into standard input | A demo where the context has to stay separate from the evidence in the request |
| `--on-error continue` | It needs an error row shape, a failure count on standard error, and an exit code of its own. Stopping at the first failure needs none of those | A demo over a large file where one bad record must not end the run |
| A request cap | Demo 05 wanted a budget and found it the wrong shape, because a per-file loop spends across processes rather than within a run | A budget that holds across processes. That is a different tool |
| CSV and TSV reading | Judgments are fields on a record now, so a CSV framing buys nothing. `jq -r '@csv'` writes the output | A demo whose input is CSV that cannot be converted upstream |
| A `required` mark in the `annotate` file | The file holds questions and nothing else. A required mark is policy | A demo where a missing answer must fail the record |
| Nesting in `annotate` output | Flat top-level fields keep a chain of judgments flat. Demo 07 found nesting made the next `jq` read `.input.input.sku` | A demo whose answers collide with record fields that cannot be renamed |
| History across many runs | A comparison of two saved runs keeps nothing. A trend over many runs needs a store, and the tool holds no state | A store that lives outside this tool and reads the saved runs |
| A threshold on `score` | `jq -e '. >= 2'` after the command cuts on the number in one line, and the help shows it | A demo where the cut has to travel inside a saved question file |
| A flag that repeats a run for trials | A shell loop does it, and a recipe averages within a case before it scores | A demo where the trials have to share one recording folder in one run |
| Packing many records into one request | Each record is sent once either way, so packing saves no tokens. It saves round trips, and those already run in parallel. It costs isolation, and accuracy falls as the evidence fills with unrelated content | Nothing measured so far |
| A two-pass `find` beyond 255 units | One request holds 255 units. A second pass over the winners would need a merge rule and a second measurement | A job whose candidate set cannot be cut to 255 upstream |
| `config set` | An editor changes a JSON file. A writer would be the first thing in this tool that writes a file the user did not name | A demand strong enough to change that rule |
| A subprocess adapter | It is the escape hatch for a vendor whose shape fits neither built-in adapter. Dynamic plugin libraries stay refused | A vendor worth supporting that serves neither `systemone` nor chat completions |
| `--none` on `choose` | The old `which` added an option meaning that no other option fits. A user adds `other` to the list and gets the same answer | A demo where the list is generated and cannot take an extra label |

## Held by ADR 0010

Ian read `sdlc/planning/open-concerns.md` on 2026-09-19 and took these out of version one. Each one had a specification page or a demo behind it, and the git history keeps both.

### `segment`

`segment` cut one document into segments at the boundaries a yes/no question found. The whole document rode in one request, every unit carried an id, and one question per gap rode with it.

Ian held it for two reasons. It is the least general verb of the nine, and it is the first one to cut when the surface has to shrink. The whole-document form also refuses a document larger than one request, and the long documents are the ones that most need cutting.

A framing that reads a long document in parts and joins the answers would bring it in, measured against the vendor's cap on questions in one request. A job that `filter` over `--lines` cannot do would bring it in sooner.

The earlier `specification/segment.md` and demo 11 stay in the git history.

### `report`

`report` read a file of `--details` rows, called no model, and printed the run's counts, accuracy against truth labels, a sweep of cuts, a calibration table, and a comparison of two runs by case id.

Ian set it aside and put `jq` recipes first. He named five outcomes:

1. No `report` at all.
2. The full `report`.
3. A partial `report`: the summary, `--truth`, `--threshold`, and the sweep.
4. The recipes alone.
5. A general way to carry recipes inside the tool.

One test decides between them. The recipes are written and tried on real rows in the recipes slice of `sdlc/planning/plan.md`. A command comes back only if the recipes prove too clumsy on those rows.

The fifth outcome carries a rule of its own. It enters only when several recipes exist that people would otherwise copy and paste. One recipe does not earn it.

An idea for that fifth outcome: the cheapest form is a command that prints a named recipe on standard output for `jq -f`, and it adds no dependency.

The earlier `specification/report.md` holds what the command knew about accuracy at coverage and about the fixed JSON shape of a report. It stays in the git history.
