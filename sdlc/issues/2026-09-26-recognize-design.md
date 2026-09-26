# recognize: the design

Status: Held by Ian. Do not start until Ian sends it.

Filed 2026-09-26. This design replaces two issues, now in `closed/`: `2026-09-26-recognize-loses-possessive-and-side-by-side-names.md` and `2026-09-26-recognize-relations-report-edges-the-text-does-not-state.md`. Its evidence comes from workspace experiments 265, 267, 270 and 271. Batching of many texts follows `2026-09-26-batching-design.md`, and the two documents share one set of batch rules.

## What recognize is for

A user hands `recognize` a text and a few kinds. It gives back every name the text contains, with its exact place in the text, its kind, and a strength. With relation rules, it also says how the names relate, as the text states it. Whether the text is true does not matter. The offsets let a program cut the name out of the original text with no guessing.

Three things make it delightful. The first call needs no options. Common English forms come back right: possessives, quoted titles, and two names side by side. A paragraph answers in about half a second for about a tenth of a cent.

## What the user sees

### Command line

The simplest call:

```sh
$ echo "George Harrison's song Something appears on Abbey Road." | thinkthen recognize person work
{"entities":[{"name":"George Harrison","kind":"person","start":0,"end":15,"strength":0.97},{"name":"Something","kind":"work","start":23,"end":32,"strength":0.98},{"name":"Abbey Road","kind":"work","start":44,"end":54,"strength":0.94}]}
```

Today the same call returns `George` alone. Strengths in every example here come from recorded runs or are illustrative. They vary a little between runs.

With no kinds, the kinds are `person`, `organization` and `place`, as today.

Two names side by side:

```sh
$ echo "On Revolver Paul McCartney sang Eleanor Rigby." | thinkthen recognize person work
{"entities":[{"name":"Revolver","kind":"work","start":3,"end":11,"strength":0.95},{"name":"Paul McCartney","kind":"person","start":12,"end":26,"strength":0.99},{"name":"Eleanor Rigby","kind":"work","start":32,"end":45,"strength":0.97}]}
```

Today this returns one person named `Revolver Paul McCartney`.

A relation the text states:

```sh
$ echo "John Lennon wrote All You Need Is Love." | thinkthen recognize person song --relation wrote=person:song --relation sang=person:song
{"entities":[{"name":"John Lennon","kind":"person","start":0,"end":11,"strength":0.99},{"name":"All You Need Is Love","kind":"song","start":18,"end":38,"strength":0.97}],"relations":[{"relation":"wrote","source":{"name":"John Lennon","kind":"person","start":0,"end":11,"strength":0.99},"target":{"name":"All You Need Is Love","kind":"song","start":18,"end":38,"strength":0.97},"probability":0.99}]}
```

Today this also prints `sang` at 0.74. The text never says Lennon sang it.

Many short texts, one per line:

```sh
$ thinkthen recognize person work --lines < sentences.txt
{"input":"George Harrison's song Something appears on Abbey Road.","value":{"entities":[…]}}
{"input":"On Revolver Paul McCartney sang Eleanor Rigby.","value":{"entities":[…]}}
```

Once ticket R7 lands, the tool sends up to 10 texts in one request by the batching design's rules. No flag is needed. `--batch 1` sends one text a request, as today.

Every call takes `--details` for probabilities and run facts, as today.

### Python

One line returns the names. Every result carries its run facts.

```python
import thinkthen as tt

names = tt.recognize("George Harrison's song Something appears on Abbey Road.", kinds=["person", "work"])
names[0]
# Entity(name='George Harrison', kind='person', start=0, end=15, strength=0.97)
names.facts
# Facts(records=1, requests_sent=1, cached_requests=0, input_tokens=3380, output_tokens=790, seconds=0.21, model='jev-1.13.0')
```

`names` is a list of `Entity`. Code that treats it as a list keeps working. `names.facts` holds the run facts that `2026-09-26-every-surface-should-give-back-run-facts.md` asks for, with the names the batching design fixes. The numbers above are illustrative. Experiment 270 billed 3,045 input tokens for this sentence with five kinds under today's splitter. The new rules add one word, about 345 tokens.

A column of texts batches the same way the command does:

```python
frame = tt.recognize(df, kinds=["person", "work"], on="text")
```

TypeScript, Ruby and R take the same arguments under their own spelling and return the same shape.

### DuckDB

```sql
SELECT id, unnest(thinkthen_recognize(body, ['person', 'work'])) AS name FROM notes;
```

Each name is a row of `(name, kind, start, end, strength)`, as today. DuckDB hands the extension up to 2,048 rows at once, and the extension batches them as the batching design fixes.

### Walkthrough: a first user and one paragraph

A user pastes a 60-word paragraph about the Beatles into `thinkthen recognize person work place`. The tool splits the text into about 64 words and asks two questions about each word in one request. The request carries about 22,000 input tokens. Experiment 270 billed 345 input tokens a word. At the recorded input price of $0.042 a million tokens, the paragraph costs about $0.0009. Output tokens are free under the vendor's price list. When a run of name words changes kind, a second request asks whether it is one name or several. Experiment 271 answered a request of 1,224 questions in 0.63 s, so both round trips together should take about half a second. The first acceptance run measures this time.

The user sees one line of names. `--details` adds every word's probabilities and the run facts.

### Friction list

| What a user must know today | What this design does |
| --- | --- |
| That `'s`, quote marks and brackets break names | Designed away. The default word rules handle them |
| That two touching names merge | Designed away when their kinds differ. `confirm` separates them. Touching names of one kind stay a known gap |
| That a relation may come from what Jev knows of the world | Designed away. A recognize relation means the text states it |
| That a text over about 180 words should pass the backend's token limit | Designed away. The tool splits long texts under the built-in request ceiling. One request holds about 82 words |
| That `McDonald's` loses its `'s` | Kept. The default follows MUC-7, ACE and CoNLL. One setting, `--word-keep "McDonald's"`, fixes it |
| That `London-based` gives no `London` | Kept. Hyphens stay joined, so `Hewlett-Packard` stays whole. `--word-infix -` changes it |
| That the kinds need words the model knows | Kept. A kind can take a description with `--kind KIND=DESCRIPTION`, as today |
| That `relate` after `recognize` answers from what Jev knows of the world | Kept and documented. `relate` has no text. A user who wants what the text states uses `recognize --relation` |

## The design

### 1. Ask Jev about each word

Jev takes one evidence text and many questions, and answers each question with probabilities. It cannot return a span of text. So `recognize` keeps its present method. The whole text is the evidence. Each word gets a yes/no question: is this word part of a name? With two or more kinds, each word also gets a pick-one question: which kind? Each question shows a five-word window with the word marked, as today.

Rejected:
- Asking Jev to return spans. Jev answers only fixed questions with probabilities.
- Asking about every candidate span. The number of spans grows with the square of the word count.
- A three-way begin, inside or outside question per word. Experiment 270 measured it. It separated no pair that `confirm` missed, and it lost titles that start with `The`.

### 2. Words

Five settings decide how text becomes words. Entries are literal strings. There are no regular expressions and no callbacks.

| Setting | Default | Meaning |
| --- | --- | --- |
| `prefixes` | `"` `“` `‘` `'` `(` `[` `{` | Peeled from a word's start, one at a time |
| `suffixes` | `.` `!` `?` `,` `:` `;` `"` `”` `’` `'` `)` `]` `}` `'s` `’s` `'S` `’S` | Peeled from a word's end, longest first, one at a time |
| `infixes` | `–` `—` | Split out of a word between letters |
| `keep` | empty | Whole words never split, such as `McDonald's` or `Sgt.` |
| `trim` | Opening marks at a name's start. Closing marks, `'s`, `’s`, `'S`, `’S`, and a bare `'` or `’` at a name's end | Removed from a name's edges after the name is formed |

Fixed rules:
- A final period stays on a word that holds another period. `U.S.`, `J.R.R.` and `D.C.` stay whole.
- The model answers for every word, marks included. `Help!` keeps its `!` when the model puts the `!` in the name.
- `trim` is the only rule that overrides the model.
- Offsets always point into the original text. Inner marks come back as written: `Rock 'n' Roll Music`, `Washington, D.C.`

Each list adds to its default. `defaults: false` starts every list empty. The defaults follow MUC-7, ACE and CoNLL. A caller gets Universal NER's convention by removing the possessives from `trim`. Each list holds at most 100 entries. Each entry is nonblank, holds no white space, and is at most 32 characters.

Experiment 267 showed today's splitter can reach 141 of the key's 168 names at all. The default rules reach 166.

### 3. Names from words

One setting, `boundary`, decides how word answers become names.

| Value | Rule |
| --- | --- |
| `confirm` (default) | A run of name words is one name. When the word kinds inside a run change, the tool asks one pick-one question: is the run one name, or separate names split at each kind change? |
| `run` | Today's rule. A run of name words is one name |

The measured question, asked with the whole text as evidence:

```text
In this text, is "Revolver Paul McCartney" one name, or is it separate names that stand side by side with no word between them?
ONE:   One name: "Revolver Paul McCartney".
SPLIT: Separate names: "Revolver", "Paul McCartney".
```

On `SPLIT`, each part is trimmed and takes its kind by the same vote a run uses. On `ONE`, the run stays whole and keeps its vote. All `confirm` questions for one text ride in one request after the word answers return. Experiment 270 sent 0.09 extra requests a case.

Rejected: splitting at every kind change with no question. It would cut `Abbey Road`, whose words Jev put in place and album.

### 4. Kinds and strength

Kinds, the kind vote and `strength` stay as `specification/recognize.md` defines them. Strength is the lowest detection probability in the name times the mean probability of the winning kind. The cut stays at 0.5. Experiment 270 tuned the cut on half the key and tested it on the other half. It moved recall and precision by at most 2.5 points, in both directions.

### 5. Relations mean "the text states it"

Ian ruled that `recognize` finds what the text says. A recognize relation therefore means the text states it. Both relation questions change their wording to ask about the text:

| Method | Today | New |
| --- | --- | --- |
| Pick-one, different kinds | `Which listed song fills the blank: Item 1 (person "Ringo Starr") wrote ___? Choose none if no listed song does.` | `By what the text states, which listed song fills the blank: Item 1 (person "Ringo Starr") wrote ___? Choose none if the text states it of no listed song.` |
| Yes/no, same kind | `Does the relation hold from i1 to i2?` | `Does the text state that the relation holds from i1 to i2?` |

The request state already carries the text as `evidence`. Standalone `relate` sends no text and keeps today's wording. Its answers come from what Jev knows. `specification/relate.md` says so in one sentence.

Experiment 265 asked Jev directly, "Does the text itself state that …?" Stated edges scored 0.97 to 0.99, and the unstated edges in its table scored 0.02 to 0.04. The new pick-one wording has not been measured. Ticket R5 measures it before it lands.

### 6. Long texts

Each word costs about 345 input tokens and about 1,160 request bytes in two questions, by experiment 270's runs and dry runs. Jev refuses a request over 65,536 input tokens. At the built-in address only a relation plan splits under the built-in ceiling. A text over about 180 words should therefore pass Jev's limit and fail today. The word questions will split under the same built-in ceiling of 96,000 request bytes that `specification/backends.md` sets for relation plans. Every split request carries the whole text as evidence, as today. A name may cross a request boundary, and offsets never change. One text's split requests run at once under the engine's throttle. Today `Engine::ask_chunks` sends them one after another.

### 7. Many short texts

Under `--lines`, `--jsonl`, `--csv` or `--tsv`, `recognize` batches texts by the rules in `2026-09-26-batching-design.md`. A batch holds up to 10 texts and stays under the 96,000-byte ceiling. At about 1,160 bytes a word, ten sentences of eight words come near the ceiling, so most batches close on bytes at 8 to 10 texts. The evidence is `{"records":[T1,…,TN]}`, the same object the batching design uses. Each word question begins `In record K of the list.` and then gives today's window. A batch of one text sends today's exact request. The `confirm` questions of a whole batch ride in one request after the word answers return.

Batching short texts saves little money. Experiment 270 billed about 2,665 input tokens a sentence, and the fixed part of a request is about 250, by experiment 271. It saves requests and time. At 10 texts a request, the vendor's 1,200 requests a minute stop limiting a run. Batching changes answers. Ticket R7 measures how much on the 100-sentence key before batching turns on for `recognize`.

Relations stay per text. Each text's relation state carries its own names, so two texts cannot share a relation request.

## Defaults and what a caller can change

| Setting | Default | Command | Question file | Library | SQL |
| --- | --- | --- | --- | --- | --- |
| Kinds | `person organization place` | `KIND…` or `--kind KIND=DESCRIPTION` | `recognize.kinds` | `kinds=` | second argument |
| Word rules | Section 2 | `--word-prefix`, `--word-suffix`, `--word-infix`, `--word-keep`, `--name-trim`, each repeatable; `--no-default-words` | `recognize.words` with the five lists and `defaults` | `words=` | `words` key in the JSON recognize section |
| Boundary | `confirm` | `--boundary confirm\|run` | `recognize.boundary` | `boundary=` | `boundary` key |
| Name cut | `0.5` | `--threshold` | `threshold` | `threshold=` | `threshold` key |
| Relation rules | none | `--relation NAME=SOURCE:TARGET` | `recognize.relations` | `relations=` | `thinkthen_relations(text, file)` |
| Relation cut | `0.5` | `--relation-threshold` | `relation_threshold` | `relation_threshold=` | `relation_threshold` key |
| Texts a request | as the batching design fixes | `--batch N` or `THINKTHEN_BATCH` | `batch` | `batch=` | `SET thinkthen_batch` |

Word rules and boundary enter the question digest. Different rules never share a cache entry. `--batch` does not enter the question digest, and it changes request digests.

## Output and run facts

Bare output keeps today's shape: `{"entities":[…]}`, with `relations` present only when rules were given. Record modes print `{"input":…,"value":…}` in input order.

`--details` keeps `thinkthen.result/1` and adds three things:
- `question.words` and `question.boundary` hold the effective rules.
- `answer.tokens` holds each word's detection and kind probabilities, as today. `answer.confirm` lists each `confirm` question with its run, its two options and their probabilities.
- A batched text's `meta` carries its share of the batch's usage and `meta.batch`, as the batching design fixes.

`--dry-run` prints `thinkthen.recognize-plan/1` for the first text, or for the first batch when the input is a stream. `words` counts words under the effective rules. `confirm_questions_upper_bound` counts one for each run that could change kind, because kinds do not exist yet.

The library's `.facts` and the SQL `thinkthen_usage()` totals follow the batching design.

## Edge cases

| Case | Expected behaviour |
| --- | --- |
| `George Harrison's song` | `George Harrison`. The `'s` is split off and trimmed |
| `McDonald's` with default rules | `McDonald`. With `--word-keep "McDonald's"`, `McDonald's` |
| `"Help!"` in quotes | `Help!`. The quotes are trimmed. The `!` stays when the model puts it in the name |
| `Octopus's Garden` | Often breaks into `Octopus` and `Garden`, 5 of 6 tries in experiment 270. Known gap |
| `U.S.`, `D.C.`, `J.R.R. Tolkien` | Stay whole |
| `Revolver Paul McCartney`, kinds differ | Two names after `confirm` says `SPLIT` |
| `Paul John`, one kind | One name. Known gap |
| `Abbey Road`, words disagree on kind | `confirm` asks. On `ONE` it stays whole. Its strength can still fall under the cut. Known gap |
| `London-based` | No `London`. `--word-infix -` finds it |
| `Paris–Rome` | Two words by default, because `–` is an infix |
| Empty text | `{"entities":[]}` and no request |
| Text with no names | `{"entities":[]}` |
| Text over 96,000 request bytes of questions | Split into several requests, each with the whole text. Offsets unchanged |
| Text whose evidence alone passes Jev's evidence limit | The backend refuses it at exit 4 with today's `max_tokens_exceeded` message |
| A `confirm` or relation question that fails | That text fails, as a failed word question does today. No partial names print for it |
| `--word-keep` entry with a space inside | Usage error at exit 2 |
| 101 entries in one list | Usage error at exit 2 |
| Relation the text states but that is false (`Yoko Ono wrote Something.`) | The edge prints |
| Relation that is true but unstated | No edge |
| `--batch 1` under `--lines` | One text a request, byte for byte today's request |
| A batch where several texts need `confirm` | One `confirm` request for the batch after its word answers |

## Acceptance tests

The key is workspace experiment 267's `cases.jsonl`: 100 hand-written sentences with 168 names in 14 categories. Ticket R1 copies it to `specification/fixtures/recognize/names-100.jsonl`. A name counts only on exact character offsets and label.

1. **Reach, no model.** Under the default word rules, 166 of the 168 key names start and end on word edges. The exceptions are `London` in `London-based` and `Microsoft` in `anti-Microsoft`. With `-` added to `infixes`, all 168 are reachable. Under today's splitter, 141 are. A unit test pins all three counts.
2. **Trim, no model.** Mark every word of a key name plus its next possessive or quote word as a name word. Assembly returns the key name exactly. A table test covers every possessive and quotes-brackets case except n011 and n012.
3. **Caller rules, no model.** `keep: ["McDonald's"]` makes n011 return `McDonald's`. Removing `'s` from `trim` makes n001 return `George Harrison's`.
4. **Boundary, recorded.** Ticket R3 records one live run of the key under `confirm`, about $0.012. Replay it under `run` and under `confirm`. `confirm` separates n017, n018, n019 and n021. It loses no exact name that `run` finds.
5. **Regression, recorded.** Experiment 265's 30 sentences lose no exact name found today, except the known losses: `McDonald's`, `Sainsbury's`, `Octopus's Garden`, and strength-cut misses on `Sgt. Pepper's Lonely Hearts Club Band` and `St. Mary's Church`.
6. **Target, one recorded live run.** Use the key's kinds, `person place organisation work thing`, and every other default. Exact recall and precision on the key each reach at least 85%. Recall pooled over the possessive, quotes-brackets and side-by-side categories reaches at least 80%. Experiment 270 measured 87.5%, 88.6% and 83.3% pooled. Per category it measured 82.7%, 91.7% and 76.5%. Recording the 100 sentences costs about $0.012.
7. **Relations, one recorded live run.** On experiment 265's 30 sentences with `sang=person:song` and `wrote=person:song`, none of the five unstated edges in the closed relations issue passes 0.5. At least 21 of the 27 stated edges pass, today's count.
8. **Long text, no model.** `--dry-run` on a 1,000-word text shows several requests, each at most 96,000 bytes, each carrying the whole text. A replay through a fake backend returns the same names as one unsplit request.
9. **Batch, recorded.** Run the key with `--jsonl --field /text` and the key's kinds, three times at `--batch 1` and three times at the default. The mean recall and mean precision of the batched runs each fall at most 2 points below the unbatched means. The six runs cost about $0.07. `--batch 1` sends bytes identical to today's requests. The dry run of the first batch shows `closed` as `bytes` or `full`.

## Tickets

In order. "Needs ADR" marks a ticket that changes a Settled specification page.

| # | Outcome | Scope | Proof | Depends on | ADR |
| --- | --- | --- | --- | --- | --- |
| R0 | Ian's rulings recorded | One ADR for word rules, `confirm` as default, stated relations, the split of long texts, and the recognize batch shape. It lands after batching B0 and builds on it. It retires the 2026-09-23 maximal-run baseline as default and keeps it as `boundary: run`. It lifts the ban on recognition policy keys in `question-file.md` and ticket 0080 | ADR accepted; `recognize.md`, `question-file.md` and `backends.md` updated | none | This is the ADR |
| R1 | The keys live in the repo with a scorer | The 100-case fixture; experiment 265's 30 sentences and its stated-edge key as `specification/fixtures/recognize/relations-30.jsonl`; a test helper that scores exact offsets and labels | Test 1's count of 141 under today's splitter | none | no |
| R2 | Default word rules on the command and question file | Splitter, trim, five settings, limits, digest, `--details` and `--dry-run` fields | Tests 1, 2, 3 and 5 | R0, R1 | covered by R0 |
| R3 | `confirm` boundary | Confirm question, one request per text, `--boundary`, `answer.confirm`; one authorized live recording of the key | Tests 4 and 6 | R2 | covered by R0 |
| R4 | Long texts split | Built-in ceiling reaches word questions. One text's split requests run at once under the throttle | Test 8 | R0; batching B11 for the concurrent send | covered by R0 |
| R5 | Relations mean the text states them | Two question wordings, `recognize.md` and one sentence in `relate.md` | Test 7. If it fails, the ticket stops and reports the scores to Ian | R0 | covered by R0 |
| R6 | Word rules and boundary on every library and SQL surface | Rust builder, Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL and SQLite; the conformance cases; `.facts` on recognize results | One shared conformance case per setting passes on every surface | R2, R3; batching B12 for `.facts` | no |
| R7 | Many short texts batch | Batch evidence, question prefix and batched `confirm` for recognize on the command, libraries and SQL. One authorized live measurement. If test 9 fails, recognize stays at one text a request and the ticket reports to Ian | Test 9 | R3; batching B4, B5 and B13 | covered by R0 |
| R8 | The manual shows the dials | One recognize page: word rules, boundary, relation meaning, a symptom-to-setting table with measured numbers | Docs review | R3, R5 | no |

## Known gaps

- Touching names of one kind: `Paul John`, `Portland Oregon`.
- `Octopus's Garden` breaks in two in most tries.
- The strength formula drops some whole names on kind doubt: `Boeing 747` at 0.32 to 0.39, `Sgt. Pepper's Lonely Hearts Club Band` and `St. Mary's Church` in 5 of 9 tries. It stays until a measured change is proposed.

## Open items Ian can overturn

1. Every default: the word lists, `confirm` over `run`, the 0.5 cuts, and hyphens joined.
2. The new relation wordings, and the bar of test 7.
3. The targets and margins of tests 6 and 9.
4. Leaving out `begin` and the split with no question.
5. Batching texts for `recognize` by default once test 9 passes.
6. The shared evidence name `records` for texts.
