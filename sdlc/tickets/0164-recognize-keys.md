---
flow: build
priority: 164
opens: specification/fixtures/recognize sdlc/scripts/recognize-keys sdlc/scripts/lint sdlc/scripts/spec sdlc/scripts/README.md crates/thinkthen/src/core/recognize.rs sdlc/ratchet.json sdlc/issues/2026-09-26-recognize-design.md sdlc/records sdlc/tickets
---

# 0164: The recognize keys live in the repository, and audit grades them

Status: ready. The coordinator accepted it on 2026-09-26 after two fresh read-only reviews. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A builder or reviewer grades any `recognize` run against the 200-sentence name key with one `audit --match strict` command. The key lives in the repository, every offset in it is checked on every lint, and a no-model check proves that `audit` accepts it. The 30 relation sentences of local experiment 265 live beside it, with their names and their stated edges. A reader filters either key to a run's kinds with one `jq` file, because `audit` refuses a key kind the run did not ask for.

This is row R1 of `sdlc/issues/2026-09-26-recognize-design.md`, "The keys live in the repo and `audit` grades them", and the "recognize R1" row of `sdlc/planning/backlog-0-1-2026-09-26.md` line 102, which says "It can start today". Its only dependency, ticket 0135, has landed. The design's "Acceptance tests" section (line 341) reads: "Ticket R1 copies it to `specification/fixtures/recognize/names-100.jsonl` and writes it as `audit` key lines. Ticket 0135 grades `recognize` with `audit --match strict`, so every scored test below grades with `audit` and no new scoring helper." The row was never ticketed, and Ian asked where the key went.

The name key comes from local experiment 277. Ian wants `recognize` to handle every subtle case. The coordinator therefore ruled that this ticket ships 277's 200-case key now. Its first 100 lines are 267's key, byte for byte. Ian can overturn that ruling.

Design tests 1, 4, 5, 6 and 9, and ticket 0147's tests 5 and 12, read this key. R2 and R3 cannot build without it.

## What happens today

- Main holds no copy of either key. The 200 sentences sit only in local experiment 277's `cases.jsonl`, 62,057 bytes, SHA-256 beginning `c361329c6d5b16f8`. Each line holds `id`, `text`, `entities` (each `text`, `start`, `end`, `label`), `category` and `note`. Offsets count Unicode code points.
- Lines n001 to n100 of that file are byte-identical to local experiment 267's `cases.jsonl` (SHA-256 beginning `87b49d13e01d6f64`). Lines n101 to n200 are new.
- Local experiment 277's `tools/check_cases.py` checks those offsets and prints `200 cases, 372 names, 0 with problems`. It lives outside the repository, so no rung runs it.
- The key has 33 categories: possessive 20, quotes-brackets 10, punctuation 9, extra-types 9, titles-honorifics 8, side-by-side 7, initials-abbrev 7, small-words 6, case 6, list-and 6, sentence-edge 6, numbers 6, non-english 6, regnal-sequel 6, the-article 6, metonymy 6, ambiguous 6, hyphen 5, nested 5, no-names 5, brand-case 5, handles-hashtags 5, shared-names 5, descriptions 5, abbrev-defined 4, nicknames 4, long-titles 4, url-email 4, dates-numbers 4, emoji 4, informal-lowercase 4, transliteration 4, line-break 3.
- The key has ten kinds. The five core kinds are place 96, person 90, organisation 82, work 57 and thing 22. The five extra labels are event 10, language 6, law 4, award 3 and nationality 2, 25 names in all.
- Local experiment 277's scoring treats the extra labels as optional for a system that knows only the five core kinds. It follows MUC-7's `STATUS="OPT"`: a prediction at an extra-label span counts neither as a hit nor as a false alarm, and a missed extra-label name is not counted.
- The 30 relation sentences sit only in local experiment 265's `cases/cases.jsonl`, SHA-256 beginning `3b63bb99b5f6c0cf`, at that experiment's commit `2aea0a7`. Each entity is `[name, kind]` with no offsets. Relations are `[relation, source name, target name]`. Three lines carry `optional_relations`, and c17 carries `alt_labels` (`Indica Gallery` may be `place`). The five unstated edges that design test 7 names are not in that file. Experiment 265's README, misses row 11, names them.
- `audit` reads a `recognize` key line as `{"id": …, "value": {"entities": [{kind, start, end}, …]}}` and ignores other members (`specification/audit.md`, "Inputs"). It refuses a key kind the line's question lacks, at exit 2 with `thinkthen: audit: key line N names a level, label, or unit the question does not have`.
- `audit` does not grade `recognize`'s relations (`specification/audit.md` line 142: "`recognize`'s beta relations are not graded"). It grades `relate` edges `{relation, source: {name, kind}, target: {name, kind}}`.
- Today's splitter is `tokenize` in `crates/thinkthen/src/core/recognize.rs:70`. Its tests pin two short strings.

I checked the shape before writing this ticket. I converted both keys in a scratch folder and graded them with a debug build of this branch at `f04a2cb0`, which holds ticket 0135's `audit`. Every line in proof test 2 is that build's output. A scratch test that called today's `tokenize` gave the counts in proof test 3.

## Design

### The files

`specification/fixtures/recognize/` gains four files.

**`names.jsonl`**, one line per sentence of local experiment 277's key, in its order. Each line is an `audit` key line and a `recognize` input record at once:

```json
{"id":"n001","text":"George Harrison's song Something appears on Abbey Road.","category":"possessive","note":"…","value":{"entities":[{"name":"George Harrison","kind":"person","start":0,"end":15},{"name":"Something","kind":"work","start":23,"end":32},{"name":"Abbey Road","kind":"work","start":44,"end":54}]}}
```

- `value.entities` takes the command's own value shape: `name`, `kind`, `start`, `end`, in text order. `label` becomes `kind`, and `text` becomes `name`. The kinds are the key's own ten, spelled as the key spells them.
- The five extra labels stay as ordinary kinds. A run is graded against the key filtered by `kinds.jq` to the run's kinds. A run under the five core kinds therefore never sees an extra-label name in the key.
- `text`, `category` and `note` are copied. `audit` ignores them. A run reads the text with `--field /text`, and `audit` finds the record by its default `--id /id`.
- Five notes say "Experiment 265 …": n001, n017, n019, n024 and n034. The converter writes "Local experiment 265 …" in those five and changes no other byte of any note.
- A no-names line has `"entities":[]`, which `audit` counts as labeled.

**`relations.jsonl`**, one line per sentence of local experiment 265, in its order:

```json
{"id":"c01","text":"Ringo Starr sang Octopus's Garden.","types":["easy","possessive"],"value":{"entities":[{"name":"Ringo Starr","kind":"person","start":0,"end":11},{"name":"Octopus's Garden","kind":"song","start":17,"end":33}]},"relations":[{"relation":"sang","source":{"name":"Ringo Starr","kind":"person"},"target":{"name":"Octopus's Garden","kind":"song"}}],"unstated":[{"relation":"wrote","source":{"name":"Ringo Starr","kind":"person"},"target":{"name":"Octopus's Garden","kind":"song"}}]}
```

- `value.entities` is the name key, as in `names.jsonl`. Offsets come from a forward search that starts at the previous name's end, as experiment 265's scorer did. That places c24's two `Yellow Submarine` names in order.
- `relations` holds the 27 stated edges in `relate`'s edge shape. Each endpoint takes its kind from the line's own entities.
- `optional_relations`, present on c10, c11 and c12, holds the edges experiment 265 neither credited nor penalised, in the same shape.
- `unstated` holds the five edges below, on their five lines. Each is true in the world, and its sentence does not state it. Design test 7 requires that none of them passes 0.5.
- `alt_kinds` on c17 is `{"Indica Gallery":["place"]}`, copied. `audit --match strict` does not read it, so a `place` answer there counts as a miss. The README says so.
- `types` is copied from the source's `type`.

| Line | Unstated edge |
| --- | --- |
| c01 | `wrote` Ringo Starr, Octopus's Garden |
| c02 | `sang` John Lennon, All You Need Is Love |
| c05 | `sang` Paul McCartney, Let It Be |
| c06 | `wrote` Paul McCartney, Help! |
| c14 | `sang` Paul McCartney, Yesterday |

**`kinds.jq`** keeps every line and drops the names whose kind is not listed:

```jq
# Keep each key line. Keep only the names, and on relation lines only the edges, whose kinds $kinds lists.
.value.entities |= map(select(.kind | IN($kinds[])))
| if has("relations") then .relations |= map(select((.source.kind | IN($kinds[])) and (.target.kind | IN($kinds[])))) else . end
```

A reader runs `jq -c --argjson kinds '["person"]' -f specification/fixtures/recognize/kinds.jq specification/fixtures/recognize/names.jsonl`. A line whose names all drop stays, with an empty list, so a name printed there still counts as extra. This is the per-kind key that ticket 0147's tests 5 and 12 and the design's test 5 need. It is a `jq` step and adds no scoring helper. No filtered copy is committed.

**The optional-span rule is not expressible here.** Under `audit --match strict` and `kinds.jq`, local experiment 277's MUC-7 rule cannot hold. A core-kind answer at an extra-label span counts as extra, where 277 counts it neither way. For example, a five-kind run that prints `Wimbledon` in n143 as `thing` loses one point of precision. The misses agree with 277, because the filter drops extra-label names from the key. The effect is bounded by the 25 extra-label names of 372. The README states this, and deferred gap 5 keeps OPT grading.

**`README.md`** says:

- the source of each file: "written by hand for this repository in local experiments 267 and 277" and "local experiment 265", with each source's SHA-256 and the date, 2026-09-26. It says lines n001 to n100 are local experiment 267's key unchanged;
- the counts: 200 sentences, 372 names, the 33 categories with their counts in one paragraph, as in line 28 of this ticket, and the names per kind, core and extra;
- the license: every sentence was written for this project, and the files are under the repository's MIT license. No sentence comes from a dataset. Local experiment 267 kept a 25-group WNUT-17 sample, which is CC-BY 4.0, and this ticket copies none of it. Note n066 names WNUT-17 as the source of a known confusion, and copies no WNUT-17 text. Local experiment 277's rules cite Universal NER, MUC-7, OntoNotes, ACE, WNUT-17, CheckList and RockNER, and copy no text from any of them;
- the convention the name key follows: local experiment 277's 29 rules, copied. Rules 1 to 11 are 267's rules, and 277 widened rules 1, 3 and 5;
- the agreement result: a blind second annotator agreed with the key on 59 of 60 sampled cases after one key fix, with span F1 and labelled F1 both 0.995. The second annotator is the same model family as the key's author, so the figure is likely high;
- the kinds of each file, and that `organisation` is spelled as the key spells it;
- how to grade a run, and how to filter by kind, as executable blocks (proof test 2);
- the known key limits: the optional-span rule above, c17's alternative kind, the optional edges, and `Mr. and Mrs. Smith` in n152, where the key gives one span, `Smith` at 13 to 18, for two people.

### The script

`sdlc/scripts/recognize-keys` is one Python file with three modes.

- `recognize-keys convert names SOURCE` and `recognize-keys convert relations SOURCE` print the converted lines to standard output. The builder runs each once and commits the output. The five unstated edges are a literal table in the script, citing local experiment 265's README, misses row 11. The rung cannot run the conversion, because the sources stay outside the repository. The record gives both commands and both source checksums, so a reviewer with the experiments can rerun them and compare bytes.
- `recognize-keys` with no argument checks both committed files and prints one line each, such as `names.jsonl: 200 lines, 372 names, every offset exact`. It exits 1 on the first file with a problem and names the line and the problem.
- `recognize-keys --self-test` builds each planted fault below in a temporary file and checks that the check refuses it with its pinned sentence. It then checks a clean two-line file passes.

The check reads each file whole and requires, for every line:

- a JSON object with a string `id`, unique in its file, and a nonempty string `text`;
- `value.entities` as a list of objects with exactly `name`, `kind`, `start` and `end`;
- integer offsets with `0 <= start < end <= len(text)`, counted in code points;
- `text[start:end] == name`, and no white space at either end of `name`;
- names in text order, none overlapping the one before;
- a nonempty string `kind`;
- in `relations.jsonl`, every endpoint of `relations`, `optional_relations` and `unstated` equal to the `name` and `kind` of an entity of its own line, and no edge in both `relations` and `unstated`.

The check reads no count or kind list from a constant. The counts block in the README pins the counts.

### The reach count

`core/recognize.rs`'s test module gains one test. It reads `names.jsonl` with `include_str!`, as `core/batch/tests.rs` reads its fixture. It parses each line with `serde_json`, runs today's `tokenize` on the text, and counts the key names whose `start` is some word's `start` and whose `end` is some word's `end`. It pins two numbers: 322 of 372 names reachable, and 1,735 words asked.

The same scratch test over lines n001 to n100 alone gave 141 of 168 and 772 words. Those match section 11 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`, so the scratch test counts as that record did.

The design's other two counts, 166 under the default rules and 168 with `-` as an infix, cover n001 to n100 and wait for R2. The rules they count do not exist in the product yet. Computing them now would mean a second copy of R2's splitter in Python, and that copy would pin itself rather than the product. R2 counts all 372 names under both rule sets and adds both counts to this test, with their word counts. R2 decides whether 322 stays as a count under today's rules written as a word-rules value, or leaves with today's splitter.

### The design issue

The lander changes three lines of `sdlc/issues/2026-09-26-recognize-design.md`:

- line 341 names local experiment 277's key, 200 sentences with 372 names in 33 categories, whose first 100 are experiment 267's, at `specification/fixtures/recognize/names.jsonl`;
- line 343, test 1, says its 166 and 168 count n001 to n100, and that today's splitter reaches 322 of the key's 372 names;
- line 372, row R1, names `names.jsonl` and `relations.jsonl`.

## Decisions

1. **The paths are `names.jsonl` and `relations.jsonl`, with no count.** The design names `names-100.jsonl` and `relations-30.jsonl`. The name key already holds 200 cases, and a count in a path goes stale whenever a key grows. The lander changes the paths in the design issue's lines 341 and 372 in the same commit.
2. **One file is both the key and the input.** The key line carries `text`, so `recognize --jsonl --field /text` reads the same file `audit` grades against. No second input file can drift from the key.
3. **Offsets are copied from the source.** Local experiment 277's offsets are the key. The check does not re-derive them by forward search, because n065 and n089 each place a name after an earlier copy of the same word. The check proves every slice is the name, which is what `--match strict` needs.
4. **The relation key uses `relate`'s edge shape.** `audit` does not grade `recognize` relations. The edges still take the one shape `audit` can grade. A reader who wants `audit` over them reshapes a run with `jq` into `relate` lines, as proof test 2 does for answers. Making `audit` grade `recognize` relations is a change to a Settled page, so this ticket defers it (deferred gap 2). Design test 7 counts its edges with `jq`, which its own ticket, R5, writes.
5. **The per-kind key is a filter, not ten files.** One `jq` file serves any kind set. Design test 5 needs a set of kinds, and ticket 0147's test 12 needs one kind.
6. **The extra labels are ordinary kinds.** The key keeps `event`, `law`, `language`, `nationality` and `award` as kinds. `kinds.jq` drops them from the key for a run that did not ask for them. A run that asks for them is graded on them. This follows 277's full scoring and loses only its optional-span credit, which deferred gap 5 keeps.
7. **The check runs in lint, and the grading proof runs in spec.** The offset check needs only Python, so it runs on the cheapest rung. The grading proof needs the built binary, so the `spec` rung runs the README with `mustmatch`, as it runs `transforms/README.md`.
8. **The script sits in `sdlc/scripts/`.** It is a gate script, like `tickets` and `pages`, and the scripts README gains its row.
9. **ADR 0054 reaches this ticket lightly.** This ticket asks the model nothing and sets no default. Items 1 to 4 have nothing to act on here. Item 5 asks each default to be measured against the simplest alternative, and this ticket sets none.

## Edge cases

| Input | Expected |
| --- | --- |
| A name with a non-ASCII character before it, such as n002's `Rickenbacker` after `’` or n094's `Zürich` | Offsets count code points. The check fails a byte offset |
| n178, `👩‍🚀 Sally Ride`, a zero-width-joiner emoji before the name | The emoji counts three code points, so `Sally Ride` starts at 4. The check passes |
| n156, `Pink\nFloyd`, a line break inside a name | The name holds the `\n`. The check passes |
| n065, `Apple` twice, the name on the second | Offset 23 is kept from the source. The check passes |
| c24, `Yellow Submarine` twice | Two names at their own offsets, in order |
| A no-names line | `"entities":[]`. `audit` counts it as labeled |
| A run under `person` alone, graded against the whole key | `audit` exits 2 with its key-line sentence |
| The same run, graded against the key filtered to `person` | Graded. A name printed as `person` whose key kind is another counts as extra |
| A run under the five core kinds, graded against the key filtered to them | 347 key names. No extra-label name is in the key, so none is missed |
| That run prints n143's `Wimbledon`, an `event`, as `thing` | Counts as extra. 277 would count it neither way. Deferred gap 5 |
| A run whose kinds include `event` | The filtered key keeps the 10 event names, and they grade like any kind |
| c17 answered `place` for Indica Gallery | Counts as a miss under `--match strict`. The README says so |
| A key line whose names all drop under `kinds.jq` | The line stays, with an empty list. The `person` filter leaves 124 such lines, and the core filter leaves 13 |

## Proof

No test sends a request or reads a key. Tests 1 and 3 need no binary.

| Test | What it runs | Planted fault that turns it red |
| --- | --- | --- |
| 1. The offset check, lint rung | `recognize-keys --self-test`, then `recognize-keys` over both files | Each in its own temporary file: (a) n001's first `start` plus one; (b) a repeated `id`; (c) a `name` changed by one letter; (d) two overlapping names; (e) a relation endpoint whose kind differs from its entity's; (f) n002's `Rickenbacker` given byte offsets, which the `’` before it moves by two. Each exits 1 with its pinned sentence. The committed files pass |
| 2. `audit` grades the keys, spec rung | `mustmatch test specification/fixtures/recognize/README.md`. Its blocks build answer lines from the key with `jq` and pipe them to the built `thinkthen audit --table` | Blocks (g) to (l) below |
| 3. Reach under today's splitter, test rung | The new unit test in `core/recognize.rs` | (m) `split_piece` stops peeling `,`: the counts become 310 and 1,709. (n) n001's `Something` given `end` 33: the count becomes 321, and test 1 fails too |

Every block in test 2 compares whole lines exactly. The `audit` lines are the second line of `--table` output, which starts with two spaces, as `spec/audit.md` pins it. The (i) block turns `set -e` off, pins exit code 2, and pins the whole error sentence.

- (g) Perfect answers under all ten kinds, against `names.jsonl`.
- (h) The same answers with n001's first `start` plus one.
- (i) Person answers against the whole key. Exit code 2, and standard error is the sentence below.
- (j) Person answers against the key filtered to `person`. Then the same answers with n018's `Liverpool`, 3 to 12, added as `person`.
- (j2) Perfect answers under the five core kinds against the key filtered to them. Then the same answers with n143's `Wimbledon`, 10 to 19, added as `thing`.
- (k) The 27 stated edges, reshaped as `relate` answers, against `{id, value: .relations}`. Then the same answers with the five unstated edges added.

```text
(g)   matched 372, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000
(h)   matched 371, extra 1, missed 1: precision 0.997   recall 0.997   f1 0.997
(i) thinkthen: audit: key line 1 names a level, label, or unit the question does not have
(j)   matched 90, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000
(j)   matched 90, extra 1, missed 0: precision 0.989   recall 1.000   f1 0.994
(j2)  matched 347, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000
(j2)  matched 347, extra 1, missed 0: precision 0.997   recall 1.000   f1 0.999
(k)   matched 27, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000
(k)   matched 27, extra 5, missed 0: precision 0.844   recall 1.000   f1 0.915
```

The label in brackets is not part of any line. Each `audit` line starts with two spaces, as printed.

- (l) A counts block pins each file's lines, names and edges, and the names per kind. `names.jsonl`: 200 lines, 372 names, 33 categories, place 96, person 90, organisation 82, work 57, thing 22, event 10, language 6, law 4, award 3, nationality 2. `relations.jsonl`: 30 lines, 73 names, 27 stated edges, 5 unstated.

Each block turns its expected line into a failure by one change, and the build pins each by hand. The debug build named under "What happens today" printed every line above. A scratch test that called today's `tokenize` gave (m)'s and (n)'s counts.

The four questions:

- **What behavior does it protect?** The key stays a key `audit` grades exactly: every offset cuts its name out of its text, every line has the shape `audit` reads, and a filtered key grades a run with fewer kinds. The reach test protects the count R2 improves on.
- **What credible regression fails it?** A hand edit that shifts one offset. A converter that writes byte offsets. A key line in a shape `audit` refuses, such as `label` for `kind`. A kind filter that drops lines and so hides extras. A splitter change that moves word edges without anyone restating the count.
- **Why does no existing test catch it?** No key is in the repository. `spec/audit.md` grades small made-up files, and no test reads the 200 sentences.
- **Does it need a test-only export, flag, or hook?** No. Test 1 reads committed files. Test 2 runs the built command on files. Test 3 calls the production `tokenize` from its own module's tests, which already call it.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `specification/fixtures/recognize/names.jsonl`: 200. `relations.jsonl`: 30. `kinds.jq`: at most 4. `README.md`: at most 150. It copies the 29 rules verbatim, about 48 lines, and gives the 33 categories as one paragraph.
- `sdlc/scripts/recognize-keys`: at most 160, with its self-test.
- `sdlc/scripts/lint`: 3 added. `sdlc/scripts/spec`: 2 added. `sdlc/scripts/README.md`: 1 row.
- `crates/thinkthen/src/core/recognize.rs`: at most 25 added. `sdlc/ratchet.json` moves to the measured total. The commit says the reach test grew it and names the tests it looked at for duplication first.
- The design issue: 3 changed lines. The record: at most 60.
- No dependency. No setting, so `specification/settings.md` does not change.

## Stop rules

1. Stop if the Rust count differs from 322 of 372 or 1,735 words. Report both numbers and the names that differ. Do not change the pin to match.
2. Stop if any sentence of either file matches text in local experiment 267's WNUT-17 sample, or in any other dataset. Exclude it or attribute it, and say which, before committing.
3. Stop if `audit` refuses either converted file for a reason other than proof (i)'s planted one.
4. Stop if the check fails on a committed line. Fix the converter, never the line by hand.
5. Stop before crossing a budget by more than a tenth, or adding a dependency.
6. Stop if any plant stays green.
7. Never run `sdlc/scripts/live`, read `auth.json`, or make a paid call. Unset `THINKTHEN_API_KEY` for every rung.

## Collisions with other work

- **Ticket 0147**, the recognize ADR, is on its branch at `bd50e9a2` and opens no file here. Its test 12 grades a `recognize person` run against the key filtered to `person` with `jq`, as proof (j) does. Its test 5 filters the key to a run's kinds, as `kinds.jq` does.
- **R3** records the first run of the key. Design tests 4, 6 and 9 and ticket 0147's test 12 took their bars from runs over n001 to n100, in local experiments 270 and 274. This ticket changes no bar. R3 owns whether each bar holds over the 200 cases.
- **R2** opens `core/recognize.rs` whole. This ticket adds one test there. If R2 builds first, this ticket merges it and pins the counts under whatever splitter main then holds, and says so.
- **Ticket 0146** and the batching work open none of these files.

## Routing

Builder: Claude in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 1; state and timing 0; reach 1; proof 1; cost of error 1; total 4. Final level: 1. The risk is a key whose offsets or shape are quietly wrong, so every later recognize score is wrong. Tests 1 and 2 guard each.

## Deferred gaps

1. R2 counts reach under its default word rules and with `-` as an infix, over all 372 names. Local experiment 267's 166 and 168 cover n001 to n100 only.
2. `audit` does not grade `recognize` relations. The relation key grades through a `jq` reshape into `relate` lines. A change to `specification/audit.md` would need its own ticket.
3. The key has one kind per name. c17's alternative kind is recorded and not graded.
4. The optional edges of c10, c11 and c12 are recorded. `audit` counts one as extra, so R5 counts them with `jq`.
5. OPT grading. Local experiment 277 counts a core-kind answer at an extra-label span neither way. `audit --match strict` with `kinds.jq` cannot express that, so such an answer counts as extra. At most 25 of 372 names are affected. Grading it needs a change to `specification/audit.md` or a documented `jq` step, in its own ticket.
6. The agreement figure rests on one second annotator of the same model family, over 60 of 200 cases. A second human annotator over all 200 would give a real reliability figure.
7. No recorded run of the key is committed. R3 records the first, by design test 4.

## What Ian can overturn

- The coordinator's ruling to ship local experiment 277's 200-case key now.
- Decision 1: paths without a count.
- Decision 2: one file as key and input.
- Decision 4: relation edges in `relate`'s shape, with `recognize` relations still ungraded by `audit`.
- Decision 5: a `jq` filter in place of per-kind files.
- Decision 6: extra labels as ordinary kinds, with a core-kind answer at an extra-label span counted as extra.
- The five-note rewrite from "Experiment 265" to "Local experiment 265".

## Closes

No issue closes. The lander marks row R1 built in the design issue's ticket table and the backlog's lane 2 row.

## Evidence

- Starts from: local experiment 277's `cases.jsonl` (SHA-256 beginning `c361329c6d5b16f8`, 62,057 bytes), `tools/check_cases.py`, README and REPORT. The README gives the 29 rules, the labels, the optional-span rule and the category table. The REPORT gives the sources, the gap list and the agreement study. Local experiment 267's `cases.jsonl` (SHA-256 beginning `87b49d13e01d6f64`), equal byte for byte to 277's first 100 lines, and its README, whose "Result in one paragraph" gives 141, 166 and 168 of 168 and 772, 818 and 836 words. Local experiment 265's `cases/cases.jsonl` (SHA-256 beginning `3b63bb99b5f6c0cf`, commit `2aea0a7`) and README misses row 11 for the five unstated edges. Sections 10 and 11 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`. The design issue's "Acceptance tests" and R1 row. `specification/audit.md`, "Inputs" and "Names and edges". Ticket 0147 on its branch at `bd50e9a2`, tests 5 and 12. ADR 0054. A trial on 2026-09-26, from this branch at `f04a2cb0`, that converted both keys in a scratch folder, graded them with a debug build's `audit`, and counted reach with a scratch test over today's `tokenize`. It gave every line in proof tests 2 and 3.
- Keeps: every sentence, name, kind, offset, category and note of local experiment 277's key, except five notes' "Experiment 265" wording. Every sentence, name, kind, stated edge and optional edge of local experiment 265. `audit` and `recognize` unchanged. Today's splitter unchanged.
- Changes: `specification/fixtures/recognize/` gains `names.jsonl`, `relations.jsonl`, `kinds.jq` and `README.md`. `sdlc/scripts/recognize-keys` converts and checks them, and lint runs it. The spec rung runs the README. `core/recognize.rs` gains the reach test. The design issue names the new paths and key.
- Proof: Tests 1 to 3 under "Proof", with plants (a) to (n).
- Defers: The counts under R2's rules. `audit` grading `recognize` relations. Alternative kinds and optional edges in `audit`. OPT grading of the extra labels. A human agreement study. A recorded run of the key.
