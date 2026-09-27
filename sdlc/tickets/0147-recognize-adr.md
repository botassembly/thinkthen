---
flow: build
priority: 147
opens: sdlc/planning/adr/0056-recognize-is-three-steps.md crates/thinkthen/src/core/recognize.rs crates/thinkthen/src/core/recognize_file.rs crates/thinkthen/src/core/relation crates/thinkthen/src/core/relation.rs crates/thinkthen/src/core/measure crates/thinkthen/src/engine/facade/recognize crates/thinkthen/src/engine/facade/recognize.rs crates/thinkthen/src/public/recognize.rs crates/thinkthen/src/cli crates/thinkthen/tests/backend crates/thinkthen/tests/fixtures conformance libraries databases spec/recognize.md spec/audit.md demos/44-recognize-names specification/recognize.md specification/question-file.md specification/question-file.schema.json specification/result.md specification/channels.md specification/settings.md specification/backends.md specification/audit.md specification/relate.md specification/fixtures/recognize sdlc/issues/2026-09-26-recognize-design.md sdlc/ratchet.json sdlc/records sdlc/tickets CHANGELOG.md
---

# 0147: Build recognize in three steps

Status: built 2026-09-27, awaiting code review. The coordinator accepted it on 2026-09-27 after five rounds of fresh read-only reviews, with step 1 moved to the q3 wording on local experiment 288's evidence. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

`recognize` finds names in three steps, as ADR 0056 decides. Step 1 splits the text into pieces and asks one BILOU question per piece. Step 2 labels each found name and checks its edges in one request. Step 3 asks only the relation pairs a rule allows, about what the text itself states. Each name prints its text, start, end, length, kind and one ranking score, `strength`, and `--threshold` cuts on it. `audit` grades it as `specification/audit.md` says today, with one new rule for runs with no kinds. The same path serves a sentence and a book.

The authority is ADR 0056, accepted 2026-09-26 by Ian's ruling. Ian's words: "I just want to make sure it's doing a reasonably good job, it's simple, and it's the three-step process using BILU labeling and relationships." His earlier rulings stand: "No rule-based systems. Only use the agent to figure out where the boundaries of the starting and ending of entities are." "A simple approach is worth at least 3-5% over a complex, brittle idea. Don't overfit!"

The earlier version of this ticket carried four items Ian accepted on 2026-09-26. This version keeps them. No kinds means every name has the kind `ENTITY`. A bare rule means any kind to any kind, and `ANY` spells `*`. A caller kind can be declined. A relation asks only pairs a rule allows. Its test 12, the key filtered to `person`, stays as test 3 below.

## What happens today

Each line is at `origin/main` `19ca8302`.

- `core/recognize.rs` splits at white space and peels `.`, `!`, `?`, `,`, `:` and `;` from a word's end. Each word gets a yes/no detection question and, with two or more kinds, a kind question. Both show five words around the word. A run of name words becomes one name.
- With one kind, `kind_questions` returns nothing and every name takes that kind at probability one. `recognize person` labels songs and companies `person`.
- With no kinds, the kinds are `person`, `organization` and `place` (`specification/recognize.md` line 5).
- Every request carries the whole text. A text over about 200 words passes the backend's token limit and fails.
- A different-kind relation asks one choice per member of the larger side. At the default cut it keeps at most one edge per asking name.
- Each name prints `name`, `kind`, `start`, `end` and a computed `strength`. `--threshold` cuts on strength. `audit` reads `strength` and the run cut from each recognize line.
- `--relation knows` exits 2 on `recognize`. `ANY` is not a wildcard anywhere.

## Design

### Step 1: boundaries

The splitter makes pieces. White space separates pieces. Each character of Unicode general category P or S is a piece of its own. A run of characters of category Mn, Me or Cf joins the piece that ends right before it, whatever that piece is, so consecutive marks all join. A run after white space or at the text's start begins a piece, as a letter does. Nothing else joins or splits. Offsets count Unicode scalar values.

Each piece gets one pick-one question. The wording is local experiment 279's q3, as local experiments 285 and 288 sent it at six pieces. It is an owner call in ADR 0056 that Ian can overturn. With kinds it reads:

```text
Tokens are split at spaces and at each punctuation mark. Where does the [[ ]] token stand in a name of one of these kinds: KINDS? Other names, ordinary words, dates, numbers, and marks that are not part of a name's own spelling are OUT.

Snippet: SNIPPET
```

`KINDS` is the caller's kind names joined by `, `, with no descriptions. With no kinds, every sentence after the first reads `Where does the [[ ]] token stand in a name? A name is the proper name of a particular person, organisation, place, product, work, event or other thing. Ordinary words, dates, numbers, and marks that are not part of a name's own spelling are OUT.` The spelling `organisation` is the measured text.

`SNIPPET` is the original text from the start of the sixth piece before to the end of the sixth piece after, with the marked piece wrapped in `[[ ]]`. It stops at the text's edges. The options and their descriptions are:

| Option | Description |
| --- | --- |
| `BEGIN` | first token of a name of two or more tokens |
| `INSIDE` | a middle token of a name |
| `END` | last token of a name of two or more tokens |
| `SINGLE` | a one-token name |
| `OUT` | not part of a name |

A step-1 request holds at most 40 consecutive piece questions. Its evidence is the original text from the sixth piece before its first piece to the sixth piece after its last. A text of 40 pieces or fewer therefore sends its whole text once. The requests go out at the engine's width, as ticket 0143 sends one text's parts.

After every step-1 request returns, a Viterbi decode picks the most likely valid tag sequence over the whole text. `OUT`, `SINGLE` and `END` may be followed by `BEGIN`, `SINGLE` or `OUT`. `BEGIN` and `INSIDE` must be followed by `INSIDE` or `END`. The sequence starts with `BEGIN`, `SINGLE` or `OUT` and ends with `OUT`, `SINGLE` or `END`. The score is the sum of log probabilities, with each probability floored at one in a million. Ties go by the table order above. At each step and at the final tag, the earlier tag in the table wins. A `SINGLE` piece is a name. A `BEGIN` through its `END` is a name.

### Step 2: labels and edges

Step 2 sends one request for each step-1 request whose pieces hold the first piece of a found name. Its evidence is the original text from the sixth piece before its first name to the sixth piece after its last. It holds two kinds of question.

- **The kind question.** One per found name, when the run has kinds. The wording is local experiment 278's `none2`, which local experiment 284 kept: `In the text below, some words are wrapped in [[ ]]. Going by what they refer to in this text, which listed kind of name are they? Choose none of these when they are not a proper name, or when they name something that no listed kind covers.` Then `Text: ` and the name's six-piece snippet. The options are the caller's kinds in order, each with the caller's description or none, then `none of these`, described as `They are not a proper name, or no listed kind covers what they name.`
- **The edge question.** One per found name that has two or more options. The options are the name as found, the name plus a touching P or S piece at its right end, the name plus one at its left end, the name less its last piece when that piece is P or S, and the name less its first piece when that piece is P or S. A removal option never leaves an empty stretch, so a one-piece name gets no removal option. "Touching" means no white space between. The wording is local experiment 279's edge wording a: `In the text below, a name was found at the words wrapped in [[ ]]. Each option wraps a slightly different stretch of the text. Pick the option that wraps exactly the whole name. A punctuation mark that is part of the name's own spelling belongs inside it. A mark that belongs to the sentence around the name stays outside.` Then `Text: ` and the name's six-piece snippet. Each option's label is its stretch's text with every run of white space or control characters (Unicode Cc) shown as one space, then a space added until it differs from the labels before it. The option's place still picks the stretch. Its description is the stretch's six-piece snippet, with `...` at an end that stops short of the text's edge.

A name whose kind answer is `none of these` is dropped. Every other name takes its picked stretch and its picked kind. With no kinds, only edge questions go out, and every name takes the kind `ENTITY`. A name with no edge question keeps its span. Names print in order of `start`, then `end`. When the edge pick leaves two names with the same start, end and kind print once, as "Output" says. Beyond the cut under "Output", no other rule touches a name.

### Step 3: stated relations

Each ordered pair of kept names whose kinds match some rule's source and target gets one yes/no question. Under a rule with reading READS it reads `Does the text itself state that i1 READS i2?`. Under `either` it reads `Does the text itself state that i1 READS i2, or that i2 READS i1?`, and the pair is asked once. `*` and `ANY` expand to the kinds of kept names, in first-seen order. Names with equal text and kind are asked once, and an edge names the first. All of a text's pair questions share one request with the whole text as evidence. The existing splitter divides it at ADR 0040's ceiling and at 400 questions. A pair at or above `--relation-threshold`, default 0.5, becomes an edge.

### Output

```json
{"entities":[{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"person","strength":0.97}]}
```

`strength` is P(kind) times P(span), rounded to four decimal places as today. P(kind) is step 2's probability of the chosen kind. P(span) belongs to the stretch step 1 found, before the edge pick, so a widened name keeps its step-1 P(span). It is the summed weight of the valid BILOU paths that tag exactly this stretch as one name, divided by the summed weight of all valid paths. Each tag probability is first floored at one in a million, with no renormalization, and a path's weight is the product of its floored tag probabilities. This is local experiment 287's form, computed by one forward-backward pass in log space. Valid paths follow the decode's transitions. The pass reuses the probabilities the decode already holds, so it asks nothing new. With no kinds, `strength` is P(span). When the edge pick leaves two names with the same start, end and kind, the one with the higher strength prints, and on equal strength the first. `--threshold` keeps a name whose printed four-place strength is at or above the cut, so `audit` rescoring a saved line matches a live run. The default cut is 0.5. The cut stays above 0, as today, and a low cut such as 0.01 keeps nearly every name. A name under the cut leaves before step 3, so no pair names it. `strength` is a ranking score, not a probability, and the pages say so. An edge repeats both names in that shape and keeps `relation` and `probability`. `relations` is absent with no rule, and an empty list when rules gave no edge. Record modes keep `{"input":…,"value":…}`.

`--details` keeps `thinkthen.result/1`. `answer.pieces` lists each piece's offsets and its five tag probabilities. `answer.names` lists each found name's span as found, its kind probabilities and its edge option probabilities, or null with no edge question. `answer.pairs` lists each pair's probability. For `Ada met Acme.` at kinds `person organization` with `--relation knows`, one entry of each reads:

```json
{"start":0,"end":3,"tags":{"BEGIN":0.01,"INSIDE":0.0,"END":0.0,"SINGLE":0.97,"OUT":0.02}}
{"start":8,"end":12,"kinds":{"person":0.02,"organization":0.96,"none of these":0.02},"edges":{"Acme":0.95,"Acme.":0.05}}
{"relation":"knows","source":{"start":0,"end":3},"target":{"start":8,"end":12},"probability":0.91}
```

`--dry-run` prints `thinkthen.recognize-plan/2`: `schema`, `url`, `model`, `key_env`, optional `from`, `pieces`, `request_count`, `name_requests_upper_bound`, the optional relation bounds, and `requests`. `request_count` counts the step-1 requests. `name_requests_upper_bound` equals it, because each step-1 request leads to at most one step-2 request. `requests` lists the step-1 requests as today.

### The guard

A text over 600,000 UTF-8 bytes exits 2 before any request, at every address. The message is `thinkthen: recognize: the text is N bytes, over the limit of M; raise it with --max-text-bytes`. `--max-text-bytes N`, from 1 to 2^53 - 1, sets the limit. It enters no digest. In record mode an oversize record fails that record, as a record over 16 MiB fails today.

### Kinds and rules

A run takes 0 to 20 distinct nonblank kinds, and the count sentence becomes `thinkthen: recognize takes 0 to 20 distinct, nonblank kinds`. `none of these`, `ENTITY` and `ANY` are refused as kinds in any ASCII case, with no text echoed. The refusal follows the existing `RecognizeConfigError` wording: `thinkthen: recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case`, at exit 2 on the command line and exit 5 from a question file. A question file may leave out `recognize.kinds`, and the canonical question writes `"kinds":{}`. `--relation knows` means `knows=*:*` on `recognize` and `relate`. Either side may be `*` or `ANY`. The canonical question writes `*`. A file relation may leave out `source` or `target`, which means `*`. `NAME=KIND` stays malformed. A concrete side naming a kind the run lacks exits 2, as today. `relate` gains only the `ANY` alias and the optional file sides. Its planner stays.

### What goes

The five-word snippets, the detection and kind questions, the kind vote, the `strength` formula, the field name `name`, the three default kinds, and the choice method in `recognize` relations. Every recognize question digest and request body changes.

### `audit`

`audit` grades `recognize` as `specification/audit.md` says today. It reads each name's `strength` and the run cut from `question.threshold`. It prints counts, precision, recall, F1, the suggested bar and the crossed line. Calibration, AUC and the coverage curve stay null, because `strength` is a ranking score and not a probability (`audit.md`, "Names and edges"). It accepts `--threshold` at or above the run cut. `--write` puts a steady bar into the recognize question file's top-level `threshold`. `relate` grading is unchanged.

One rule is new. A recognize line whose `question.kinds` is empty grades every said name and every key name as the kind `ENTITY`. Today such a line refuses any key name at exit 2 with `thinkthen: audit: key line N names a level, label, or unit the question does not have`, because the question lacks the key's kinds.

### Surfaces

The public Rust type keeps one entity with `text`, `start`, `end`, `length`, `kind` and `strength`. Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL and SQLite return those six fields in that order, with each surface's existing offset unit. `text` replaces `name`, and `length` is new. `strength` keeps its name and type on every surface. The shared conformance cases carry the new shape.

## Changed declarations

Normative after rustfmt, under the same rules as the 0084 block. `RecognizedEntity::name` is removed, and these replace it.

```rust
impl RecognizedEntity {
    pub fn text(&self) -> &str;
    pub fn length(&self) -> usize;
}
```

## The design issue's rows

- R0, the ADR of Ian's rulings, is replaced by ADR 0056.
- R1, the keys and their grading, is built by ticket 0164 and stays as it landed.
- R2, R3 and R4 retire. Word rules, `confirm` and windowed pieces with `--window` are not built.
- R4b, `recognize --jobs` on one text, stays and builds on ADR 0056.
- R5, stated relations, is built by ticket 0147.
- R6 retires. ADR 0056 removes `keep`, `infixes` and `boundary`, and ticket 0147 carries `--max-text-bytes` itself.
- R7, batching texts, stays and builds on ADR 0056.
- R8, the manual page, stays and builds on ADR 0056.

## Decisions

The owner's calls. Ian can overturn each.

1. **One ticket builds the whole path.** The three steps share one request plan and one output shape. Splitting them would ship a surface that changes twice.
2. **Each name keeps one ranking score and a cut.** Ian ruled on 2026-09-26: "explain? we lose calibration to trade FN and FP?" and "audit and diff should be fully supported". A score lets a caller trade missed names against false ones, and it lets `audit` suggest the cut. Local experiment 287 chose P(kind) times P(span) and the 0.5 cut offline. At 0.5 against no cut it scored 78.6 to 81.0 on the full public split and 53.4 to 60.1 on WNUT-17, both with p1 on the whole sentence. It scored 83.1 to 84.3 on the long documents with p1 at six pieces, and 87.3 to 87.2 on the key with p1 on the whole sentence. With the shipped q3 wording at six pieces and the cut, local experiment 288 measured 80.1 on the full public split and 59.1 on WNUT-17. Today's form, the lowest tag probability times the kind probability, is better calibrated on the public sets: expected calibration error 3.1 against 12.1 on the full public split and 11.2 against 19.5 on WNUT-17. It also had higher average precision on the key, 83.2 against 82.3. At 0.5 it scored 84.0 on the key and 79.8 on the long documents, against 87.2 and 84.3 for the chosen score, 3.2 and 4.5 points lower. It had lower average precision on every public set. The choice rests on F1 at the 0.5 cut and on public-set average precision. P(kind) alone trailed by 1.4 to 4.1 F1 at 0.5 on the public sets and the long documents. On the key it scored 87.7 against 87.2. `relation_threshold` keeps its 0.5 default, which local experiment 278 measured.
3. **The fields are `text` and `strength`.** The coordinator set `text`, and the Python frame already names that column `text`. `audit` keys keep `name`, because `audit` compares offsets and kind only. The score keeps the name `strength`, because local experiment 287 found it ranks well but is not calibrated across kinds of text: expected calibration error 6.7 to 19.5. `result.md` keeps its sentence that `strength` is not itself a probability. The libraries, SQL and `audit` keep their field.
4. **Step 2 groups by step-1 request.** Local experiment 285 sent one step-2 request a name. Grouping sends fewer requests and keeps the six-piece snippets. On a short text it equals the one request a text that local experiments 278, 279 and 283 measured. Test 5 records one long text under it.
5. **The edge question goes with the kind question.** Local experiment 279 asked it after step 2, on kept names only. Asking both at once saves a round trip. A declined name's edge answer is ignored.
6. **The guard is `--max-text-bytes`, command line only.** A library or SQL caller gets the default. A later ticket may carry it further when a demo needs it.
7. **Overlap after an edge pick stays.** A widened name may overlap a neighbour by one mark, as `Help!` over a stray `!` name did in local experiment 283. Resolving it would be a rule.
8. **Plan schema version 2.** The key set changes, so `thinkthen.recognize-plan/1` becomes `/2`.
9. **Edge labels show white space and control characters as one space.** A name that spans a line break, a tab or an escape byte would put a control character in its label, and labels refuse control characters. The coordinator ruled on 2026-09-27 that each label shows every white-space run as one space, then widened the rule the same day to every run of white space or control characters (Unicode Cc), so no label can fail after step 1 is paid. The description keeps the real snippet. A step-2 label error is a defect, never a kinds error.
10. **`relate` reads what `recognize` found.** The rename from `name` to `text` broke the handoff from `recognize` to `relate`. The coordinator ruled on 2026-09-27 that `relate` takes an entity carrying `text` in place of `name` and uses `text` as the name. `name` wins when both are present. The command applies this only at the default `/name` field. Python, Ruby, TypeScript, R and C read a found name the same way. One test on each surface feeds `recognize` output into `relate`.

## Choices filled from experiments

1. **The step-2 wording stays `none2` with one way out.** Local experiment 284 rejected a broader wording. Its held-out F1 fell at three and five kinds, and it added 3.7 and 9.7 extras a run there. It gained at ten kinds, 93.4 against 92.5, but the gain sat in one sentence about drinks. It rejected two way-outs, which fell 2.0 to 4.1 held-out points by kind set and broke 19 to 34 real names. Caller descriptions pass into step 2 only when supplied. At five kinds they lifted held-out F1 from 92.2 to 94.0 in all three runs, fixing 13 names and breaking 2, and tied on public text. They failed the keep rule at three and ten kinds. The key's descriptions are its own label lines, so part of the lift restates the key.
2. **No hidden kinds.** `web address`, `email address` and `social handle` are ordinary kinds a caller may ask for, with no special handling. A run that does not ask for them does not offer them. Local experiment 286 found that hidden decoys in step 2 raised the full public split from 78.6 to 79.7 and WNUT-17 from 53.4 to 55.2, with p1 on the whole sentence and no cut, and dropped 127 real WNUT-17 names. Asked for directly, the kinds were right 92% to 100% of the time. Whole addresses came back only 52% to 83% of the time, because pieces split at marks.
3. **The score is `strength`, P(kind) times P(span), with a default cut of 0.5.** "Output" gives the definition, and decisions 2 and 3 give the evidence from local experiment 287.

## Edge cases

| Case | Expected |
| --- | --- |
| Empty or blank text | Exit 2, `thinkthen: the evidence is empty or blank`, no request |
| No names | `{"entities":[]}`, one step-1 request, no step-2 request |
| `George Harrison's` | Pieces `George`, `Harrison`, `'`, `s`. The model decides the edge |
| `U.S.` at a sentence end | Pieces `U`, `.`, `S`, `.`. When step 1 finds `U.S`, the edge question also offers `U.S.` |
| `Help!` | When step 1 finds `Help`, the edge question also offers `Help!` |
| `London-based` | `London` is its own piece and reachable |
| `Drakeʼs` with U+02BC | One piece. Accepted limit |
| `Dame Judi Dench` | Kept whole with its title. Accepted limit |
| `Paul John` | One name. Accepted limit |
| `é` written as `e` plus U+0301 | U+0301 joins the word piece. One piece |
| `e` plus U+0301 plus U+0302 | Both marks join the word piece. One piece |
| `♥` plus U+FE0F | U+FE0F is category Mn and joins the symbol piece. One piece |
| `.` plus U+0301 | U+0301 joins the punctuation piece. One piece |
| `👨` U+200D `👩` | U+200D is category Cf and joins `👨`. Pieces `👨` U+200D and `👩` |
| U+0301 after white space, then `a` | The mark begins a piece, and `a` continues it. One piece |
| `👍🏽` | Two pieces of one scalar each. Both characters are category So or Sk |
| `Hi "!" ok` with the one-mark name `!` | Edge options `!`, `!"` and `"!`, in that order. No removal option |
| A name across two step-1 requests | One name. The decode runs after every step-1 request |
| 40 pieces | One step-1 request with the whole text |
| 41 pieces | Two step-1 requests. The Requests table gives the offsets |
| One step-1 or step-2 request fails | The text fails at exit 4. No partial names print |
| Text of 600,000 bytes | Planned |
| Text of 600,001 bytes | Exit 2, zero sends |
| `--max-text-bytes 700000` on 600,001 bytes | Planned |
| `--max-text-bytes 0` | Usage error, exit 2 |
| No kinds | Kind `ENTITY`. Edge questions only in step 2 |
| Kinds `ENTITY`, `any` or `None Of These` on the command line | Exit 2, `thinkthen: recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case`, zero sends |
| The kind `entity` in a question file | Exit 5, the same sentence, zero sends |
| `Ada met Bob` with no kinds | No kind question, and no name touches a mark, so no edge question. The step-2 request has no questions and is not sent |
| `--relation knows` with no kinds | `ENTITY` to `ENTITY` pairs |
| `--relation knows=ANY:organization` | Same plan and digest as `'knows=*:organization'` |
| `--relation works_for=person` | Malformed, exit 2 |
| A name declined as `none of these` | Never expands a wildcard or enters a pair |
| Twenty kinds under a profile with `max_options` 20 | Exit 2 before any request, naming 21 options and the limit |
| `recognize --threshold 0.9` | Keeps only names whose `strength` is at least 0.9 |
| `recognize --threshold 0.01` | Nearly every name the decode and step 2 keep |
| `recognize --threshold 0` | Exit 2, as today: the cut must be above 0 |
| A question file with no `recognize.kinds` | Canonical `"kinds":{}`. The digest is pinned in test 7 |
| `https://t.co/x` with the kind `web address` | Pieces split at `:`, `/` and `.`, so the whole address may not come back. Accepted limit |
| A name under the cut | Not printed, and no pair names it |

## Proof

Every scored test replays a recording and grades it with `audit --match strict` against `specification/fixtures/recognize/`. Test 4 alone passes its replayed lines through a named `jq` step first. For tests 1, 3 and 5, `kinds.jq` filters the names key to the run's kinds. Test 2 grades under the `ENTITY` rule. Test 4 grades against the two relation keys it names. The bars sit under the lowest measured run, so a pass is stable and a fail is beyond the noise. Every run grades at the default cut of 0.5. `sdlc/records/2026-09-26-recognize-three-step-evidence.md` copies every figure this ticket cites. The key's reference figures for tests 1 to 3 come from local experiment 279 part 3a, which ran the shipped q3 wording at six pieces with step 2 and the edge question, and no cut. Local experiment 287 measured the key with p1 on the whole sentence at 87.2 at the 0.5 cut and 87.3 with no cut, so the cut barely moves the key. Key sentences average 7.4 words, so a six-piece window shows most of each one.

### Outside-in tests

1. **The key at five kinds, replayed.** `person place organisation work thing` over all 200 key lines. Bar: F1 of at least 82. With q3 at six pieces, local experiment 279 measured 88.8 to 90.5 on the first half and 83.2 to 83.9 on the second.
2. **The key with no kinds, replayed.** Graded by `audit` under the new rule, which grades every name as `ENTITY`. No `jq` mapping runs. Bar: F1 of at least 86. With q3 at six pieces, local experiment 279 measured 89.8 to 90.9 and 87.7 to 88.4.
3. **The key filtered to `person`, replayed.** `recognize person`. A printed name of any other kind counts against precision. Bar: F1 of at least 70. With q3 at six pieces, local experiment 279 measured 92.6 in every run and 75.5 to 78.4. Main scored 27.0 at `person` on the first half (local experiment 274).
4. **Relations, replayed.** The 30 relation sentences at kinds `person song album place organisation`, as local experiment 278 ran them, so its figure is a same-setup reference. `relations.jsonl` holds 4 `organisation` names. The four rules are local experiment 278's: `sang=person:song`, `wrote=person:song`, `appears_on=song:album` and `recorded_at=album:place`. A recognize edge prints as:

   ```json
   {"relation":"sang","source":{"text":"Paul McCartney","start":0,"end":14,"length":14,"kind":"person","strength":0.9712},"target":{"text":"Yesterday","start":20,"end":29,"length":9,"kind":"song","strength":0.9431},"probability":0.97}
   ```

   A named `jq` step, `specification/fixtures/recognize/edges-as-relate.jq`, turns each replayed line into relate's shape. Then `audit --match strict` grades it as `relate` against two keys built from the fixture's relation lines. The program is:

   ```jq
   .question |= (.verb = "relate" | .threshold = .relation_threshold)
   | .value = [.value.relations[] | {relation, source: {name: .source.text, kind: .source.kind}, target: {name: .target.text, kind: .target.kind}, probability}]
   ```

   The stated key comes from `jq -c '{id, value: .relations}' relations.jsonl`. The unstated key comes from `jq -c '{id, value: [.unstated[]?]}' relations.jsonl`. It holds the five unstated edges, on `c01`, `c02`, `c05`, `c06` and `c14`. Each grade prints its row with `thinkthen audit ... --table | sed -n 2p`, in the form the fixture README pins. The stated row reads `  matched M, extra X, missed Y: precision P   recall R   f1 F`, and M is at least 19 of the 27 stated edges. Local experiment 278 found 22 in each run with p1 on the whole sentence. No experiment ran these sentences with q3, so the recording run sets the first q3 figure.

   The bar was 21. The recorded run found 20, and stop rule 2 stopped the build. The coordinator ruled on 2026-09-27 to set the bar at 19, one under the recorded 20, to leave room for one run's noise on a re-record. No wording was tuned. The seven misses by cause:

   - Four are `Help!`, on c06, c07, c09 and c23. Step 1 ends the name before its `!`, and the edge pick does not add it back. Each prints a `Help` edge that counts as extra. This is the accepted `Help!` limit under "Deferred gaps".
   - Three are step 3 saying no, on c08, c16 and c29.

   The other two extras are the optional edges of c10, `wrote` at 0.51, and c12, `appears_on` at 0.77. In c23, step 1 also found a stray `"` as a song. The bar of 19 is a coordinator call Ian can overturn. The unstated row reads exactly `  matched 0, extra E, missed 5: precision 0.000   recall 0.000   f1 0.000`, where E is the number of edges the run kept. Each kept edge counts as extra against that key, so no unstated edge passes.
5. **A long text, replayed.** Local experiment 279's invented 1,018-word text and its 65-name key join `specification/fixtures/recognize/` as `long.jsonl`. The text splits into 1,183 pieces, measured with this splitter. The dry run therefore shows 1,183 divided by 40, rounded up, which is 30 step-1 requests of at most 40 pieces, and no request carries the whole text. The replay at five kinds scores F1 of at least 88. Local experiment 279 measured 92.9 to 94.5 with the q3 wording. That run's step-1 requests showed six pieces each side. Its step-2 and edge questions showed 30 pieces each side. The reference figure therefore used a wider step-2 and edge window than this ticket's six pieces. Input tokens a word stay under 450. With q3 at six pieces, local experiment 285 measured 322 on public documents and local experiment 288 measured 308 on the full public sets.
6. **Loopback plans and counts.** A listener counts requests and keeps their bodies. `Ada met Acme.` at kinds `person organization` sends one step-1 request, then one step-2 request. No kinds sends no kind question. A text with no names sends no step-2 request. A step-2 request with no questions is not sent: `Ada met Bob` with no kinds sends one step-1 request only. With `--kind person=DESC --kind organization=ORGDESC`, `DESC` appears in the step-2 `person` option and nowhere in the step-1 body. A failed second request fails the text. A text of 600,000 bytes plans under `--dry-run`. A text of 600,001 bytes sends zero.
7. **Rules with no kind limits.** Dry runs of `knows`, `'knows=*:*'` and `knows=ANY:ANY` on `recognize person organization` print one plan and digest. A dry run with no kinds prints the canonical question with `"kinds":{}`, and the test pins its digest. A loopback run finds three names and declines one. Its pair request holds 2 questions, the two ordered pairs of the kept names under `knows=*:*`, and asks no pair naming the declined name. `relate` gives one plan for the same three spellings.
8. **Audit and write, replayed.** `audit --match strict` over test 1's replayed run prints counts, precision, recall, F1, a suggested bar and a crossed line, and null calibration, AUC and coverage curve. The test pins that row exactly. Over test 2's replayed run it grades with the `ENTITY` rule and does not print the old refusal. `--threshold` at the run cut and above it grades. `--write` puts the steady bar into a copy of the run's recognize question file, and a rerun's digest matches it.
9. **Secrecy.** A secret in the text and in a kind description never reaches standard output, standard error or any `Debug` line, on every new failure path: the guard, a failed step-2 request, a reserved kind. The test pins the reserved-kind sentence at exit 2 on the command line and exit 5 from a file.

Each answers the four questions. They protect the three steps' output on real recordings, `audit`'s full support, the request plan, the rule parser and secrecy. Restoring the old splitter, the old questions, the whole text as evidence, the choice method, or a wildcard over a declined kind fails at least one. No existing test pins any of these, because main's tests pin the old method. None needs a test-only hook. They drive the real command against recordings and a loopback listener.

### Edge-case tables

Five table tests in `core`, each a list of inputs and exact outputs:

- **Pieces.** Every splitter row of "Edge cases", with each piece's scalar offsets.
- **Decode.** Tag probabilities and the names they give: a lone `BEGIN` at the end, `BEGIN` then `OUT`, equal scores, a name across a request edge, and a floor row. In the equal-scores row the one-piece text `Ada` has `SINGLE` 0.5, `OUT` 0.5 and 0 for the other tags. Both paths score the same, `SINGLE` comes before `OUT` in the table, and the row yields one name, `Ada`. In the floor row one path holds a zero beside high probabilities and the other holds only small nonzero ones. The first path wins only with the floor.
- **Score.** Tag and kind probabilities, the exact `strength` they give under local experiment 287's form, and whether the default cut keeps or drops the name. A worked row: a one-piece text with `SINGLE` 0.9, `OUT` 0.1 and 0 for the other tags has two valid paths, so P(span) is 0.9 over 1.0, and a kind probability of 0.8 gives 0.72. The other rows: one-piece and three-piece names, a name whose decoded path is not the only likely one, a name with no kinds, a name at 0.5 and one at 0.49, a raw product of 0.49996 that prints 0.5 and is kept, a raw product of 0.49994 that prints 0.4999 and is dropped, a name the edge pick widened that keeps its step-1 P(span), and two names the edge pick leaves with the same start, end and kind, where the higher strength stays.
- **Edge options.** Each case's option stretches: marks at both ends, a one-piece name, the one-mark name `!`, and two equal labels. A name across a line feed, `"Maria\nChen."`, gives the labels `"Maria Chen"` and `"Maria Chen."`. A name with an escape byte inside a word, `"Mar\u{1b}ia Chen."`, gives `"Mar ia Chen"` and `"Mar ia Chen."`.
- **Requests.** The text is 41 one-letter words `a` joined by single spaces, 81 scalars long, so piece k spans `[2k-2, 2k-1)`. The first 40 words give the 40-piece text, 79 scalars long. The table pins each request's pieces and evidence offsets, and the step-2 grouping:

| Input | Step-1 requests | Step-2 requests |
| --- | --- | --- |
| 40 pieces, a name at piece 3 | One: pieces 1 to 40, evidence `[0,79)` | One: evidence `[0,17)`, from the text's start to the end of piece 9 |
| 41 pieces, names at pieces 3 and 41 | Two: pieces 1 to 40 with evidence `[0,81)`; piece 41 with evidence `[68,81)`, from the start of piece 35 | Two: evidence `[0,17)` for the name at piece 3; `[68,81)` for the name at piece 41 |
| 41 pieces, one name over pieces 40 and 41 | As above | One, grouped with the first request, because the name's first piece sits there: evidence `[66,81)`, from the start of piece 34 |

### Deliberate breaks

Each break is made, run and reverted. Each must turn the named row red.

| Break | Row that turns red |
| --- | --- |
| Split U+02BC as punctuation | Pieces: `Drakeʼs` |
| Let `BEGIN` be followed by `OUT` | Decode: `BEGIN` then `OUT` |
| Drop the probability floor | Decode: the floor row |
| Offsets in bytes | Pieces: `👍🏽` |
| Drop `none of these` | Test 3 |
| Ask the kind question with no kinds | Test 6: no kinds |
| Send the edge question in its own request | Test 6: `Ada met Acme.` request count |
| Send the whole text as step-1 evidence | Test 5: the dry run |
| Refuse at 600,000 bytes | Test 6: the text of 600,000 bytes plans |
| Let a declined name enter the pair list | Test 7: the pair request holds 6 questions, not 2 |
| Read `ANY` as a concrete kind | Test 7 |
| Print the text in the guard's message | Test 9 |
| Drop `strength` from a name | Test 8 |
| Score with the lowest tag probability, today's form | Score: the name whose decoded path is not the only likely one |
| Keep a name under the cut | Score: the name at 0.49, marked dropped |
| Cut on the raw product, not the printed strength | Score: the raw product of 0.49996 |
| Compute P(span) over the widened stretch | Score: the widened name |
| Drop the `ENTITY` rule in `audit` | Test 2 |
| Put descriptions in step 1 | Test 6: the description check |
| Drop descriptions from step 2 | Test 6: the description check |
| Chunk at 41 pieces | Requests: the 41-piece rows |
| Use a five-piece window | Requests: evidence `[68,81)` |

## Recordings

Every recognize request changes, so every recognize recording is made again. Each run below goes through `sdlc/scripts/live` under a token cap, and Ian authorizes it before it starts. Costs use $0.042 a million input tokens and the q3 tokens a word of local experiments 279 and 285.

| Run | Words | Estimated cost |
| --- | --- | --- |
| Test 1, the key at five kinds | 1,489 | about $0.022 |
| Test 2, the key with no kinds | 1,489 | about $0.018 |
| Test 3, the key at `person` | 1,489 | about $0.021 |
| Test 4, the relation sentences at five kinds with rules | about 300 | about $0.005 |
| Test 5, the long text at five kinds | 1,018 | about $0.014 |
| Demo 44, `spec/recognize.md`, conformance cases 41 to 50 | about 200 | about $0.003 |

The total is about $0.08. The 40 harvest cases in `tests/fixtures/recognize-225` and the relation fixtures in `recognize-239` go, and the key recordings replace them. A test that still needs one of them fails loudly, and the builder moves it to a key recording.

## Pages

In the commit that changes each behavior:

- `specification/recognize.md`: rewritten around the three steps, the window, the output, the guard and the kinds rules.
- `specification/question-file.md` and its schema: optional `recognize.kinds`, optional relation sides, `ANY`, the reserved kinds, and the canonical order, unchanged in its keys: `verb`, `kinds`, optional `relations`, `threshold`, `relation_threshold`, optional `profile`.
- `specification/result.md`: the new `--details` members. Its sentence that `strength` is not itself a probability stays, with the new formula.
- `specification/audit.md`: a recognize line with an empty kind set grades every said and key name as the kind `ENTITY`.
- `sdlc/records/2026-09-26-recognize-three-step-evidence.md`: the tables this ticket and ADR 0056 cite, from local experiments 278 to 288. It was filed with this design.
- `specification/channels.md`: `--max-text-bytes`, and plan schema version 2.
- `specification/settings.md`: the Kinds row default becomes none. The guard moves into the table. The `keep`, `infixes`, `prefixes`, `boundary` and `window` lines go.
- `specification/backends.md`: step-1 and step-2 requests carry a window. Relation requests carry the whole text.
- `specification/relate.md`: the `ANY` alias and optional file sides.
- `specification/fixtures/recognize/README.md`: `long.jsonl` and its source.
- `spec/recognize.md` and demo 44: the new plan and output. Demo 44 keeps its strength step.
- `sdlc/issues/2026-09-26-recognize-design.md`: already marked superseded by ADR 0056 where it rejects BILOU. Each of its rows R0 to R8 gains its marker from the list under "The design issue's rows".
- `CHANGELOG.md`: one line naming the output change, the removed options and the new guard.
- Two issues for the marketing lead name the site's recognize and relate recordings and examples, because `site/` changes hands only through them.

## Ratchet

Main's ceiling was 70,310 lines on 2026-09-27. The build reads it when it starts. The estimate is +430 against it, about 70,740, from 70,590 to 71,040.

- Grows: the splitter, the step-1 questions and chunking (about 70 lines), the decode (about 45), the forward-backward score (about 30), the step-2 kind and edge questions (about 110), the three-round facade (about 60), the guard and the rule parser (about 40), and the tests: five tables and nine outside-in tests (about 500).
- Shrinks: the old splitter, the five-word window, the detection and kind questions, the kind vote and the strength formula (about 230), and the old recognize tests that pin them (about 200).
- Before raising the ceiling, the builder looks for duplication in `core/relation.rs`, whose yes/no pair path step 3 reuses, and in the old recognize facade.

## Stop rules

1. Stop if the ceiling would pass 900 over the ceiling when the build starts. Report what grew.
2. Stop if a recorded run misses its bar. Report the scores to the coordinator with the error groups. Do not tune the wording to the key.
3. Stop before any paid run that Ian has not authorized.
4. Stop if a design rule here needs a ruling ADR 0056 does not give. Report it with options.
5. Stop if a deliberate break stays green.
6. Stop if `Labels` refuses an edge option label that the key produces. Report the case, and do not invent a label rule.
7. Stop if another in-flight ticket edits the same recognize files. The coordinator orders the two.

## Routing

Owner and builder: Claude. Reviewer: a fresh read-only Claude session for the design and for the diff. The change raises the ceiling and changes public output, so the diff review names what it checked.

## Complexity

Contract 3; state and timing 1; reach 3; proof 2; cost of error 2; total 11. The risk is a shape change across every surface. The conformance cases and the recorded bars guard it.

## Deferred gaps

- **`diff`.** `diff` reads neither `recognize` nor `relate` today. Ticket 0165 adds both after this ticket fixes the output shape.
- **Public benchmark figures.** ADR 0056 records local experiment 288's figures for the shipped setup: 80.1 on the full public split against the rule-based baseline's 77.7, and 59.1 on WNUT-17. The recognize page states them with R8. Each ran once, so their run-to-run spread is unmeasured.
- **Long texts.** ADR 0056's window replaces R4's pieces and `--window`. `recognize --jobs` on one text stays R4b.
- **The `either` wording.** No experiment measured the step-3 wording under `either`. Test 4 uses no `either` rule.
- **Relation evidence.** Step 3 keeps the whole text as evidence, so a long text with rules can pass the backend's evidence limit and exit 4.
- **Standalone `relate`.** Its choice planner stays until its own ticket moves it to pairs under Ian's ruling of 2026-09-26.
- **Batching texts.** Record modes send each text alone, as today, until R7.
- **The shipped combination.** Local experiment 288 measured the shipped wording, window, decode, step 2 and cut on the full public sets. It sent one step-2 request a name and asked the edge question after the cut. No experiment measured step 2 grouped per step-1 request with the edge question in the same request. The recording runs for tests 1 to 5 measure the whole combination on the repository's own key, and their bars stop the build if it falls short.
- **The accepted limits.** Titles inside names, two first names side by side, weekday names with no kinds, kinds that depend on use, pieces of web addresses and handles on web text, whole web addresses under the web kinds, and `Help!` in relation texts. `strength` is not calibrated across kinds of text.

## What Ian can overturn

- ADR 0056 as a whole, and each owner call it lists.
- Decisions 1 to 10, and the coordinator's calls of 2026-09-27 behind decisions 9 and 10.
- The choices filled from local experiments 284, 286, 287 and 288.
- The bars of tests 1 to 5, and the coordinator's call of 2026-09-27 that set test 4's bar at 19 of 27 stated edges.
- The four kinds items carried from the earlier version: `ENTITY`, bare rules and `ANY`, declining through `none of these`, and pairs only where a rule allows.

## Closes

None. The design issue stays open for R4b, R7 and R8. `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md` stays open. This ticket answers its item 1. Its item 2 stays open for standalone `relate`.

## Evidence

- Starts from: ADR 0056 and Ian's rulings of 2026-09-26. `sdlc/records/2026-09-26-recognize-three-step-evidence.md`, which copies the cited tables. Local experiments 278 (three steps against the baseline), 279 (the edge question, descriptions, and the rejected title, split and trim arms), 280 and 281 (the public sample, the full public split, WNUT-17, and q3 at six pieces against p1 on the whole sentence), 282 (error groups and plan rules), 283 (the rejected span check), 284 (step 2 on the key's own names), 285 (windows on public documents), 286 (web kinds), 287 (the name score) and 288 (q3 against p1 at six pieces on the full public sets). Local experiment 274 for the one-kind failure. The earlier version of this ticket for the four kinds items. `origin/main` at `19ca8302`.
- Keeps: Unicode scalar offsets with an exclusive end. Record modes and their envelope. `--kind KIND=DESCRIPTION`. The `strength` field at four places, `--threshold` above 0 on names, and `audit`'s recognize grading as `audit.md` states it, `--write` included. Relation rules, `--relation-threshold` and its 0.5 default. `relate`'s planner. The request ceiling of ADR 0040. The `audit` key files.
- Changes: The splitter, all recognize questions, the decode, windows at every length, the output shape, `--details`, the dry-run schema, the guard, the kinds and rule parsing, the `strength` formula, and every surface's entity type. The default kinds go.
- Proof: Five replayed recordings graded by `audit` against bars under the lowest measured run. One replayed test of `audit`, its `ENTITY` rule and `--write`. Three loopback tests, five edge-case tables, and twenty-two deliberate breaks with the row each turns red.
- Defers: `diff` over recognize and relate to ticket 0165. Public benchmark figures to R8. `--jobs` on one text to R4b. Batching to R7. Windowed relation evidence. Standalone `relate`'s pair method. The accepted limits.
