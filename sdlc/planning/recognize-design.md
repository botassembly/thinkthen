# `recognize`: the design for the command and the nine surfaces

Status: Proposed by the marketing side on 2026-09-21, which holds the product and library shape job by Ian's word. Ian ruled the same day that `recognize` is the ninth function. The build team reviews this page before any ticket. It authorizes no build and no paid run.

The method is settled elsewhere and this page does not reopen it: `experiments/RECOGNIZE-PRODUCT-SPEC.md` holds the three steps, the confidence formula, and the measured limits. `experiments/222-recognize-demo/` holds the first output in the final shape. This page rules what a user types and what comes back, on every surface.

## The one line

**Find every name in a text and say what kind it is.**

Relations are the second answer: how two names relate. Relations are beta, and every page that mentions them says so in the same sentence.

## The words

| Word | Means | Note |
| --- | --- | --- |
| name | One stretch of the text that names a thing | The object's field is `entities`, the word an engineer searches for. Prose says "name" |
| kind | What a name is: `person`, `organization`, `place`, or the user's own | The output carries the user's word. It never carries a code such as `PER` |
| relation | A named link from one name to another | Always has a direction |
| relation rule | One row that says which kinds a relation may join | |
| confidence | The number on a name or a relation | The one place the public copy says "confidence". It is computed from several probabilities and is no single probability. The manual says so once. `--details` carries the raw probabilities |

## The command

```
thinkthen recognize [OPTIONS] [KIND]...
thinkthen recognize [OPTIONS] @names.json
```

`recognize` is the one function with no question words. The kinds are the question. They sit where `tag` puts its labels, and they follow the same two forms:

```
thinkthen recognize person organization place < hire.txt
thinkthen recognize --kind person='A human being, by name.' --kind vessel='A named ship.' < log.txt
```

With no kind given, the kinds are `person`, `organization`, and `place`. A kind takes 1 to 20 entries, the same limit as a `tag` label.

| Option | Does |
| --- | --- |
| `--kind KIND=DESCRIPTION` | One kind and what it means. Once per kind |
| `--relation NAME=FROM:TO` | One relation rule. Once per rule. Turns relations on |
| `--threshold T` | The bar a name must reach. Default 0.5 |
| `--relation-threshold T` | The bar a relation must reach. Default 0.5 |
| `--details` | The full result, with each word's probabilities |
| `--input`, `--lines`, `--jsonl`, `--csv`, `--tsv`, `--field`, `--dry-run`, `--cache`, `--replay`, `--no-cache` | As on every function |

The word bar, the formula choice, and the boundary repairs from the specification are question-file keys only. A first user never needs them, and a command line with three thresholds teaches the wrong thing.

Exit codes: 0 when the run finished, including a text with no names in it. An empty `entities` list is an answer. There is no exit 1 and no exit 3, because nothing here is a yes, a no, or a "not sure". 2, 4, 5, and 70 mean what they mean everywhere.

## The relation rule

Ian ruled the shape on 2026-09-21: a relation has a direction, and each end is one kind or any kind.

```
--relation works_for=person:organization     one kind to one kind
--relation mentions=person:*                 one kind to any kind
--relation located_in=*:place                any kind to one kind
```

- The left of the colon is the kind the relation comes **from**. The right is the kind it goes **to**. `works_for=person:organization` reads "a person works for an organization".
- `*` means any kind. It is always written. A missing end is a usage error, exit 2.
- A rule is one-way. A relation that reads the same both ways, such as `married_to`, is written once with `"either": true` in the question file, and the tool asks it once per pair.
- `*:*` is legal. The pair count grows with the square of the names in a text, and `--dry-run` prints the pair count before anything is sent.
- The tool lists only the pairs a rule allows and never asks about any other pair. "No relation" is always one of the options and the user never writes it.
- A name and itself are never a pair.

In the question file:

```json
{
  "version": 1,
  "recognize": {
    "kinds": {
      "person": "A human being, by name.",
      "organization": "A company, agency, or institution.",
      "place": "A city, region, or country."
    },
    "relations": [
      { "name": "works_for", "source": "person", "target": "organization", "reads": "works for" },
      { "name": "located_in", "source": "*", "target": "place", "reads": "is located in" },
      { "name": "married_to", "source": "person", "target": "person", "either": true, "reads": "is married to" }
    ]
  },
  "threshold": 0.5,
  "relation_threshold": 0.5
}
```

`reads` is the plain verb the model sees between the two names. It defaults to the name with underscores turned to spaces. The file loads through the `@file` form and the same checks as every question file (ADR 0013). The demo's `relation_map.json` says `head`, `tail`, and `directed`. This page renames them to `from`, `to`, and `either`, because a user who has never read a paper on the subject can guess those.

## What comes back

One JSON object per text:

```json
{"entities": [
   {"id": 1, "text": "Maria Chen", "kind": "person", "start": 0, "end": 10, "confidence": 0.98},
   {"id": 2, "text": "Northwind Freight", "kind": "organization", "start": 18, "end": 35, "confidence": 1.0},
   {"id": 3, "text": "Chicago", "kind": "place", "start": 39, "end": 46, "confidence": 0.7154}],
 "relations": [
   {"name": "works_for", "source": 1, "target": 2, "probability": 1.0}]}
```

- `start` and `end` count characters in the text the user gave, so `text[start:end]` is the name, on every surface, in that surface's own string indexing. Each library converts once and documents it.
- `relations` is present only when a rule was given. It is `[]` when none was found.
- `source` and `target` are entity ids, because one text can name "Chicago" twice.
- The specification's `token_start` and `token_end` move to `--details`. A user never sees our word splitting unless they ask.
- The relation field is `name`, to match the rule that made it. The demo says `type`.
- Names below the bar are left out. `--threshold 0` returns every candidate with its number.

Under `--lines`, one object per line. Under `--jsonl`, `--csv`, and `--tsv`, each record comes back with the object attached under `recognize`, the way `annotate` attaches its answers. The record-loss finding in `sdlc/issues/2026-09-21-two-function-flows-lose-the-record-between-stages.md` does not repeat here.

## Size and cost, stated to the user

`recognize` is the one function that reads a text in pieces. It asks a question of every word, and a request holds a limited number of questions. The rule on the size slide, "the text crosses whole", does not hold for it. Three duties follow:

1. The manual says so in the first paragraph about size.
2. `--dry-run` prints the number of requests and, with a relation rule, the number of pairs.
3. Offsets always refer to the whole text the user gave, whatever the pieces were.

The measured cost is about 0.02 cents for one sentence with relations. The packing fix the specification names comes before any public price.

## The libraries

One rule for all six: `recognize` takes the text and the kinds, returns the object above as the host's own records, and takes a list or a column in one crossing like every other function. A saved question file goes where the kinds go.

| Surface | The call | Returns |
| --- | --- | --- |
| Python | `tt.recognize(text, kinds=["person", "organization"], relations={"works_for": ("person", "organization")})` | An object with `.entities` and `.relations`, each a list of small records. `tt.recognize(df, on="body")` takes a Polars DataFrame and returns a long one: one row per name, with the source row number |
| TypeScript | `await tt.recognize(text, { kinds, relations: { works_for: ["person", "organization"] }, signal })` | `{ entities, relations }`, typed |
| Ruby | `ThinkThen.recognize(text, kinds: %w[person organization], relations: { works_for: %i[person organization] })` | A struct with `entities` and `relations` |
| R | `tt_recognize(body, kinds)` | A list column of data frames. `tidyr::unnest()` makes one row per name |
| Rust | `tt.recognize(&Recognize::kinds(["person", "organization"]).relation("works_for", "person", "organization")?, &text)?` | `Recognized { entities, relations }`. `Kind::Any` is the `*` |
| C | `thinkthen_recognize(tt, spec_json, text, text_len, &out_json, &out_len)` and `thinkthen_string_free` | The JSON object as a string. The result has no fixed size, and C gets no struct tree to free by hand |

In every language `"*"` is the any-kind end, written as that one-character string, except Rust.

## The databases, and why `recognize` matters most there

A database user wants names as rows, because rows join.

| Engine | The function | Returns |
| --- | --- | --- |
| DuckDB | `thinkthen_recognize(body, ['person', 'organization'])` | A list of structs `(text, kind, start, end, confidence)`. `unnest()` makes rows |
| SQLite | `thinkthen_recognize(body, 'person,organization')` as a table-valued function | Rows with those five columns |
| PostgreSQL | `thinkthen_recognize(body, ARRAY['person', 'organization'])` as a set-returning function, used with `LATERAL` | The same five columns |
| All three | `thinkthen_relations(body, '@names.json')` | Rows `(name, source_text, source_kind, target_text, target_kind, confidence)`. Relations need the question file. Beta |

**The join rule, for the manual and the how-to.** A join by meaning, `JOIN ... ON thinkthen_decide(...)`, asks one question for every pair of rows. A thousand tickets against a thousand incidents is a million requests. The deck's join use case works because it joins two rows to two rows. The manual must give the three ways to keep a join affordable, in this order:

1. **Recognize once, then join on the names.** Reading a thousand tickets costs a thousand texts. The result is an ordinary table of names, and every join after that is an equality join the engine can index, with no request at all. This is the pattern to teach first.
2. **Narrow with an ordinary condition before the meaning condition.** A date range or a customer id in the `ON` clause cuts the pairs before any question is asked. The database pages must say whether each engine runs the cheap condition first, and must show how to force it with a subquery where it does not.
3. **Warm, then query.** `thinkthen_warm` pays once and every later query reads saved answers.

The planner never knows a function costs money. Every database page carries that sentence.

## What the deck shows

`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md` is the acceptance test for the libraries and extensions. It gains a section with the `recognize` call on every surface, and one new slide, "Names become rows", with pattern 1 in DuckDB.

## What is open for the build team

1. How a text is cut into pieces, and what happens to a name that crosses a cut.
2. The question count one request may hold, per backend. The second-backend experiment measured 16 on one open model.
3. Whether `thinkthen_relations` should exist, or whether one function should return both and let SQL pick. Two functions read better in SQL. One costs less when a user wants both.
4. Whether `recognize` belongs in `annotate`'s question set as a fifth question type. The suggestion is no for the first release.

## Ruled 2026-09-21: where the design page and the method page disagree, this page wins on what a user sees

An audit of `experiments/RECOGNIZE-PRODUCT-SPEC.md` against this page found five differences. The method page is older. `experiments/225-recognize-harvest-package/` already follows this page, and it is what a builder reads first.

| The method page says | The ruling |
| --- | --- |
| A `depth` dial: spans, labels, relations | No depth option. A relation rule turns relations on. Nothing else changes how deep the command goes. A caller who wants names without kinds gives one kind |
| Kinds `PER`, `ORG`, `LOC`, `MISC` by default | `person`, `organization`, `place`. No fourth catch-all kind by default. A user who wants one names it |
| `head`, `tail`, `type` | `source`, `target`, `name` |
| Word positions in the main object | Under `--details` only |
| A margin term in the confidence formula | The harvest package's formula, the least of the word probabilities times the mean of the kind probabilities, with connector words left out. It is what every recorded case used |

The internal rules stay internal and have no option: the connector list with its bar of 0.3, how overlapping names are settled, and the possessive rule. Their home is `experiments/225-recognize-harvest-package/rules/rules.md`, each with a test. The manual describes them in one paragraph, because they explain why "Nathan der Weise" comes back whole.

## One tool, ten functions

`--details` prints the standard result object, `thinkthen.result/1`, with this function's value in `value` and the same `question`, `answer`, and `meta` keys as the other functions. The question file grammar, the exit-code table, the cache, and the recording are the same. A caller that handles one function's result handles this one. A question that fails inside a request that otherwise succeeded is marked on that answer and counted in `meta`. It never prints `null`, because `null` means "not sure".

## Ruled 2026-09-21: a relation's ends are `source` and `target`, everywhere

`from` is a reserved word in Python and in SQL, so four of the nine surfaces could never say it. The library team chose `source` and `target` for every host. The product side extends that to the command's own JSON and to the question file, so no door converts anything and a user sees one pair of words on every surface. The rule on the command line is unchanged, `--relation NAME=FROM:TO`, because it names no field. This section overrides any older line on this page that says `from`, `to`, `head`, or `tail`. Ian can overturn it.

## What Ian can overturn

All of it. He has ruled: `recognize` ships, relations have a direction, and each end is a kind or any kind.
