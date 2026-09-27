# recognize: the design

Status: Sent by Ian to the main builder on 2026-09-26. Review findings and Ian's rulings of 2026-09-26 applied.

Filed 2026-09-26. This design replaces two issues, now in `closed/`: `2026-09-26-recognize-loses-possessive-and-side-by-side-names.md` and `2026-09-26-recognize-relations-report-edges-the-text-does-not-state.md`. Its evidence comes from workspace experiments 265, 267, 268, 270 and 271. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` copies every table this design cites. Batching of many texts follows `2026-09-26-batching-design.md`, and the two documents share one set of batch rules. Ian's rulings of 2026-09-26 are listed in that design. Several reach this one: long texts use a bounded window of evidence, the steps below are stated plainly, library results carry `facts` on every call, batching puts speed first, batching ticket C1 builds the settings page, and batching ticket S1 measures speed. Ian can overturn each.

## What recognize is for

A user hands `recognize` a text and a few kinds. It gives back every name the text contains, with its exact place in the text, its kind, and a strength. With relation rules, it also says how the names relate, as the text states it. Whether the text is true does not matter. The offsets let a program cut the name out of the original text with no guessing.

Three things make it delightful. The first call needs no options. Common English forms come back right: possessives, quoted titles, and two names side by side. A paragraph answers in under a second for under a tenth of a cent.

### The steps, stated plainly

- One request finds and labels the names. It asks two questions about each word: is this word part of a name, and which kind is it? A text longer than about 80 words sends one such request for each piece, by section 6.
- `confirm` adds a request only when touching names need separating.
- Relations add requests, at least one for each relation rule.

This is not a BILOU scheme, and it asks no begin, inside or outside question. Experiment 270 measured an in-or-out question plus `confirm` against a BILOU-style begin, inside or outside question. The in-or-out scheme did no worse overall: 87.5% recall and 88.6% precision, against 88.5% and 86.8%. Both separated 12 of 18 touching pairs.

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

`recognize` keeps `--lines` here. Its default input is one document, because a pasted paragraph is one text and should not split at each line. `--field` with no framing flag also reads one JSON document on `recognize`, as ADR 0007 rules for `decide`. Ticket 0137 gives `filter` and `rank` another rule: there a pointer with no flag means JSON Lines. Those two verbs cannot read one document, so JSON Lines is the only JSON framing left for them. `recognize` can, and one document stays its default. Once ticket R7 lands, the tool fills each request with as many texts as fit, by the batching design's rules. No flag is needed. `--batch 1` sends one text a request, as today.

Every call takes `--details` for probabilities, as today. `--facts` makes the command print its run facts, as the batching design fixes. `--facts` controls only what the command line prints.

### Python

One line returns the names. Every result carries its run facts on every call, with no setting and no second call.

```python
import thinkthen as tt

names = tt.recognize("George Harrison's song Something appears on Abbey Road.", kinds=["person", "work"])
names[0]
# Entity(name=<15 bytes withheld>, kind=<6 bytes withheld>, start=0, end=15, strength=0.97)
names.facts
# Facts(records=1, requests_sent=1, cache_answers=0, input_tokens=3350, output_tokens=790, seconds=0.21, model='jev-1.13.0')
```

`names` is a list of `Entity`. Code that treats it as a list keeps working. `names.facts` holds the run facts that `2026-09-26-every-surface-should-give-back-run-facts.md` asks for, with the names the batching design fixes. The numbers above are illustrative. Experiment 270 billed 3,045 input tokens for this sentence with five kinds under today's splitter. The new rules add one word, about 300 tokens.

A column of texts batches the same way the command does:

```python
frame = tt.recognize(df, kinds=["person", "work"], on="text")
```

TypeScript, Ruby and R take the same arguments under their own spelling and return the same shape.

### DuckDB

```sql
SELECT id, unnest(thinkthen_recognize(body, ['person', 'work'])) AS name FROM notes;
```

Each name is a row of `(name, kind, start, end, strength)`, as today. DuckDB hands the extension up to 2,048 rows at once, as `sdlc/planning/databases/README.md` records. The extension batches them as the batching design fixes.

### Walkthrough: a paragraph and a document

A user pastes a 60-word paragraph about the Beatles into `thinkthen recognize person work place`. The tool splits the text into about 64 words. One request finds and labels the names, with two questions about each word. The request carries about 19,600 input tokens: 300 for the request, about 144 for the text, and 300 a word. At the recorded input price of $0.042 a million tokens, the paragraph costs about $0.0008. Output tokens are free under the vendor's price list. When a run of name words changes kind, a `confirm` request asks whether it is one name or several. Experiment 271 answered a request of 21,871 input tokens in 0.52 s, and experiment 268 measured a small request near 140 ms. The paragraph should take about half a second, or about 0.7 s with `confirm`. Ticket S1 measures this time on a named build.

The user sees one line of names. `--details` adds every word's probabilities.

A user filing documents needs the cost of a full page up front. A long text splits into pieces. Each piece carries its own words plus up to 200 words of neighbouring text on each side, not the whole text. The cost therefore grows in step with the text's length. Ten times the text costs about ten times the tokens.

| Text | Words after the rules | Words a request | Word requests | Time, estimated | Input tokens, at most | Cost |
| --- | --- | --- | --- | --- | --- | --- |
| 60-word paragraph | about 64 | 64, the whole text | 1, plus 1 `confirm` request when needed | about 0.5 s, 0.7 s with `confirm` | about 19,600 | about $0.0008 |
| 1,000-word document | about 1,060 | 80 | 14, then the `confirm` requests | about 2 s, plus the `confirm` rounds | about 338,000 | about $0.014 |
| 10,000-word document | about 10,600 | 80 | 133, then the `confirm` requests | about 17 s, plus the `confirm` rounds | about 3,370,000 | about $0.14 |
| The hard cap, 600,000 bytes, about 100,000 words | about 106,000 | 80 | 1,325, then the `confirm` requests | about 2.8 minutes, plus the `confirm` rounds | about 33,700,000 | about $1.42 |

Section 13 of the evidence record shows the working. A request costs about 300 input tokens, plus its evidence at 0.40 tokens a byte, plus 300 tokens a word for the word's two questions. A least-squares fit over experiment 270's 100 cases gives those per-request and per-word figures. A word's two questions take about 1,150 request bytes. A piece of P words carries at most (P + 400) × 6 bytes of evidence. A request holds P words when 1,150 × P + (P + 400) × 6 + 74 ≤ 96,000, and that gives 80. So each request costs at most 300 + 1,152 tokens beside its words:

- 1,000 words: 1,060 ÷ 80 gives 14 requests. 14 × 1,452 + 1,060 × 300 = 338,328 tokens.
- 10,000 words: 10,600 ÷ 80 gives 133 requests. 133 × 1,452 + 10,600 × 300 = 3,373,116 tokens.
- The cap: 106,000 ÷ 80 gives 1,325 requests. 1,325 × 1,452 + 106,000 × 300 = 33,723,900 tokens.

The first and last pieces carry a window on one side only, so these figures are upper bounds. Experiment 270 used five bare kinds. The walkthrough uses three. That shortens each kind question by about 30 bytes, so these figures run slightly high. Time assumes 4 requests in flight, each round about half a second by experiment 271: 4 rounds for 14 requests, 34 for 133, and 332 for 1,325. The `confirm` requests carry a few questions and one window each, and the token and cost figures leave them out. Relations add at least one request for each concrete relation rule. Ticket R8 records one live run of a 1,000-word document and reports its requests, time and tokens.

### Friction list

| What a user must know today | What this design does |
| --- | --- |
| That `'s`, quote marks and brackets break names | Designed away. The default word rules handle them |
| That two touching names merge | Designed away when their kinds differ. `confirm` separates them. Touching names of one kind stay a known gap |
| That a relation may come from what Jev knows of the world | Designed away. A recognize relation means the text states it |
| That a text over about 200 words should pass the backend's token limit | Designed away up to the hard cap of 600,000 bytes. The tool splits long texts into pieces under the built-in request ceiling. A request holds about 80 words and a window of neighbouring text |
| That a very long text costs far more a word | Designed away. The window keeps the cost a word near 320 tokens at any length. A text over the hard cap is refused, and `--dry-run` shows the request count and bytes for any text |
| That a clue far from a name may be out of view | Kept and stated. A piece sees 200 words on each side by default. `--window N` widens it |
| That `McDonald's` loses its `'s` | Kept. The default follows MUC-7, ACE and CoNLL. One setting, `--word-keep "McDonald's"`, fixes it |
| That `London-based` gives no `London` | Kept. Hyphens stay joined, so `Hewlett-Packard` stays whole. `--word-infix -` changes it |
| That the kinds need words the model knows | Kept. A kind can take a description with `--kind KIND=DESCRIPTION`, as today |
| That `relate` after `recognize` answers from what Jev knows of the world | Kept and documented. `relate` has no text. A user who wants what the text states uses `recognize --relation` |

## The design

### 1. Ask Jev about each word

Jev takes one evidence text and many questions, and answers each question with probabilities. It cannot return a span of text. So `recognize` keeps its present method. One request finds and labels the names. Each word gets a yes/no question: is this word part of a name? With two or more kinds, each word also gets a pick-one question: which kind? Each question shows a five-word snippet with the word marked, as today. The evidence is the text, or for a long text the piece's window, by section 6.

Rejected:
- Asking Jev to return spans. Jev answers only fixed questions with probabilities.
- Asking about every candidate span. The number of spans grows with the square of the word count.
- A BILOU-style scheme: a three-way begin, inside or outside question per word. Experiment 270 measured it against in-or-out plus `confirm`. It scored 88.5% recall and 86.8% precision against 87.5% and 88.6%. It separated no pair that `confirm` missed, and it lost titles that start with `The`.

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

Each list adds to its default. `defaults: false` starts every list empty. The defaults follow MUC-7, ACE and CoNLL. A caller gets Universal NER's convention by removing the possessives from `trim` in a question file. Each list holds at most 100 entries. Each entry is nonblank, holds no white space, and is at most 32 characters.

`keep` and `infixes` reach every surface, because the two known gaps each need one: `McDonald's` needs `keep`, and `London-based` needs `infixes`. `prefixes`, `suffixes`, `trim` and `defaults` live only in the question file until a demo needs them elsewhere.

Experiment 267 showed today's splitter can reach 141 of the key's 168 names at all. The default rules reach 166.

### 3. Names from words

One setting, `boundary`, decides how word answers become names. It reaches every surface.

| Value | Rule |
| --- | --- |
| `confirm` (default) | A run of name words is one name. When the word kinds inside a run change, the tool asks one pick-one question: is the run one name, or separate names split at each kind change? |
| `run` | Today's rule. A run of name words is one name |

The measured question, asked with the text as evidence:

```text
In this text, is "Revolver Paul McCartney" one name, or is it separate names that stand side by side with no word between them?
ONE:   One name: "Revolver Paul McCartney".
SPLIT: Separate names: "Revolver", "Paul McCartney".
```

On `SPLIT`, each part is trimmed and takes its kind by the same vote a run uses. On `ONE`, the run stays whole and keeps its vote. `confirm` adds a request only when a run changes kind. All `confirm` questions for one piece ride in one request after the word answers return, with that piece's window as evidence. A run belongs to the piece that holds its first word. A short text is one piece. Experiment 270 sent 0.09 extra requests a case.

Rejected: splitting at every kind change with no question. It would cut `Abbey Road`, whose words Jev put in place and album.

### 4. Kinds and strength

Kinds, the kind vote and `strength` stay as `specification/recognize.md` defines them. Strength is the lowest detection probability in the name times the mean probability of the winning kind. The cut stays at 0.5. Experiment 270 tuned the cut on half the key and tested it on the other half. It moved recall and precision by at most 2.5 points, in both directions.

### 5. Relations mean "the text states it"

Ian ruled that `recognize` finds what the text says. A recognize relation therefore means the text states it. Both relation questions change their wording to ask about the text:

| Method | Today | New |
| --- | --- | --- |
| Pick-one, different kinds | `Which listed song fills the blank: Item 1 (person "Ringo Starr") wrote ___? Choose none if no listed song does.` | `By what the text states, which listed song fills the blank: Item 1 (person "Ringo Starr") wrote ___? Choose none if the text states it of no listed song.` |
| Yes/no, same kind | `Does the relation hold from i1 to i2?` | `Does the text state that the relation holds from i1 to i2?` |

The request state already carries the text as `evidence`. The relation step keeps the whole text as evidence, as today, because a relation pairs names from anywhere in the text. Its requests split by the ceiling, as `specification/backends.md` fixes. A text whose relation request passes Jev's evidence limit fails at exit 4 with today's message. Windowed relations are a known gap. Standalone `relate` sends no text and keeps today's wording. Its answers come from what Jev knows. `specification/relate.md` says so in one sentence.

Under today's wording, experiment 265's stated edges with correct names scored 0.89 to 1.0. Its five unstated edges, each true in the world, scored 0.55 to 0.78 in run one, and one reached 0.80 in runs two and three. Asked directly, "Does the text itself state that …?", six stated edges scored 0.84 to 0.99, and six unstated edges scored 0.02 to 0.05. Three stated edges that are false in the world scored 0.97 to 0.98. The new pick-one wording has not been measured. Ticket R5 measures it before it lands.

Rejected: raising the relation cut to 0.8 with today's wording. In run one it removed every unstated edge and lost no recall. In runs two and three an unstated edge scored exactly 0.80 and passed. The margin to the lowest stated edge, 0.89, is thin, and the cut would still measure what Jev knows of the world.

### 6. Long texts

Each word costs about 300 input tokens and about 1,150 request bytes in two questions, by experiment 270's runs and dry runs. Each request adds about 300 tokens and its evidence. Jev refuses a request over 65,536 input tokens. Today only a relation plan splits under the built-in ceiling, and every request carries the whole text. A text over about 200 words therefore fails today, and the cost of a split text would grow with the square of its length.

**Pieces and windows.** A text splits into pieces automatically. A piece is a run of consecutive words, filled until the next word would pass the ceiling of 96,000 request bytes that `specification/backends.md` sets for relation plans. A profile's `max_request_bytes` and `max_questions` replace it. Each piece's request carries its own words plus a window: up to `window` words before its first word and after its last word, clipped at the text's edges. The evidence is the original text from the first window word's start to the last window word's end, byte for byte. The default window is 200 words on each side. `--window N` sets it, from 0 to 5,000. Pieces go out under the throttle, up to `--jobs` at once.

Rules for a split text:
- The cost grows in step with the text's length. A request holds about 80 words at the default window, and each costs at most about 25,450 input tokens.
- A text shorter than a piece plus its windows is its own evidence, so a paragraph sends today's evidence.
- A name may cross a piece boundary. Names form only after every request for the text returns.
- A failed request fails the whole text. No partial names print for it.
- Offsets count Unicode scalar values into the original text, as today, and never change with the split.
- At an address with no ceiling and no profile, a text is one piece and carries the whole text, as today.

**Why 200 words.** No experiment measured a bounded window. Experiment 270's key sentences run 1 to 14 words and at most 74 bytes, so every measured case falls wholly inside any window of 14 or more words. Its results carry over unchanged. A window of 200 words on each side adds at most about 2,400 bytes, about 960 tokens, to a request of about 25,000. So a wide window costs little, and 200 words holds a few paragraphs on each side of a name. Section 13 of the evidence record shows the working. Ticket R8 measures a long document live at the default.

**The risk.** A window can lose a clue that sits far from a name. A name's kind may hang on a sentence 300 words earlier, such as a definition at the top of a report. The piece that holds the name then cannot see it. Test 11 proves the window reaches a clue inside it. A caller who needs more sets `--window N`. The default stands until a measurement shows a far clue matters on real text.

**The hard cap.** A text over 600,000 bytes exits 2 before any request, at every address. That is about 100,000 words, a full-length book. At the cap a text takes about 1,325 word requests, costs at most about $1.42, and takes about 2.8 minutes at 4 in flight. The cap has no option. The message names the text's size and the limit, and says to split the text into records with `--lines` or `--jsonl`. It echoes no text. `--dry-run` already prints the request count and every request's bytes, so no new warning line is needed.

**Concurrency.** Ian ruled that `recognize` accepts `--jobs` for one document, as `annotate` does, with a default of 4. One text's pieces then run at once under the engine's throttle. Today `Engine::ask_chunks` sends them one after another. Ticket J1 in the batching design makes them concurrent, and ticket R4b gives `recognize` the flag.

### 7. Many short texts

Under `--lines`, `--jsonl`, `--csv` or `--tsv`, `recognize` batches texts by the rules in `2026-09-26-batching-design.md`. A batch fills to the limit: it closes at the 96,000-byte ceiling, a profile limit, a content cut, `--batch N`, a pause or the end. At about 1,150 bytes a word, a request holds about 80 words, so about ten sentences of eight words. The evidence is `{"records":[T1,…,TN]}`, the same object the batching design uses. Each word question begins `In record K of the list.` and then gives today's window. A batch of one text sends today's exact request. The `confirm` questions of a whole batch ride in one request after the word answers return.

Batching short texts saves little money. Experiment 270 billed about 2,665 input tokens a sentence, and the fixed part of a request is about 250, by experiment 271. It saves requests and time. At 10 texts a request, the vendor's documented 1,200 requests a minute stop limiting a run, by `specification/records.md` `jobs`. Batching changes answers. Ian ruled that speed wins, so batching is on by default. Ticket R7 measures and reports the cost on the 100-sentence key, and the recognize page states it.

Relations stay per text. Each text's relation state carries its own names, so two texts cannot share a relation request.

## Defaults and what a caller can change

`specification/settings.md`, from batching ticket C1, is the full reference for every setting. This table lists the recognize settings only.

| Setting | Default | Command | Question file | Library | SQL |
| --- | --- | --- | --- | --- | --- |
| Kinds | `person organization place` | `KIND…` or `--kind KIND=DESCRIPTION` | `recognize.kinds` | `kinds=` | second argument |
| Kept words | none | `--word-keep`, repeatable | `recognize.words.keep` | `keep=` | `keep` key in the JSON recognize section |
| Infixes | `–` `—` | `--word-infix`, repeatable | `recognize.words.infixes` | `infixes=` | `infixes` key |
| Other word lists | Section 2 | none | `recognize.words` with `prefixes`, `suffixes`, `trim` and `defaults` | none | none |
| Boundary | `confirm` | `--boundary confirm\|run` | `recognize.boundary` | `boundary=` | `boundary` key |
| Name cut | `0.5` | `--threshold` | `threshold` | `threshold=` | `threshold` key |
| Relation rules | none | `--relation NAME=SOURCE:TARGET` | `recognize.relations` | `relations=` | `thinkthen_relations(text, file)` |
| Relation cut | `0.5` | `--relation-threshold` | `relation_threshold` | `relation_threshold=` | `relation_threshold` key |
| Window of neighbouring words on each side of a piece | 200, from 0 to 5,000 | `--window N` | `recognize.window` | `window=` | `window` key |
| Hard cap on one text | 600,000 bytes | none | none | none | none |
| Requests in flight for one text | 4, set by `--jobs`, 1 to 32 | `--jobs N` | none | the engine's throttle | the extension's throttle |
| Texts a request | as many as fit, as the batching design fixes | `--batch N` or `THINKTHEN_BATCH` | `batch` | `batch=` | `SET thinkthen_batch` |

Word rules, boundary and window enter the question digest. Different rules never share a cache entry. `--batch` does not enter the question digest, and it changes request digests.

The recognize canonical form in `specification/question-file.md` line 130 becomes `verb`, `kinds`, optional `relations`, `words`, `boundary`, `window`, `threshold`, `relation_threshold`, optional `profile`. `words` holds the five effective lists in the order `prefixes`, `suffixes`, `infixes`, `keep`, `trim`. Every recognize question digest therefore changes, including one whose file never names the new keys.

That breaks ticket 0135's `audit --write` on old runs. `--write` compares each line's `meta.question_sha256` with the question file's digest, and an old line carries the old digest. `audit` keeps grading old runs, and `--write` refuses them with its existing digest sentence. The old runs used today's splitter, so a cut tuned on them does not carry over to the new words. A user reruns under the new rules and writes from that run. The R0 ADR says so, and the audit page gains one sentence.

## Output and run facts

Bare output keeps today's shape: `{"entities":[…]}`, with `relations` present only when rules were given. Record modes print `{"input":…,"value":…}` in input order.

`--details` keeps `thinkthen.result/1` and adds three things:
- `question.words`, `question.boundary` and `question.window` hold the effective rules.
- `answer.tokens` holds each word's detection and kind probabilities, as today. `answer.confirm` lists each `confirm` question with its run, its two options and their probabilities.
- A batched text's `meta` carries its share of the batch's usage and `meta.batch`, as the batching design fixes.

`--facts` writes the batching design's `thinkthen.run/1` line to standard error. It controls only what the command line prints.

`--dry-run` prints `thinkthen.recognize-plan/1` for the first text, or for the first batch when the input is a stream. For a split text it gives each request's evidence range as start and end offsets. `words` counts words under the effective rules. `confirm` breaks the rule that `request_count` is exact, because its questions depend on the word answers. The new rule: `request_count` counts exactly the requests the tool forms before any answer. Those are the word requests. `confirm_questions_upper_bound` counts one for each run that could change kind, because kinds do not exist yet. The `confirm` requests come after and are not in `request_count`, as the relation requests already sit under their own bounds.

Library results carry `.facts` on every call, with no setting and no second call. The SQL `thinkthen_usage()` totals follow the batching design.

## Secrecy

No new failure line echoes a text, a list entry, a name or a key. That covers the word-rule refusals, a `confirm` failure, a split request's failure, the hard cap and the batch stop line. Each names the setting, the limit or the record number only.

`Debug` on the word rules prints each list's length and no entries. `Debug` on `Facts` prints counts and the model. `Debug` on a `confirm` entry prints its run's offsets and probabilities and no name or text.

The secrecy tests in R2, R3, R4 and R7 plant a secret in the text, a list entry and the key. Each drives its new failure paths and every new `Debug` line, and checks that the secret appears nowhere in standard output, standard error or the debug output.

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
| Text over 96,000 request bytes of questions | Split into pieces, each with its own window. Offsets unchanged |
| A name whose words fall in two split requests | One name. Names form after every request returns |
| One split request fails | The whole text fails. No partial names print |
| `confirm` questions over the ceiling | Split into several `confirm` requests, each with its piece's window |
| Text over 600,000 bytes, at any address | Exit 2 before any request. The message names the size and the limit and echoes no text |
| Text of 100,000 bytes at an address with no ceiling and no profile | One request with the whole text, as today |
| A clue 150 words before a name in a long text | Inside the default window. The name's request sees it |
| A clue 300 words before a name in a long text | Outside the default window. `--window 300` brings it in. Known risk |
| `--window 0` | Each piece carries only its own words |
| `--window 5001` | Usage error at exit 2 |
| Text whose relation request passes Jev's evidence limit | The backend refuses it at exit 4 with today's `max_tokens_exceeded` message |
| A `confirm` or relation question that fails | That text fails, as a failed word question does today. No partial names print for it |
| `recognize --jobs 8` on one document | Up to 8 split requests in flight |
| `recognize --field /body` with no framing flag | Reads the input as one JSON document and sends `/body` |
| `--word-keep` entry with a space inside | Usage error at exit 2 |
| 101 entries in one list | Usage error at exit 2 |
| Relation the text states but that is false (`Yoko Ono wrote Something.`) | The edge prints |
| Relation that is true but unstated | No edge |
| `--batch 1` under `--lines` | One text a request, byte for byte today's request |
| A batch where several texts need `confirm` | One `confirm` request for the batch after its word answers |

## What breaks

| What | Why it breaks | Ticket that repairs it |
| --- | --- | --- |
| `crates/thinkthen/tests/backend/recognize.rs` | Its loopback answers key off today's snippets and relation wording. The dry-run key order gains `confirm_questions_upper_bound`. Its 64,000-byte loopback test has no ceiling, stays one piece and keeps passing | R2 for words and dry-run fields, R3 for `confirm`, R4 for pieces and windows at the built-in address, R5 for the wording |
| `specification/backends.md` line 17, "Recognition keeps the complete source text in every request" | Word and `confirm` requests carry a window | R0 amends it, and R4 builds it |
| `spec/recognize.md` | Its prose says which trailing marks become words. Its pinned `Ada met Acme.` counts stay 4 words and 1 request | R2 |
| The recognize conformance cases in `conformance/cases.json`, such as `42-recognize-C01-relations` | Their bodies hold today's relation wording and today's words | R5 re-records the relation cases. R2 checks the rest under the new words |
| The relation recordings in `crates/thinkthen/tests/fixtures/recognize-239/` | They hold today's relation wording | R5 re-records them |
| The 40 harvest cases in `crates/thinkthen/tests/fixtures/recognize-225/` | C06 and C16 hold marks the default rules split, so their requests change. A case whose run changes kind would send a `confirm` request that no recording holds | R2 and R3 replay the harvest at `boundary: run`, and re-record C06 and C16 |
| Demo 44 | Checked under `confirm`: its words match today's, and no run changes kind (Maria Chen, Northwind Freight, Chicago), so `confirm` asks nothing and the replay should stay green | R2 and R3 replay it as proof |
| Old recognize runs under 0135's `audit --write` | The question digest changes | R2. The runs are rerun |

## Acceptance tests

The key is local experiment 277's key: 200 hand-written sentences with 372 names in 33 categories, whose first 100 are local experiment 267's key. Ticket 0164 copies it to `specification/fixtures/recognize/names.jsonl` as `audit` key lines. Ticket 0135 grades `recognize` with `audit --match strict`, so every scored test below grades with `audit` and no new scoring helper. A name counts only on exact offsets and kind.

1. **Reach, no model.** Over n001 to n100, under the default word rules, 166 of the 168 key names start and end on word edges. The exceptions are `London` in `London-based` and `Microsoft` in `anti-Microsoft`. With `-` added to `infixes`, all 168 are reachable. Under today's splitter, 141 are. Over the whole key, today's splitter reaches 322 of 372 names, and ticket 0164's unit test pins that count. R2 adds its two counts over all 372 names.
2. **Trim, no model.** Mark every word of a key name plus its next possessive or quote word as a name word. Assembly returns the key name exactly. A table test covers every possessive and quotes-brackets case except n011 and n012.
3. **Caller rules, no model.** `keep: ["McDonald's"]` makes n011 return `McDonald's`. A question file that removes `'s` from `trim` makes n001 return `George Harrison's`.
4. **Boundary, recorded.** Ticket R3 records one live run of the key under `confirm`, about $0.012. Replay it under `run` and under `confirm`, and grade both with `audit`. `confirm` separates n017, n018, n019 and n021. It loses no exact name that `run` finds.
5. **Regression, recorded.** Experiment 265's 30 sentences lose no exact name found today, except the known losses: `McDonald's`, `Sainsbury's`, `Octopus's Garden`, and strength-cut misses on `Sgt. Pepper's Lonely Hearts Club Band` and `St. Mary's Church`.
6. **Target, one recorded live run.** Use the key's kinds, `person place organisation work thing`, and every other default. `audit --match strict` reports recall and precision on the key of at least 85% each. Recall pooled over the possessive, quotes-brackets and side-by-side categories reaches at least 80%. Experiment 270 measured 87.5%, 88.6% and 83.3% pooled. Per category it measured 82.7%, 91.7% and 76.5%. Recording the 100 sentences costs about $0.012.
7. **Relations, one recorded live run.** On experiment 265's 30 sentences with `sang=person:song` and `wrote=person:song`, none of the five unstated edges in the closed relations issue passes 0.5. At least 21 of the 27 stated edges pass, today's count.
8. **Long text, loopback.** A 1,000-word text at the loopback backend under a profile with a `max_request_bytes` that splits it. `--dry-run` shows several requests, each within the limit. Each carries its piece's words plus the 200 words on each side, and no request carries the whole text. One key name's words fall on both sides of a piece boundary. The loopback run returns that name whole, with the offsets of one unsplit request. A second run in which the backend fails the second request fails the whole text, prints no names and echoes no text.
9. **Batch, recorded.** Run the key with `--jsonl --field /text` and the key's kinds, three times at `--batch 1` and three times at the default. The ticket reports the mean recall and mean precision of both, and the gap. It gates nothing, by Ian's ruling that speed wins. The recognize page states the gap with this record. The six runs cost about $0.07. `--batch 1` sends bytes identical to today's requests. The dry run of the first batch shows `closed` as `content`, `size`, `limit` or `end`.
10. **Hard cap, no model.** A 600,001-byte text exits 2 and sends nothing, at the built-in address and at a loopback address. A 600,000-byte text plans its requests. A loopback listener counts zero requests for the refusal.
11. **Clue in the window, loopback and one recorded run.** A 2,000-word text holds one name whose kind hangs on a clue sentence 150 words before it. The name is the first word of its piece, so the clue sits in the piece before. `--dry-run` shows that the name's request carries the clue sentence at the default window, and does not carry it at `--window 100`. One recorded live run at the default gives the name its right kind. The run costs under $0.02.

## Collisions with other work

- **Ticket 0135** grades `recognize` with `audit --match strict`. R1 therefore builds no scoring helper. Its question-digest check refuses old runs after R2, as "Defaults and what a caller can change" says.
- **Ticket 0132** deferred splitting `recognize` requests at the built-in address. R4 closes that gap.
- **`2026-09-25-recognize-and-relate-scale-and-shape.md`.** R8 covers its item 7, the symptom-to-dial table. Its item 8, the fixed news-document wording, stays untouched.
- **Ticket J1** in the batching design, formerly batching B11, brings the concurrent send that R4b uses.
- **Ticket 0137** makes `--field` with no flag mean JSON Lines on `filter` and `rank` only. `recognize` keeps one document.

## Tickets

In order. "Needs ADR" marks a ticket that changes a Settled specification page.

Any ticket that adds or changes a setting updates that setting's row in `specification/settings.md` in the same commit. Batching ticket C1 creates that page.

| # | Outcome | Scope | Proof | Depends on | ADR |
| --- | --- | --- | --- | --- | --- |
| R0 | Ian's rulings recorded | One ADR for word rules, `confirm` as default, stated relations, pieces and windows for long texts, the hard cap, `recognize --jobs` for one document, and the recognize batch shape. It lands after batching B0 and builds on its batch shape. It retires the 2026-09-23 maximal-run baseline as default and keeps it as `boundary: run`. It lifts the ban on recognition policy keys in `question-file.md` and ticket 0080. It amends ADR 0040 so the built-in ceiling reaches word and `confirm` questions. It amends `backends.md` line 17 so word and `confirm` requests carry a window in place of the complete text. It amends `question-file.md` line 130's key order, `result.md` for `answer.confirm` and `meta.batch`, `channels.md` for `--word-keep`, `--word-infix`, `--boundary` and `recognize --jobs`, `records.md` `jobs` for `recognize` on one document, and `relate.md` with R5's sentence. It defines `request_count` as the requests formed before any answer | ADR accepted; `recognize.md`, `question-file.md`, `result.md`, `channels.md`, `records.md`, `relate.md` and `backends.md` updated | batching B0 | This is the ADR |
| R1 | The keys live in the repo and `audit` grades them | Local experiment 277's 200-case key as `audit` key lines in `specification/fixtures/recognize/names.jsonl`; experiment 265's 30 sentences and its stated-edge key as `specification/fixtures/recognize/relations.jsonl` | Test 1's count of 322 under today's splitter; `audit` grades both keys | ticket 0135 | no |
| R2 | Default word rules on the command and question file | Splitter, trim, five lists in the question file, `--word-keep` and `--word-infix`, limits, digest, `--details` and `--dry-run` fields, secrecy | Tests 1, 2, 3 and 5; the secrecy test | R0, R1 | covered by R0 |
| R3 | `confirm` boundary | Confirm question, one request per text, `--boundary`, `answer.confirm`, secrecy; one authorized live recording of the key | Tests 4 and 6; the secrecy test | R2 | covered by R0 |
| R4 | Long texts split into windowed pieces | The built-in ceiling reaches word and `confirm` questions. Pieces, the window and `--window`. Names form after every request. The hard cap. Secrecy. Closes ticket 0132's deferred gap | Tests 8, 10 and 11; the secrecy test | R0 | covered by R0 |
| R4b | `recognize --jobs` for one document | The flag, default 4, over one text's pieces | A loopback count reaches the job count on a split text | R4, batching J1 | covered by R0 |
| R5 | Relations mean the text states them | Two question wordings, `recognize.md`, one sentence in `relate.md`, re-recorded relation fixtures | Test 7. If it fails, the ticket stops and reports the scores to Ian | R0 | covered by R0 |
| R6 | `keep`, `infixes` and `boundary` on every library and SQL surface | Rust builder, Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL and SQLite; the conformance cases | One shared conformance case per setting passes on every surface | R2, R3 | no |
| R7 | Many short texts batch on the command, filling to the limit by default | Batch evidence, question prefix and batched `confirm`, secrecy. One authorized live measurement. The ticket reports the cost and does not change the default. It removes `recognize` over records from batching S1's list. The library and SQL surfaces follow in their batching tickets B12a to B13e | Test 9; the secrecy test | R3; batching B4 and B5 | covered by R0 |
| R8 | The manual shows the dials | One recognize page: the steps, word rules, boundary, relation meaning, the window and its risk, the hard cap, the batching cost from test 9, and a symptom-to-setting table with measured numbers. One authorized live run of a 1,000-word document at the default window, about $0.015, reports its requests, time and tokens. Covers item 7 of the scale-and-shape issue | Docs review | R3, R4b, R5, R7 | no |

`.facts` on recognize results comes with each surface's batching ticket, B12a to B12f. No recognize ticket depends on `--context`, batching B7.

## Known gaps

- Touching names of one kind: `Paul John`, `Portland Oregon`.
- `Octopus's Garden` breaks in two in most tries.
- The strength formula drops some whole names on kind doubt: `Boeing 747` at 0.32 to 0.39, `Sgt. Pepper's Lonely Hearts Club Band` and `St. Mary's Church` in 5 of 9 tries. It stays until a measured change is proposed.
- The fixed news-document wording of the word questions, item 8 of the scale-and-shape issue.
- A clue further from a name than the window is out of view.
- The relation step keeps the whole text as evidence, so a long text with relation rules can pass Jev's evidence limit.

## Open items Ian can overturn

Ian's rulings. Ian can overturn each.

1. Relations in `recognize` mean the text states them.
2. `recognize --jobs` for one document, default 4.
3. Long texts split into pieces with a bounded window, under a hard cap.
4. Test 9 reporting in place of gating, because speed wins.

The author's calls. Ian can overturn each.

5. The window default of 200 words on each side, and its range of 0 to 5,000.
6. The hard cap of 600,000 bytes, with no option.
7. Batching texts for `recognize` by default, filling to the limit.
8. Every word default: the word lists, `confirm` over `run`, the 0.5 cuts, and hyphens joined.
9. The new relation wordings, and the bar of test 7.
10. The targets of test 6.
11. Leaving out `begin` and the split with no question.
12. The shared evidence name `records` for texts.
13. Only `keep`, `infixes` and `boundary` reaching surfaces beyond the question file.
14. `--field` with no framing flag reading one document on `recognize`, unlike `filter` and `rank`.
15. The relation step keeping the whole text as evidence for now.
