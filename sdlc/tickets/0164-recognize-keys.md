---
flow: build
priority: 164
opens: specification/fixtures/recognize sdlc/scripts/recognize-keys sdlc/scripts/lint sdlc/scripts/spec sdlc/scripts/README.md crates/thinkthen/src/core/recognize.rs sdlc/ratchet.json sdlc/issues/2026-09-26-recognize-design.md sdlc/records sdlc/tickets
---

# 0164: The recognize keys live in the repository, and audit grades them

Status: ready for review. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A builder or reviewer grades any `recognize` run against the 100-sentence name key with one `audit --match strict` command. The key lives in the repository, every offset in it is checked on every lint, and a no-model check proves that `audit` accepts it. The 30 relation sentences of local experiment 265 live beside it, with their names and their stated edges. A reader filters either key to a run's kinds with one `jq` file, because `audit` refuses a key kind the run did not ask for.

This is row R1 of `sdlc/issues/2026-09-26-recognize-design.md`, "The keys live in the repo and `audit` grades them", and the "recognize R1" row of `sdlc/planning/backlog-0-1-2026-09-26.md` line 102, which says "It can start today". Its only dependency, ticket 0135, has landed. The design's "Acceptance tests" section (line 341) reads: "Ticket R1 copies it to `specification/fixtures/recognize/names-100.jsonl` and writes it as `audit` key lines. Ticket 0135 grades `recognize` with `audit --match strict`, so every scored test below grades with `audit` and no new scoring helper." The row was never ticketed, and Ian asked where the key went.

Design tests 1, 4, 5, 6 and 9, and ticket 0147's test 12, read this key. R2 and R3 cannot build without it.

## What happens today

- Main holds no copy of either key. The 100 sentences sit only in local experiment 267's `cases.jsonl`, 27,902 bytes, SHA-256 beginning `87b49d13e01d6f64`. Each line holds `id`, `text`, `entities` (each `text`, `start`, `end`, `label`), `category` and `note`. Offsets count Unicode code points.
- Local experiment 267's `tools/check_cases.py` checks those offsets. It lives outside the repository, so no rung runs it.
- The 30 relation sentences sit only in local experiment 265's `cases/cases.jsonl`, SHA-256 beginning `3b63bb99b5f6c0cf`, at that experiment's commit `2aea0a7`. Each entity is `[name, kind]` with no offsets. Relations are `[relation, source name, target name]`. Three lines carry `optional_relations`, and c17 carries `alt_labels` (`Indica Gallery` may be `place`). The five unstated edges that design test 7 names are not in that file. Experiment 265's README, misses row 11, names them.
- `audit` reads a `recognize` key line as `{"id": …, "value": {"entities": [{kind, start, end}, …]}}` and ignores other members (`specification/audit.md`, "Inputs"). It refuses a key kind the line's question lacks, at exit 2 with `thinkthen: audit: key line N names a level, label, or unit the question does not have`.
- `audit` does not grade `recognize`'s relations (`specification/audit.md` line 142: "`recognize`'s beta relations are not graded"). It grades `relate` edges `{relation, source: {name, kind}, target: {name, kind}}`.
- Today's splitter is `tokenize` in `crates/thinkthen/src/core/recognize.rs:70`. Its tests pin two short strings.

I checked the shape before writing this ticket, with a debug build from another lane that holds ticket 0135's `audit`. A trial conversion of the 100 sentences, scored by main's `audit` against answers built from the key, printed `matched 168, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000`. The same answers with n001's first `start` moved by one printed `matched 167, extra 1, missed 1: precision 0.994   recall 0.994   f1 0.994`. Answers under kinds `person` alone, graded against the whole key, exited 2 with the key-line sentence above.

## Design

### The files

`specification/fixtures/recognize/` gains four files.

**`names.jsonl`**, one line per sentence of local experiment 267's key, in its order. Each line is an `audit` key line and a `recognize` input record at once:

```json
{"id":"n001","text":"George Harrison's song Something appears on Abbey Road.","category":"possessive","note":"…","value":{"entities":[{"name":"George Harrison","kind":"person","start":0,"end":15},{"name":"Something","kind":"work","start":23,"end":32},{"name":"Abbey Road","kind":"work","start":44,"end":54}]}}
```

- `value.entities` takes the command's own value shape: `name`, `kind`, `start`, `end`, in text order. `label` becomes `kind`, and `text` becomes `name`. The kinds are the key's own: `person`, `place`, `organisation`, `work` and `thing`.
- `text`, `category` and `note` are copied. `audit` ignores them. A run reads the text with `--field /text`, and `audit` finds the record by its default `--id /id`.
- Five notes say "Experiment 265 …". The converter writes "Local experiment 265 …" in those five and changes no other byte of any note.
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

A reader runs `jq -c --argjson kinds '["person"]' -f specification/fixtures/recognize/kinds.jq specification/fixtures/recognize/names.jsonl`. A line whose names all drop stays, with an empty list, so a name printed there still counts as extra. This is the per-kind key that ticket 0147's test 12 and the design's test 5 need. It is a `jq` step, not a scoring helper. No filtered copy is committed.

**`README.md`** says:

- the source of each file: "written by hand for this repository in local experiment 267" and "local experiment 265", with each source's SHA-256 and the date, 2026-09-26;
- the license: every sentence was written for this project, and the files are under the repository's MIT license. No sentence comes from a dataset. Local experiment 267 kept a 25-group WNUT-17 sample, which is CC-BY 4.0, and this ticket copies none of it. Note n066 names WNUT-17 as the source of a known confusion, and copies no WNUT-17 text;
- the convention the name key follows, copied from local experiment 267's eleven rules;
- the kinds of each file, and that `organisation` is spelled as the key spells it;
- how to grade a run, and how to filter by kind, as executable blocks (proof test 2);
- the known key limits: c17's alternative kind, and the optional edges;
- that local experiment 277 is extending this key to about 200 cases with new categories and measured annotator agreement, and that its key will replace `names.jsonl` in place.

### The script

`sdlc/scripts/recognize-keys` is one Python file with three modes.

- `recognize-keys convert names SOURCE` and `recognize-keys convert relations SOURCE` print the converted lines to standard output. The builder runs each once and commits the output. The five unstated edges are a literal table in the script, citing local experiment 265's README, misses row 11. The rung cannot run the conversion, because the sources stay outside the repository. The record gives both commands and both source checksums, so a reviewer with the experiments can rerun them and compare bytes.
- `recognize-keys` with no argument checks both committed files and prints one line each, such as `names.jsonl: 100 lines, 168 names, every offset exact`. It exits 1 on the first file with a problem and names the line and the problem.
- `recognize-keys --self-test` builds each planted fault below in a temporary file and checks that the check refuses it with its pinned sentence. It then checks a clean two-line file passes.

The check reads each file whole and requires, for every line:

- a JSON object with a string `id`, unique in its file, and a nonempty string `text`;
- `value.entities` as a list of objects with exactly `name`, `kind`, `start` and `end`;
- integer offsets with `0 <= start < end <= len(text)`, counted in code points;
- `text[start:end] == name`, and no white space at either end of `name`;
- names in text order, none overlapping the one before;
- a nonempty string `kind`;
- in `relations.jsonl`, every endpoint of `relations`, `optional_relations` and `unstated` equal to the `name` and `kind` of an entity of its own line, and no edge in both `relations` and `unstated`.

The check reads no count from a constant. Any number of lines passes. That is what lets local experiment 277's key replace this one with no change to the script.

### The reach count

`core/recognize.rs`'s test module gains one test. It reads `names.jsonl` with `include_str!`, as `core/batch/tests.rs` reads its fixture. It parses each line with `serde_json`, runs today's `tokenize` on the text, and counts the key names whose `start` is some word's `start` and whose `end` is some word's `end`. It pins two numbers: 141 of 168 names reachable, and 772 words asked. Both match section 11 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`.

The design's other two counts, 166 under the default rules and 168 with `-` as an infix, wait for R2. The rules they count do not exist in the product yet. Computing them now would mean a second copy of R2's splitter in Python, and that copy would pin itself rather than the product. R2 adds both counts to this test, beside the word counts 818 and 836 from local experiment 267. R2 decides whether 141 stays as a count under today's rules written as a word-rules value, or leaves with today's splitter.

## Decisions

1. **The paths are `names.jsonl` and `relations.jsonl`, with no count.** The design names `names-100.jsonl` and `relations-30.jsonl`. Local experiment 277 is growing the name key to about 200 cases. A count in the path would force every reader to change when that key lands. The lander changes the two paths in the design issue's lines 341 and 373 in the same commit.
2. **One file is both the key and the input.** The key line carries `text`, so `recognize --jsonl --field /text` reads the same file `audit` grades against. No second input file can drift from the key.
3. **Offsets are copied from the source.** Local experiment 267's offsets are the key. The check does not re-derive them by forward search, because n065 and n089 each place a name after an earlier copy of the same word. The check proves every slice is the name, which is what `--match strict` needs.
4. **The relation key uses `relate`'s edge shape.** `audit` does not grade `recognize` relations. The edges still take the one shape `audit` can grade. A reader who wants `audit` over them reshapes a run with `jq` into `relate` lines, as proof test 2 does for answers. Making `audit` grade `recognize` relations is a change to a Settled page, so this ticket defers it (deferred gap 2). Design test 7 counts its edges with `jq`, which its own ticket, R5, writes.
5. **The per-kind key is a filter, not five files.** One `jq` file serves any kind set. Design test 5 needs a set of kinds, and ticket 0147's test 12 needs one kind.
6. **The check runs in lint, and the grading proof runs in spec.** The offset check needs only Python, so it runs on the cheapest rung. The grading proof needs the built binary, so the `spec` rung runs the README with `mustmatch`, as it runs `transforms/README.md`.
7. **The script sits in `sdlc/scripts/`.** It is a gate script, like `tickets` and `pages`, and the scripts README gains its row.
8. **ADR 0054 reaches this ticket lightly.** This ticket asks the model nothing and sets no default. Items 1 to 4 have nothing to act on here. Item 5 asks each default to be measured against the simplest alternative, and this ticket sets none.

## Edge cases

| Input | Expected |
| --- | --- |
| A name with a non-ASCII character before it, such as n002's `Rickenbacker` after `’` or n094's `Zürich` | Offsets count code points. The check fails a byte offset |
| n065, `Apple` twice, the name on the second | Offset 23 is kept from the source. The check passes |
| c24, `Yellow Submarine` twice | Two names at their own offsets, in order |
| A no-names line | `"entities":[]`. `audit` counts it as labeled |
| A run under `person` alone, graded against the whole key | `audit` exits 2 with its key-line sentence |
| The same run, graded against the key filtered to `person` | Graded. A name printed as `person` whose key kind is another counts as extra |
| c17 answered `place` for Indica Gallery | Counts as a miss under `--match strict`. The README says so |
| A key line whose names all drop under `kinds.jq` | The line stays, with an empty list |
| Local experiment 277's key replacing `names.jsonl` | The script passes it with no change. The README's counts, the reach test's pins and the executable blocks change in that commit |

## Proof

No test sends a request or reads a key. Tests 1 and 3 need no binary.

| Test | What it runs | Planted fault that turns it red |
| --- | --- | --- |
| 1. The offset check, lint rung | `recognize-keys --self-test`, then `recognize-keys` over both files | Each in its own temporary file: (a) n001's first `start` plus one; (b) a repeated `id`; (c) a `name` changed by one letter; (d) two overlapping names; (e) a relation endpoint whose kind differs from its entity's; (f) n002's `Rickenbacker` given byte offsets, which the `’` before it moves by two. Each exits 1 with its pinned sentence. The committed files pass |
| 2. `audit` grades the keys, spec rung | `mustmatch test specification/fixtures/recognize/README.md`. Its blocks build answer lines from the key with `jq` and pipe them to the built `thinkthen audit` | (g) Perfect answers over `names.jsonl` print `matched 168, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000`. (h) n001's first `start` plus one prints `matched 167, extra 1, missed 1: precision 0.994   recall 0.994   f1 0.994`. (i) Person answers against the whole key exit 2 with `thinkthen: audit: key line 1 names a level, label, or unit the question does not have`. (j) Against the key filtered to `person` they print `matched 46, extra 0, missed 0`. With n018's `Liverpool`, 3 to 12, added as `person`, they print `matched 46, extra 1, missed 0: precision 0.979   recall 1.000   f1 0.989`. (k) The 27 stated edges, reshaped as `relate` answers against `{id, value: .relations}`, print `matched 27, extra 0, missed 0`. The five unstated edges added print `matched 27, extra 5, missed 0: precision 0.844   recall 1.000   f1 0.915`. (l) A counts block pins each file's lines, names and edges, and the names per kind: 100 lines, 168 names, 14 categories, person 46, place 43, organisation 35, work 36, thing 8; 30 lines, 73 names, 27 stated edges, 5 unstated |
| 3. Reach under today's splitter, test rung | The new unit test in `core/recognize.rs` | (m) `split_piece` stops peeling `,`: the counts become 135 and 761. (n) n001's `Something` given `end` 33: the count becomes 140, and test 1 fails too |

Each block in test 2 turns its expected line into a failure by one change, and the build pins each by hand. A trial conversion graded by that debug build's `audit` printed (g) to (j) and (k)'s first line. With one unstated edge added, (k) printed `matched 27, extra 1, missed 0: precision 0.964   recall 1.000   f1 0.982`, so the five-edge line is computed from the same rule. A Python copy of today's splitter gave (m)'s counts.

The four questions:

- **What behavior does it protect?** The key stays a key `audit` grades exactly: every offset cuts its name out of its text, every line has the shape `audit` reads, and a filtered key grades a run with fewer kinds. The reach test protects the count R2 improves on.
- **What credible regression fails it?** A hand edit that shifts one offset. A converter that writes byte offsets. A key line in a shape `audit` refuses, such as `label` for `kind`. A kind filter that drops lines and so hides extras. A splitter change that moves word edges without anyone restating the count.
- **Why does no existing test catch it?** No key is in the repository. `spec/audit.md` grades small made-up files, and no test reads the 100 sentences.
- **Does it need a test-only export, flag, or hook?** No. Test 1 reads committed files. Test 2 runs the built command on files. Test 3 calls the production `tokenize` from its own module's tests, which already call it.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `specification/fixtures/recognize/names.jsonl`: 100. `relations.jsonl`: 30. `kinds.jq`: at most 4. `README.md`: at most 90.
- `sdlc/scripts/recognize-keys`: at most 160, with its self-test.
- `sdlc/scripts/lint`: 3 added. `sdlc/scripts/spec`: 2 added. `sdlc/scripts/README.md`: 1 row.
- `crates/thinkthen/src/core/recognize.rs`: at most 25 added. `sdlc/ratchet.json` moves to the measured total. The commit says the reach test grew it and names the tests it looked at for duplication first.
- The design issue: 2 changed lines. The record: at most 60.
- No dependency. No setting, so `specification/settings.md` does not change.

## Stop rules

1. Stop if the Rust count differs from 141 of 168 or 772 words. Report both numbers and the names that differ. Do not change the pin to match.
2. Stop if any sentence of either file matches text in local experiment 267's WNUT-17 sample, or in any other dataset. Exclude it or attribute it, and say which, before committing.
3. Stop if `audit` refuses either converted file for a reason other than proof (i)'s planted one.
4. Stop if the check fails on a committed line. Fix the converter, never the line by hand.
5. Stop if local experiment 277 lands its key before this ticket is built. Ask the coordinator whether to ship the 100-case key first or build on 277's. The default is to ship the 100-case key now.
6. Stop before crossing a budget by more than a tenth, or adding a dependency.
7. Stop if any plant stays green.
8. Never run `sdlc/scripts/live`, read `auth.json`, or make a paid call. Unset `THINKTHEN_API_KEY` for every rung.

## Collisions with other work

- **Ticket 0147**, the recognize ADR, is on its branch and opens no file here. Its test 12 says "`audit --match strict` over the whole key gives precision and F1" for a run under `recognize person`. That grading exits 2, by proof (i). The key filtered to `person` gives precision, recall and F1 in one row, and a printed `person` name of another kind still counts against precision, by proof (j). The lander tells 0147's owner, who changes the sentence. This ticket does not edit 0147.
- **R2** opens `core/recognize.rs` whole. This ticket adds one test there. If R2 builds first, this ticket merges it and pins the counts under whatever splitter main then holds, and says so.
- **Ticket 0146** and the batching work open none of these files.

## Routing

Builder: Claude in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 1; state and timing 0; reach 1; proof 1; cost of error 1; total 4. Final level: 1. The risk is a key whose offsets or shape are quietly wrong, so every later recognize score is wrong. Tests 1 and 2 guard each.

## Deferred gaps

1. The reach counts 166 and 168, and the word counts 818 and 836, wait for R2's word rules.
2. `audit` does not grade `recognize` relations. The relation key grades through a `jq` reshape into `relate` lines. A change to `specification/audit.md` would need its own ticket.
3. The key has one kind per name. c17's alternative kind is recorded and not graded.
4. The optional edges of c10, c11 and c12 are recorded. `audit` counts one as extra, so R5 counts them with `jq`.
5. Local experiment 277's key of about 200 cases, with annotator agreement, replaces `names.jsonl` in a later ticket or Quick Fix. That change updates the README counts, the reach pins and the test 2 lines.
6. No recorded run of the key is committed. R3 records the first, by design test 4.

## What Ian can overturn

- Decision 1: paths without a count.
- Decision 2: one file as key and input.
- Decision 4: relation edges in `relate`'s shape, with `recognize` relations still ungraded by `audit`.
- Decision 5: a `jq` filter in place of per-kind files.
- The five-note rewrite from "Experiment 265" to "Local experiment 265".

## Closes

No issue closes. The lander marks row R1 built in the design issue's ticket table and the backlog's lane 2 row.

## Evidence

- Starts from: local experiment 267's `cases.jsonl` (SHA-256 beginning `87b49d13e01d6f64`), `tools/check_cases.py`, `tools/reachable.py` and README, whose "Result in one paragraph" gives 141, 166 and 168 of 168 and 772, 818 and 836 words. Local experiment 265's `cases/cases.jsonl` (SHA-256 beginning `3b63bb99b5f6c0cf`, commit `2aea0a7`) and README misses row 11 for the five unstated edges. Sections 10 and 11 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`. The design issue's "Acceptance tests" and R1 row. `specification/audit.md`, "Inputs" and "Names and edges". Ticket 0147 on its branch at `59f62584`, tests 5 and 12. ADR 0054. A trial on 2026-09-26, from main at `914392f5`, that converted both keys in a scratch folder and graded them with a debug build's `audit`, giving the lines in proof test 2.
- Keeps: every sentence, name, kind, offset, category and note of local experiment 267's key, except five notes' "Experiment 265" wording. Every sentence, name, kind, stated edge and optional edge of local experiment 265. `audit` and `recognize` unchanged. Today's splitter unchanged.
- Changes: `specification/fixtures/recognize/` gains `names.jsonl`, `relations.jsonl`, `kinds.jq` and `README.md`. `sdlc/scripts/recognize-keys` converts and checks them, and lint runs it. The spec rung runs the README. `core/recognize.rs` gains the reach test. The design issue names the new paths.
- Proof: Tests 1 to 3 under "Proof", with plants (a) to (n).
- Defers: The counts under R2's rules. `audit` grading `recognize` relations. Alternative kinds and optional edges in `audit`. Local experiment 277's larger key. A recorded run of the key.
