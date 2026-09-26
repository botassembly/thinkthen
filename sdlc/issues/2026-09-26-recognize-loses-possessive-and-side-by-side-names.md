# recognize: configurable word rules and name boundaries

Status: Held by Ian. Do not start until Ian sends it. Filed 2026-09-26 from workspace experiments 265, 267 and 270. This body replaces the first filing and keeps its experiment 265 evidence and strength item.

## Summary

`recognize` gets name edges wrong on common English forms: possessives, quoted titles, bracketed names, hyphenated words, and two names with no word between them. A third fault drops a detected name when its words disagree on the kind. The failures come from three places. The word splitter decides which pieces of text the model sees. The tagging scheme decides how the model's answers become names. The strength formula decides which names pass the cut.

This issue asks for two defaults, measured on a 100-sentence key with 168 names:

1. **Word rules.** Split possessives, quote marks and brackets off a word before asking about it. Trim them from a name's edges afterwards.
2. **The `confirm` boundary.** When a run of name words changes kind, ask the model once whether the run is one name or several.

Together they raised exact-span recall from 70.8% to 87.5% and precision from 76.3% to 88.6%. The strength item stays open. A three-way begin/inside/outside question was measured and is left out for now.

## Evidence from experiment 265

Workspace experiment 265 ran 30 hand-written plain English sentences three times against `jev-latest` (`jev-1.13.0`). The recognize code tested is byte-identical to main `0f255579`. Overall exact-span recall was 82.2% (60 of 73) and precision 88.2%. Every exact span had the right kind. Possessive cases scored 70%, side-by-side cases 50%, and quoted-title cases 42.9% recall. Easy, small-word and unquoted-punctuation cases scored 100%. Each result below was the same in all three runs. Asked directly about the whole span, Jev got every one right.

### 1. Attached possessives

| Input | recognize returns | Jev asked directly |
| --- | --- | --- |
| `Octopus's Garden was Ringo Starr's song.` | `Ringo Starr's` person, 21 to 34 | picks `Ringo Starr` over `Ringo` and `Ringo Starr's`, 1.0 |
| `George Harrison's song Something appears on Abbey Road.` | `George` person | picks `George Harrison`, 0.97 |
| `Paul McCartney's band Wings played in Glasgow.` | `Paul` person | picks `Paul McCartney`, 0.99 |

Smallest reproduction, with no key:

```sh
echo "Octopus's Garden was Ringo Starr's song." | thinkthen recognize person song --dry-run | jq -r '.requests[0].body_utf8'
```

The detection snippet is `was Ringo [[Starr's]] song .`. Jev must judge `Starr's` as one word. It said IN at 0.73 for `Starr's`, so the name keeps the `'s`. For `Harrison's` it said IN at 0.31 and for `McCartney's` at 0.36, so those names lose their last word.

The rulings disagree with the shipped behaviour. `sdlc/issues/closed/2026-09-21-four-small-closings-for-the-recognize-team.md` item 3 states the rule as "a trailing `'s` trims off and a middle one stays". Ticket 0080 keeps "the fixed trailing-possessive rule". `specification/recognize.md` says "A trailing separate possessive is removed". On raw text the shipped rule removes nothing.

### 2. Names side by side

| Input | recognize returns | Jev asked directly |
| --- | --- | --- |
| `On Revolver Paul McCartney sang Eleanor Rigby.` | `Revolver Paul McCartney` person, 3 to 26 | "Is Revolver Paul McCartney a single name?" 0.12. Revolver album 1.0, Paul McCartney person 1.0 |
| `In Liverpool John Lennon met Paul McCartney.` | `Liverpool John Lennon` person, 3 to 24 | single name? 0.27. Liverpool place 0.99 |
| `At Abbey Road Studios George Martin recorded Revolver.` | neither name | single name? 0.11. Abbey Road Studios place 0.98, George Martin person 1.0 |

Per-word answers for the third input (detection, then kind in the order person, song, album, place, organisation):

```text
Abbey    0.99  place 0.77
Road     1.00  place 0.82
Studios  1.00  place 0.51, organisation 0.49
George   0.97  person 0.95
Martin   0.98  person 0.97
```

Jev puts every word in a name and gives each name its own kind. `assemble` makes the five words one name. `kind_vote` picks place, three words to two. `candidate` averages the place probability over all five words, so strength is 0.97 × 0.426 = 0.413. The cut drops the whole run, and two names Jev found at 0.97 or better are both gone. The same path turns `Revolver Paul McCartney` into one person. The relation step then reports `sang(Revolver Paul McCartney, Eleanor Rigby)`.

### 3. A detected name dropped over its kind

| Input | recognize returns | Jev asked directly |
| --- | --- | --- |
| `Ringo Starr sang "Octopus's Garden" on Abbey Road.` | no `Abbey Road` | "Is Abbey Road a single name?" 0.83, and it is an album, 0.68 |

`Abbey` is IN at 0.99 with place 0.78. `Road` is IN at 0.99 with album 0.83. The kind vote ties, one word each, and the first word's kind wins. `candidate` averages the place probability over both words: (0.78 + 0.15) / 2 = 0.465. Strength is 0.99 × 0.465 = 0.46, under the 0.5 cut. A name detected at 0.99 disappears because its words disagree on the kind. The specification defines strength this way (`specification/recognize.md`, "Names").

### Quoted titles

Quoted titles came back with their marks: `"Octopus's Garden"`, `"Help!"`, `'Boys'`. Asked directly, Jev also picked the quoted form (0.72, 0.91, 0.61), so experiment 265 filed no defect for them. Every annotation guide checked leaves quote marks out: ACE, Universal NER, and CoNLL's split tokens. The design below trims them by default. Unquoted `Help!` came back exact before a space and before a comma. Titles with small words inside, such as `Lucy in the Sky with Diamonds`, came back whole.

## Evidence from experiment 267

Workspace experiment 267 studied the conventions and built the key. It made no model call.

- `cases.jsonl` holds 100 hand-written sentences with 168 names across 14 categories. The categories cover possessives in every form, side by side, quotes and brackets, punctuation, small lowercase words, hyphens, initials, case, nesting, lists, sentence edges, numbers, non-English letters and no names. A check script validates the offsets.
- A copy of today's splitter reaches 141 of the 168 names at all. The other 27 have an edge inside a whitespace word: 12 possessive, 10 quote or bracket, and 5 hyphen. The proposed word rules reach 166.
- Six pairs of key names touch. Four differ in kind and two share a kind. In/out tagging merges every such pair.
- MUC-7, ACE 2005 and CoNLL-2003 split `'s` into its own token and leave it outside a possessor's name. They keep it inside a name that is possessive by nature, such as `McDonald's`. Universal NER includes the possessive. spaCy, NLTK, CoreNLP and Hugging Face all split `'s` before tagging.
- Stanford's IOBUtils documentation says IO tagging cannot represent adjacent entities of one class. spaCy uses BILUO, Stanza uses BIOES, and CoNLL-2003 used IOB1.

## Where the answers are lost

1. **Word splitting.** `tokenize` splits on white space (`crates/thinkthen/src/core/recognize.rs:70`). `split_piece` peels only `. ! ? , : ;` (`:95`). The model can answer only for a whole word, so a name cannot end inside `Harrison's` or start after `"`. The trailing possessive rule (`:228`) needs a word equal to `'s`. The splitter never makes one, so the rule never fires.
2. **Tagging scheme.** `assemble` (`:221`) makes one name from each run of `in` words. This is in/out tagging. ThinkThen asks a kind question for every word, then discards the kinds when it forms names.
3. **Strength.** `candidate` averages the winning kind's probability over every word (`:266`). `kind_vote` (`:279` to `:310`) lets the first word's kind win a tie. A name detected at 0.99 falls under the cut at `:233` when its words disagree on kind.

## Measured in experiment 270

Workspace experiment 270 ran the 100-sentence key against Jev (`jev-1.13.0`), three live runs per variant. Kinds were `person place organisation work thing`, with no descriptions. The "today" variant is the shipped binary, with recognize code byte-identical to main. The other variants are simulated, because ThinkThen cannot take the new rules yet. A script splits the words, builds ThinkThen's own detection and kind questions, and sends them to the same endpoint. Under today's splitter the script builds all 100 request bodies byte-identical to the binary's dry run. Its assembly reproduces the binary's names and strengths exactly on all 300 recorded cases.

- **Word rules** use the Part 1 defaults below and today's run rule.
- **Confirm** adds the Part 2 choice question for each run whose word kinds change.
- **Begin** asks each word `BEGIN`, `INSIDE` or `OUT`, worded in the style of the detection question. Two wordings ran once each, and the better one ran three times. Kinds came from the word-rules run.

A name counts only on exact offsets. Counts sum over the three runs. A possessive or quote case is right when its names and kinds match the key exactly, with nothing extra.

| Variant | Recall | Precision | Label accuracy | Touching pairs split right | Possessive and quote cases right | Requests per case |
| --- | --- | --- | --- | --- | --- | --- |
| today | 70.8% (357/504) | 76.3% (357/468) | 98.9% | 0/18 | 14/78 | 1.00 |
| word rules | 82.7% (417/504) | 86.3% (417/483) | 99.0% | 0/18 | 63/78 | 1.00 |
| word rules + confirm | 87.5% (441/504) | 88.6% (441/498) | 99.1% | 12/18 | 63/78 | 1.09 |
| word rules + begin | 88.5% (446/504) | 86.8% (446/514) | 98.4% | 12/18 | 57/78 | 1.00 |

Per category, recall and precision at the 0.5 cut:

| Category | Cases | today | word rules | + confirm | + begin |
| --- | --- | --- | --- | --- | --- |
| possessive | 16 | 54.3% / 59.5% | 82.7% / 83.8% | 82.7% / 83.8% | 86.4% / 82.4% |
| side-by-side | 7 | 29.4% / 50.0% | 29.4% / 50.0% | 76.5% / 86.7% | 76.5% / 86.7% |
| quotes-brackets | 10 | 37.5% / 40.0% | 91.7% / 86.3% | 91.7% / 86.3% | 95.8% / 86.8% |
| punctuation | 9 | 84.6% / 73.3% | 82.1% / 72.7% | 82.1% / 72.7% | 84.6% / 73.3% |
| small-words | 6 | 100% / 100% | 96.7% / 100% | 96.7% / 100% | 90.0% / 79.4% |
| hyphen | 5 | 44.4% / 66.7% | 55.6% / 100% | 55.6% / 100% | 55.6% / 100% |
| initials-abbrev | 7 | 66.7% / 66.7% | 91.7% / 91.7% | 91.7% / 91.7% | 83.3% / 83.3% |
| case | 6 | 90.0% / 90.0% | 90.0% / 90.0% | 90.0% / 90.0% | 90.0% / 90.0% |
| nested | 5 | 91.7% / 100% | 95.8% / 100% | 95.8% / 100% | 100% / 72.7% |
| list-and | 6 | 93.8% / 88.2% | 93.8% / 88.2% | 93.8% / 88.2% | 100% / 100% |
| sentence-edge | 6 | 100% / 100% | 100% / 100% | 100% / 100% | 100% / 100% |
| numbers | 6 | 90.9% / 100% | 90.9% / 100% | 90.9% / 100% | 90.9% / 100% |
| non-english | 6 | 100% / 100% | 100% / 100% | 100% / 100% | 100% / 100% |
| no-names | 5 | 0 false names | 7 false | 7 false | 3 false |

What the numbers show:

1. **Today's misses** come from word splitting (81 over three runs), touching names (36), Jev's judgement (25) and the strength cut (5). The 81 are the 27 names experiment 267 found unreachable, missed in every run.
2. **Word rules** fix every possessive and quote failure from experiment 265. They lose some names today finds. `McDonald's` and `Sainsbury's` lose their `'s` to `trim`. `Octopus's Garden` breaks in two in 5 of 6 tries, because Jev gives the split-off inner `'s` about 0.5. `Sgt. Pepper's Lonely Hearts Club Band` falls under the cut in some runs, because its `'s` word scores 0.71 to 0.79. The split also makes `rock 'n' roll` in a no-names case come back as false names.
3. **Confirm** separates all four touching pairs of different kinds (n017, n018, n019, n021) in every run. It loses no name that word rules find. It kept `Eleanor Rigby`, `St. Mary's Church`, `University of Liverpool`, `Sgt. Pepper's Lonely Hearts Club Band` and `Boeing 747` whole. It sent 30 questions in 28 requests over the three runs.
4. **Begin** separates the same four pairs and no more. Under either wording, Jev did not mark `John` in `Paul John` or `Oregon` in `Portland Oregon` as starting a name. The second wording named a side-by-side example and did worse. Begin gains `Octopus's Garden`. It loses `The Lord of the Rings`, `The White Album` and `Jane Goodall` (it keeps `Dr.`). It returns a lone `The` or `the` as a name in eight cases.
5. **The strength cut.** A cut tuned on half the cases and tested on the other half landed between 0.30 and 0.55. It moved test-half recall and precision by at most 2.5 points, in both directions. The 0.5 default stays.
6. **Hard against easy categories.** With word rules and confirm, recall on the possessive, quotes-brackets and side-by-side categories is 83.3%. On the other categories it is 89.8%. Today the figures are 42.8% and 86.4%.

Misses left under word rules + confirm, over three runs:

| Case | Key name | What came back | Cause |
| --- | --- | --- | --- |
| n020, n022 | `Paul`, `John`; `Portland`, `Oregon` | `Paul John`; `Portland Oregon` | touching names of one kind |
| n011, n012 | `McDonald's`, `Sainsbury's` | `McDonald`, `Sainsbury` | default trim; a caller's `keep` fixes it |
| n003, n024 | `Octopus's Garden` | `Octopus` and `Garden` in 5 of 6 | Jev puts the inner `'s` word out of the name |
| n038 | `Washington, D.C.` | `Washington` and `D.C.` | Jev's judgement on the comma |
| n042, n058 | `Acme Widgets Inc.`, `Martin Luther King Jr.` | no final period | Jev leaves the final period out of the name |
| n073 | `Simon and Garfunkel` | `Simon` and `Garfunkel` | Jev's judgement on `and` |
| n050, n053 | `London` in `London-based`, `Microsoft` in `anti-Microsoft` | nothing | word splitting; hyphens stay joined by default |
| n051 | `Paris`, `Rome` in `Paris–Rome` | `Paris–Rome` under the cut | Jev marks the dash in the name |
| n065 | `Apple` in `Apple phones` | `Apple phones` under the cut | Jev's judgement |
| n084 | `Boeing 747` | exact, strength 0.32 to 0.39 | strength formula |
| n010, n043, n039 | `Sgt. Pepper's…`, `St. Mary's Church` | exact, under the cut in 5 of 9 | strength formula |
| n033, n068 | `United Nations`, `Bank of England` | `the United Nations`, `The Bank of England`, once each | Jev's judgement |

Spend: 1,028 exchanges, 2,546,703 input tokens, $0.107.

## Design

### Part 1. Word rules

Five settings decide how text becomes words. Entries are literal strings. There are no regular expressions and no callbacks.

| Setting | Default | Meaning |
| --- | --- | --- |
| `prefixes` | `"` `“` `‘` `'` `(` `[` `{` | Peeled from a word's start, one at a time |
| `suffixes` | `.` `!` `?` `,` `:` `;` `"` `”` `’` `'` `)` `]` `}` `'s` `’s` `'S` `’S` | Peeled from a word's end, longest first, one at a time |
| `infixes` | `–` `—` | Split out of a word between letters |
| `keep` | empty | Whole words never split, such as `McDonald's` or `Sgt.` |
| `trim` | opening marks at a name's start. Closing marks, `'s`, `’s`, `'S`, `’S`, and a bare `'` or `’` at a name's end | Removed from a name's edges after it is formed |

Fixed rules:
- A final period stays on a word that holds another period, so `U.S.`, `J.R.R.` and `D.C.` stay whole.
- The model answers for every word, marks included. `Help!` keeps its `!` when the model says the `!` belongs.
- `trim` is the only rule that overrides the model.
- Offsets always point into the original text. Internal marks come back as written: `Rock 'n' Roll Music`, `Octopus's Garden`, `Washington, D.C.`

Each list adds to its default. `defaults: false` starts every list empty. The defaults follow MUC-7, ACE and CoNLL. A caller gets Universal NER's convention by removing the possessives from `trim`.

### Part 2. Boundaries

One setting, `boundary`, decides how answers become names. The default is `confirm`.

| Value | Rule | Extra cost | Separates |
| --- | --- | --- | --- |
| `confirm` (default) | For each run whose word kinds change, ask one choice question: the whole run as one name, or the run split at each kind change | one question per such run; 0.09 extra requests per case on the key | touching names of different kinds. It keeps a name whole when the model says it is one name |
| `run` | Today's rule. A run of `in` words is one name | none | no touching names |

The measured question, asked with the whole text as state:

```text
In this text, is "Revolver Paul McCartney" one name, or is it separate names that stand side by side with no word between them?
ONE:   One name: "Revolver Paul McCartney".
SPLIT: Separate names: "Revolver", "Paul McCartney".
```

On `SPLIT`, each part is trimmed and takes its kind by the same vote a run uses. On `ONE`, the run stays whole and keeps its vote. The kind does not come from the choice answer.

Considered and left out:
- A split at every kind change, with no question asked. It would cut `Abbey Road`, whose words Jev put in place (0.78) and album (0.83).
- `begin`, a three-way begin/inside/outside question per word. Experiment 270 measured it. It separated no pair that `confirm` missed, and it lost titles that start with `The`. A later issue can bring it back with new measured evidence.

### Part 3. Strength

The strength item from experiment 265 stays open under this issue. `candidate` averages the winning kind's probability over every word. With the default cut, kind doubt removes the name itself, and the user gets no name with a doubtful kind. Experiment 270 adds three cases: `Boeing 747` scored 0.32 to 0.39, and `Sgt. Pepper's Lonely Hearts Club Band` and `St. Mary's Church` fell under the cut in 5 of 9 tries. Tuning the cut did not help. The strength formula stays as specified until a measured change is proposed.

### Surfaces

| Surface | Word rules | Boundary |
| --- | --- | --- |
| Command | `--word-prefix`, `--word-suffix`, `--word-infix`, `--word-keep`, `--name-trim` (repeatable), `--no-default-words` | `--boundary confirm\|run` |
| Question file | `recognize.words` object with the five lists and `defaults` | `recognize.boundary` |
| Rust | `RecognizeBuilder::words(Words)`, with `Words::default()` and `Words::empty()` | `RecognizeBuilder::boundary(Boundary)` |
| Libraries | `words=` | `boundary=` |
| SQL | `words` key in the JSON recognize section already accepted in place of kinds | `boundary` key in the same section |

`--details` metadata prints the effective rules and boundary. The question digest includes them, so different rules never share a cache entry. `--dry-run` counts `words` under the effective rules. Limits: at most 100 entries per list, each nonblank, with no white space inside, and at most 32 characters.

## Known gaps

- **Touching names of one kind.** n020 `The producer told Paul John had left.` returns `Paul John`. n022 `She flew into Portland Oregon last spring.` returns `Portland Oregon`. `confirm` asks only when the kinds change, and `begin` did not split them either.
- **Names possessive by nature.** n011 and n012 return `McDonald` and `Sainsbury`. The default `trim` removes their `'s`. A caller adds `McDonald's` and `Sainsbury's` to `keep`. ThinkThen ships no word list, as the 2026-09-21 ruling requires.

## Acceptance test

The key is workspace experiment 267's `cases.jsonl`. Copy it into the repository as a fixture and score by exact character offsets and label.

1. **Reachability, with no model.** Under the default word rules, 166 of the 168 key names start and end on word edges. The two exceptions are `London` in `London-based` and `Microsoft` in `anti-Microsoft`. With `-` added to `infixes`, all 168 are reachable. A unit test pins both counts.
2. **Trim, with no model.** When every word of a key name plus its adjacent possessive or quote word is marked `in`, assembly returns the key name exactly. A table test pins this for every possessive and quotes-brackets case except n011 and n012.
3. **Caller extension, with no model.** `keep: ["McDonald's"]` makes case n011 return `McDonald's`. Removing `'s` from `trim` returns `George Harrison's` for n001.
4. **Boundaries, with recorded answers.** Replay a recorded run under `run` and `confirm`. `confirm` separates the four different-kind touching pairs (n017, n018, n019, n021). It loses no name that `run` finds exactly.
5. **Regression, with a recorded run.** The experiment 265 key loses no exact span it finds today, apart from the known losses below. The known losses on the 100-sentence key are `McDonald's`, `Sainsbury's`, `Octopus's Garden`, and strength-cut misses on `Sgt. Pepper's Lonely Hearts Club Band` and `St. Mary's Church`.
6. **Target, with one recorded live run.** Under the defaults, overall exact-span recall and precision on the key each reach at least 85%. Recall on the possessive, quotes-brackets and side-by-side categories reaches at least 80%. Experiment 270 measured 87.5%, 88.6% and 83.3%. The margin allows for Jev's run-to-run variation.

Items 1 to 3 run offline. Items 4 to 6 need recordings from the built feature. Recording the 100 sentences once costs about $0.012.

## Trade-offs

- The default rules add 46 words to the 772 now asked about in the key, 6% more questions.
- Peeled marks change request bytes. Recordings and cache entries for texts that contain them stop matching.
- `keep` is a word list. The default stays empty.
- Splitting `'s` inside a title makes Jev judge it alone. `Octopus's Garden` broke in two in 5 of 6 tries.
- Trimming a bare `'` loses the mark in a title such as `Keep On Rockin'`. A caller can edit `trim`.
- Hyphens stay joined by default. `Hewlett-Packard` stays whole, and `London-based` gives no `London`.
- `confirm` adds one question for each run whose word kinds change.

## What is asked

1. Record an ADR. `specification/recognize.md` is Settled. Ticket 0080 step 2 and `specification/question-file.md` forbid recognition policy options. `sdlc/planning/architect-engine-survey-and-plan.md` records Ian's later request for configurable boundary and possessive rules. The ADR retires the 2026-09-23 plain maximal-run baseline as the default and keeps it as `boundary: run`.
2. Ticket the word rules with the Part 1 defaults. Acceptance items 1 to 3 and 5 gate them.
3. Ticket `boundary` with `confirm` as the default and `run` as the other value. Acceptance items 4 and 6 gate it. Leave `begin` out.
4. Keep the strength item open under this issue until a measured proposal exists.
5. Keep the two known gaps open under this issue.

Ian can overturn:
- every default, every list entry and every option name;
- the choice of `confirm` over `run` as the default, and the choice to leave out `begin`;
- the drop of the kind-change split with no question;
- the wording of the `confirm` question and of both measured `begin` wordings;
- the acceptance targets and their margin;
- the scoring rule that a case is right only when every name and kind matches the key exactly, with nothing extra;
- experiment 270's reuse of the word-rules kind answers for `begin`, in place of asking the kind questions again.
