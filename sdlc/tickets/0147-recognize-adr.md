---
flow: build
priority: 147
opens: sdlc/planning/adr/0056-recognize-is-three-steps.md crates/thinkthen/src/core/recognize.rs crates/thinkthen/src/core/recognize_file.rs crates/thinkthen/src/core/relation crates/thinkthen/src/core/relation.rs crates/thinkthen/src/core/measure crates/thinkthen/src/engine/facade/recognize crates/thinkthen/src/engine/facade/recognize.rs crates/thinkthen/src/public/recognize.rs crates/thinkthen/src/cli crates/thinkthen/tests/backend crates/thinkthen/tests/fixtures conformance libraries databases spec/recognize.md spec/audit.md demos/44-recognize-names specification/recognize.md specification/question-file.md specification/question-file.schema.json specification/result.md specification/channels.md specification/settings.md specification/backends.md specification/audit.md specification/relate.md specification/fixtures/recognize sdlc/issues/2026-09-26-recognize-design.md sdlc/ratchet.json sdlc/records sdlc/tickets CHANGELOG.md
---

# 0147: Build recognize in three steps

Status: draft. It becomes ready for review when the coordinator fills open choices 1 and 2 from local experiments 284 and 286. This version replaces the whole earlier ticket, which recorded the design issue's word rules in an ADR numbered 0050. That ADR was never written. Ian's rulings of 2026-09-26 replaced its method, and ADR 0056 now holds the design. A fresh read-only reviewer accepts this ticket before it is built. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

`recognize` finds names in three steps, as ADR 0056 decides. Step 1 splits the text into pieces and asks one BILOU question per piece. Step 2 labels each found name and checks its edges in one request. Step 3 asks only the relation pairs a rule allows, about what the text itself states. Each name prints its text, start, end, length and kind. The same path serves a sentence and a book.

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

The splitter makes pieces. White space separates pieces. Each character of Unicode general category P or S is a piece of its own. A character of category Mn, Me or Cf right after a symbol piece joins that piece. Nothing else joins or splits. Offsets count Unicode scalar values.

Each piece gets one pick-one question. The wording is local experiment 278's p1, as local experiments 281 and 285 sent it at six pieces. With kinds it reads:

```text
The text is split into tokens at white space and at every punctuation mark or symbol, so a mark such as a period, comma, apostrophe, quote mark, bracket or hyphen is a token of its own. Decide where the marked token stands in a name of one of these kinds: KINDS. Only names of these kinds count. Names of any other kind, ordinary words, dates and numbers are outside any name.

Snippet: SNIPPET
```

`KINDS` is the caller's kind names joined by `, `, with no descriptions. With no kinds, every sentence after the first reads `Decide where the marked token stands in the name of an entity. An entity is any particular named person, organization, place, product, creative work, event or other named thing. Ordinary words, dates and numbers that are not part of a name are outside any name.`

`SNIPPET` is the original text from the start of the sixth piece before to the end of the sixth piece after, with the marked piece wrapped in `[[ ]]`. It stops at the text's edges. The options and their descriptions are:

| Option | Description |
| --- | --- |
| `BEGIN` | The token is the first token of a name of two or more tokens. |
| `INSIDE` | The token is inside a name, after its first token and before its last. |
| `END` | The token is the last token of a name of two or more tokens. |
| `SINGLE` | The token is a whole name on its own. |
| `OUT` | The token is not part of any name. |

A step-1 request holds at most 40 consecutive piece questions. Its evidence is the original text from the sixth piece before its first piece to the sixth piece after its last. A text of 40 pieces or fewer therefore sends its whole text once. The requests go out at the engine's width, as ticket 0143 sends one text's parts.

After every step-1 request returns, a Viterbi decode picks the most likely valid tag sequence over the whole text. `OUT`, `SINGLE` and `END` may be followed by `BEGIN`, `SINGLE` or `OUT`. `BEGIN` and `INSIDE` must be followed by `INSIDE` or `END`. The sequence starts with `BEGIN`, `SINGLE` or `OUT` and ends with `OUT`, `SINGLE` or `END`. The score is the sum of log probabilities, with each probability floored at one in a million. On equal scores, the sequence whose first differing tag comes earlier in the table wins. A `SINGLE` piece is a name. A `BEGIN` through its `END` is a name.

### Step 2: labels and edges

Step 2 sends one request for each step-1 request whose pieces hold the first piece of a found name. Its evidence is the original text from the sixth piece before its first name to the sixth piece after its last. It holds two kinds of question.

- **The kind question.** One per found name, when the run has kinds. The wording is open choice 1 below. Until it is filled, it is local experiment 278's `none2`: `In the text below, some words are wrapped in [[ ]]. Going by what they refer to in this text, which listed kind of name are they? Choose none of these when they are not a proper name, or when they name something that no listed kind covers.` Then `Text: ` and the name's six-piece snippet. The options are the caller's kinds in order, each with the caller's description or none, then `none of these`, described as `They are not a proper name, or no listed kind covers what they name.`
- **The edge question.** One per found name that has two or more options. The options are the name as found, the name plus a touching P or S piece at its right end, the name plus one at its left end, the name less its last piece when that piece is P or S, and the name less its first piece when that piece is P or S. "Touching" means no white space between. The wording is local experiment 279's edge wording a: `In the text below, a name was found at the words wrapped in [[ ]]. Each option wraps a slightly different stretch of the text. Pick the option that wraps exactly the whole name. A punctuation mark that is part of the name's own spelling belongs inside it. A mark that belongs to the sentence around the name stays outside.` Then `Text: ` and the name's six-piece snippet. Each option's label is its stretch's text, with a space added until it differs from the labels before it. Its description is the stretch's six-piece snippet, with `...` at an end that stops short of the text's edge.

A name whose kind answer is `none of these` is dropped. Every other name takes its picked stretch and its picked kind. With no kinds, only edge questions go out, and every name takes the kind `ENTITY`. A name with no edge question keeps its span. Names print in order of `start`, then `end`. Two names with the same span and kind print once. No other rule touches a name.

### Step 3: stated relations

Each ordered pair of kept names whose kinds match some rule's source and target gets one yes/no question. Under a rule with reading READS it reads `Does the text itself state that i1 READS i2?`. Under `either` it reads `Does the text itself state that i1 READS i2, or that i2 READS i1?`, and the pair is asked once. `*` and `ANY` expand to the kinds of kept names, in first-seen order. Names with equal text and kind are asked once, and an edge names the first. All of a text's pair questions share one request with the whole text as evidence. The existing splitter divides it at ADR 0040's ceiling and at 400 questions. A pair at or above `--relation-threshold`, default 0.5, becomes an edge.

### Output

```json
{"entities":[{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"person"}]}
```

An edge repeats both names in that shape and keeps `relation` and `probability`. `relations` is absent with no rule, and an empty list when rules gave no edge. Record modes keep `{"input":…,"value":…}`.

`--details` keeps `thinkthen.result/1`. `answer.pieces` lists each piece's offsets and its five tag probabilities. `answer.names` lists each found name's span as found, its kind probabilities and its edge option probabilities. `answer.pairs` lists each pair's probability.

`--dry-run` prints `thinkthen.recognize-plan/2`: `schema`, `url`, `model`, `key_env`, optional `from`, `pieces`, `request_count`, `name_requests_upper_bound`, the optional relation bounds, and `requests`. `request_count` counts the step-1 requests. `name_requests_upper_bound` equals it, because each step-1 request leads to at most one step-2 request. `requests` lists the step-1 requests as today.

### The guard

A text over 600,000 UTF-8 bytes exits 2 before any request, at every address. The message is `thinkthen: recognize: the text is N bytes, over the limit of M; raise it with --max-text-bytes`. `--max-text-bytes N`, from 1 to 2^53 - 1, sets the limit. It enters no digest. In record mode an oversize record fails that record, as a record over 16 MiB fails today.

### Kinds and rules

A run takes 0 to 20 distinct nonblank kinds. `none of these`, `ENTITY` and `ANY` are refused as kinds in any ASCII case, at exit 2 on the command line and 5 in a file, with no text echoed. A question file may leave out `recognize.kinds`, and the canonical question writes `"kinds":[]`. `--relation knows` means `knows=*:*` on `recognize` and `relate`. Either side may be `*` or `ANY`. The canonical question writes `*`. A file relation may leave out `source` or `target`, which means `*`. `NAME=KIND` stays malformed. A concrete side naming a kind the run lacks exits 2, as today. `relate` gains only the `ANY` alias and the optional file sides. Its planner stays.

### What goes

The five-word snippets, the detection and kind questions, the kind vote, `strength`, `--threshold` and the top-level `threshold` on `recognize`, the three default kinds, and the choice method in `recognize` relations. Every recognize question digest and request body changes.

### `audit`

A `recognize` line carries no name cut. `audit` grades its names by `start`, `end` and `kind`, as `--match strict` does today. Its row prints `threshold`, `suggested` and `crossed` as null. A `--threshold` or band over a `recognize` line exits 2 with `thinkthen: audit: recognize prints every name it keeps and takes no --threshold`. `--write` on a recognize question file exits 2 with `thinkthen: audit: a recognize file has no name cut to write`. `relate` grading is unchanged.

### Surfaces

The public Rust type keeps one entity with `text`, `start`, `end`, `length` and `kind`. Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL and SQLite return those five fields in that order, with each surface's existing offset unit. `strength` leaves every surface. The shared conformance cases carry the new shape.

## Decisions

The owner's calls. Ian can overturn each.

1. **One ticket builds the whole path.** The three steps share one request plan and one output shape. Splitting them would ship a surface that changes twice.
2. **`strength` and the name cut go.** No experiment behind ADR 0056 used a cut. The decode and `none of these` decide which names print. A computed strength over tag and kind probabilities would be a new unmeasured rule. `--details` keeps every probability. `relation_threshold` stays, because local experiment 278 measured the 0.5 cut.
3. **The field is `text`.** The coordinator set it, and the Python frame already names that column `text`. `audit` keys keep `name`, because `audit` compares offsets and kind only.
4. **Step 2 groups by step-1 request.** Local experiment 285 sent one step-2 request a name. Grouping sends fewer requests and keeps the six-piece snippets. On a short text it equals the one request a text that local experiments 278, 279 and 283 measured. Test 5 records one long text under it.
5. **The edge question goes with the kind question.** Local experiment 279 asked it after step 2, on kept names only. Asking both at once saves a round trip. A declined name's edge answer is ignored.
6. **The guard is `--max-text-bytes`, command line only.** A library or SQL caller gets the default. R6 may carry it further when a demo needs it.
7. **Overlap after an edge pick stays.** A widened name may overlap a neighbour by one mark, as `Help!` over a stray `!` name did in local experiment 283. Resolving it would be a rule.
8. **Plan schema version 2.** The key set changes, so `thinkthen.recognize-plan/1` becomes `/2`.

## Open choices

1. **The step-2 wording.** Local experiment 284 tests a new wording on the key's own names, and a second way out. The coordinator writes the winner into "Step 2" and into ADR 0056 item 7 before review. If 284 finds no gain beyond the noise, `none2` stays. Any added way-out label is reserved as a kind, as `none of these` is.
2. **Built-in kinds for web text.** Local experiment 286 tests the built-in kinds `web address`, `email address` and `social handle`, with `punctuation or symbol` as a decoy only. Ian's rule: a caller who asks for one of these kinds gets those names with that kind. Otherwise the questions still offer the three kinds, and a name picked as one of them is dropped. The coordinator writes the result into "Step 1", "Step 2", the edge cases, the tests and ADR 0056 item 7 before review. If 286 finds no gain beyond the noise, no built-in kind is added, and pieces of web addresses stay an accepted limit.

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
| `é` written as `e` plus U+0301 | U+0301 is category Mn after a letter, so it stays in the word piece |
| `👍🏽` | Two pieces of one scalar each. Both characters are category So or Sk |
| A name across two step-1 requests | One name. The decode runs after every step-1 request |
| 40 pieces | One step-1 request with the whole text |
| 41 pieces | Two step-1 requests. The second's evidence starts six pieces before piece 41 |
| One step-1 or step-2 request fails | The text fails at exit 4. No partial names print |
| Text of 600,000 bytes | Planned |
| Text of 600,001 bytes | Exit 2, zero sends |
| `--max-text-bytes 700000` on 600,001 bytes | Planned |
| `--max-text-bytes 0` | Usage error, exit 2 |
| No kinds | Kind `ENTITY`. Edge questions only in step 2 |
| Kinds `ENTITY`, `any` or `None Of These` | Exit 2, zero sends |
| `--relation knows` with no kinds | `ENTITY` to `ENTITY` pairs |
| `--relation knows=ANY:organization` | Same plan and digest as `'knows=*:organization'` |
| `--relation works_for=person` | Malformed, exit 2 |
| A name declined as `none of these` | Never expands a wildcard or enters a pair |
| Twenty kinds under a profile with `max_options` 20 | Exit 2 before any request, naming 21 options and the limit |
| `recognize --threshold 0.5` | Unknown option, exit 2 |

## Proof

Every scored test replays a recording and grades it with `audit --match strict` against `specification/fixtures/recognize/`. `kinds.jq` filters the key to the run's kinds. The bars sit under the lowest measured run, so a pass is stable and a fail is beyond the noise. The key's figures come from p1 with the whole sentence as snippet, from local experiment 279 part 1. Key sentences average 7.4 words, so a six-piece window shows most of each one.

### Outside-in tests

1. **The key at five kinds, replayed.** `person place organisation work thing` over all 200 key lines. Bar: F1 of at least 82. Local experiment 279 measured 90.6 to 91.2 on the first half and 83.6 to 84.8 on the second.
2. **The key with no kinds, replayed.** Graded on spans with every kind mapped to `ENTITY` by `jq`. Bar: F1 of at least 86. Local experiment 279 measured 91.2 to 92.3 and 87.6 to 88.2.
3. **The key filtered to `person`, replayed.** `recognize person`. A printed name of any other kind counts against precision. Bar: F1 of at least 70. Local experiment 279 measured 89.8 to 92.6 and 72.2 to 72.9. Main scores 27 to 32 (local experiment 274).
4. **Relations, replayed.** The 30 relation sentences with `sang=person:song` and `wrote=person:song`. None of the five unstated edges passes. At least 21 of the 27 stated edges pass. Local experiment 278 found 22.
5. **A long text, replayed.** Local experiment 279's invented 1,018-word text and its 65-name key join `specification/fixtures/recognize/` as `long.jsonl`. The dry run shows 30 step-1 requests of at most 40 pieces, and no request carries the whole text. The replay at five kinds scores F1 of at least 88. Local experiment 279 measured 92.9 to 94.5 with the q3 wording, and p1 at six pieces sat 0.8 points under q3 on public documents (local experiment 285). Input tokens a word stay under 450. Local experiment 285 measured 395.
6. **Loopback plans and counts.** A listener counts requests. `Ada met Acme.` sends one step-1 request, then one step-2 request. No kinds sends no kind question. A text with no names sends no step-2 request. A failed second request fails the text. The oversize refusal sends zero.
7. **Rules with no kind limits.** Dry runs of `knows`, `'knows=*:*'` and `knows=ANY:ANY` on `recognize person organization` print one plan and digest. A loopback run that declines one name asks no pair naming it. `relate` gives one plan for the same three spellings.
8. **Secrecy.** A secret in the text and in a kind description never reaches standard output, standard error or any `Debug` line, on every new failure path: the guard, a failed step-2 request, a reserved kind.

Each answers the four questions. They protect the three steps' output on real recordings, the request plan, the rule parser and secrecy. Restoring the old splitter, the old questions, the whole text as evidence, the choice method, or a wildcard over a declined kind fails at least one. No existing test pins any of these, because main's tests pin the old method. None needs a test-only hook. They drive the real command against recordings and a loopback listener.

### Edge-case tables

Three table tests in `core`, each a list of inputs and exact outputs:

- **Pieces.** Every splitter row of "Edge cases", with each piece's scalar offsets.
- **Decode.** Tag probabilities and the names they give: a lone `BEGIN` at the end, `BEGIN` then `OUT`, equal scores, a probability of zero, and a name across a request edge.
- **Edge options.** Each case's option stretches: marks at both ends, a one-piece name, a name that is one mark, and two equal labels.

### Deliberate breaks

Each break is made, run and reverted. Each must turn the named row red.

| Break | Row that turns red |
| --- | --- |
| Split U+02BC as punctuation | Pieces: `Drakeʼs` |
| Let `BEGIN` be followed by `OUT` | Decode: `BEGIN` then `OUT` |
| Drop the probability floor | Decode: a probability of zero |
| Offsets in bytes | Pieces: `👍🏽` |
| Drop `none of these` | Test 3 |
| Ask the kind question with no kinds | Test 6: no kinds |
| Send the edge question in its own request | Test 6: `Ada met Acme.` request count |
| Send the whole text as step-1 evidence | Test 5: the dry run |
| Refuse at 600,000 bytes | Edge case: text of 600,000 bytes |
| Expand `*` to a declined name's kind | Test 7 |
| Read `ANY` as a concrete kind | Test 7 |
| Print the text in the guard's message | Test 8 |

## Recordings

Every recognize request changes, so every recognize recording is made again. Each run below goes through `sdlc/scripts/live` under a token cap, and Ian authorizes it before it starts. Costs use $0.042 a million input tokens and the p1 tokens a word of local experiments 278 and 285.

| Run | Words | Estimated cost |
| --- | --- | --- |
| Test 1, the key at five kinds | 1,489 | about $0.027 |
| Test 2, the key with no kinds | 1,489 | about $0.023 |
| Test 3, the key at `person` | 1,489 | about $0.024 |
| Test 4, the relation sentences with rules | about 300 | about $0.006 |
| Test 5, the long text at five kinds | 1,018 | about $0.017 |
| Demo 44, `spec/recognize.md`, conformance cases 41 to 50 | about 200 | about $0.004 |

The total is about $0.10. The 40 harvest cases in `tests/fixtures/recognize-225` and the relation fixtures in `recognize-239` go, and the key recordings replace them. A test that still needs one of them fails loudly, and the builder moves it to a key recording.

## Pages

In the commit that changes each behavior:

- `specification/recognize.md`: rewritten around the three steps, the window, the output, the guard and the kinds rules.
- `specification/question-file.md` and its schema: optional `recognize.kinds`, optional relation sides, `ANY`, the reserved kinds, no top-level `threshold` for `recognize`, and the new canonical order: `verb`, `kinds`, optional `relations`, `relation_threshold`, optional `profile`.
- `specification/result.md`: the new `--details` members.
- `specification/channels.md`: `--max-text-bytes`, and plan schema version 2.
- `specification/settings.md`: the Kinds row default becomes none. The guard moves into the table. The `keep`, `infixes`, `prefixes`, `boundary` and `window` lines go.
- `specification/backends.md`: step-1 and step-2 requests carry a window. Relation requests carry the whole text.
- `specification/audit.md`: recognize has no cut, and the two refusals.
- `specification/relate.md`: the `ANY` alias and optional file sides.
- `specification/fixtures/recognize/README.md`: `long.jsonl` and its source.
- `spec/recognize.md` and demo 44: the new plan and output. Demo 44 drops its strength step.
- `sdlc/issues/2026-09-26-recognize-design.md`: already marked superseded by ADR 0056 where it rejects BILOU. Its R2, R3 and R4 rows gain the same marker.
- `CHANGELOG.md`: one line naming the output change, the removed options and the new guard.
- Two issues for the marketing lead name the site's recognize and relate recordings and examples, because `site/` changes hands only through them.

## Ratchet

The ceiling is 70,015 lines. The estimate is +350, from +250 to +700.

- Grows: the splitter, the step-1 questions and chunking (about 70 lines), the decode (about 45), the step-2 kind and edge questions (about 110), the three-round facade (about 60), the guard and the rule parser (about 40), and the tests: three tables and eight outside-in tests (about 450).
- Shrinks: the old splitter, the five-word window, the detection and kind questions, the kind vote and strength (about 250), and the old recognize tests that pin them (about 200).
- Before raising the ceiling, the builder looks for duplication in `core/relation.rs`, whose yes/no pair path step 3 reuses, and in the old recognize facade.

## Stop rules

1. Stop before building until the coordinator fills open choices 1 and 2.
2. Stop if the ceiling would pass 70,915, 900 over today. Report what grew.
3. Stop if a recorded run misses its bar. Report the scores to the coordinator with the error groups. Do not tune the wording to the key.
4. Stop before any paid run that Ian has not authorized.
5. Stop if a design rule here needs a ruling ADR 0056 does not give. Report it with options.
6. Stop if a deliberate break stays green.
7. Stop if `Labels` refuses an edge option label that the key produces. Report the case, and do not invent a label rule.
8. Stop if another in-flight ticket edits the same recognize files. The coordinator orders the two.

## Routing

Owner and builder: Claude. Reviewer: a fresh read-only Claude session for the design and for the diff. The change raises the ceiling and changes public output, so the diff review names what it checked.

## Complexity

Contract 3; state and timing 1; reach 3; proof 2; cost of error 2; total 11. The risk is a shape change across every surface. The conformance cases and the recorded bars guard it.

## Deferred gaps

- **Public benchmark figures.** ADR 0056 records local experiment 281's figures on the full public split and on WNUT-17. The recognize page states them with R8. Each ran once, so their run-to-run spread is unmeasured.
- **Long texts.** ADR 0056's window replaces R4's pieces and `--window`. R4 retires. `recognize --jobs` on one text stays R4b.
- **The `either` wording.** No experiment measured the step-3 wording under `either`. Test 4 uses no `either` rule.
- **Relation evidence.** Step 3 keeps the whole text as evidence, so a long text with rules can pass the backend's evidence limit and exit 4.
- **Standalone `relate`.** Its choice planner stays until its own ticket moves it to pairs under Ian's ruling of 2026-09-26.
- **Batching texts.** Record modes send each text alone, as today, until R7.
- **The accepted limits.** Titles inside names, two first names side by side, weekday names with no kinds, kinds that depend on use, pieces of web addresses and handles on web text, and `Help!` in relation texts.

## What Ian can overturn

- ADR 0056 as a whole, and each owner call it lists.
- Decisions 1 to 8.
- Open choices 1 and 2, once filled.
- The bars of tests 1 to 5.
- The four kinds items carried from the earlier version: `ENTITY`, bare rules and `ANY`, declining through `none of these`, and pairs only where a rule allows.

## Closes

None. The design issue stays open for R4b, R6, R7 and R8. `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md` stays open. This ticket answers its item 1. Its item 2 stays open for standalone `relate`.

## Evidence

- Starts from: ADR 0056 and Ian's rulings of 2026-09-26. Local experiments 278 (three steps against the baseline), 279 (the edge question, descriptions, and the rejected title, split and trim arms), 280 and 281 (the public sample, the full public split, WNUT-17, and the rejected q3 wording), 282 (error groups and plan rules), 283 (the rejected span check) and 285 (windows on public documents). Local experiment 274 for the one-kind failure. The earlier version of this ticket for the four kinds items. `origin/main` at `19ca8302`.
- Keeps: Unicode scalar offsets with an exclusive end. Record modes and their envelope. `--kind KIND=DESCRIPTION`. Relation rules, `--relation-threshold` and its 0.5 default. `relate`'s planner. The request ceiling of ADR 0040. The `audit` key files.
- Changes: The splitter, all recognize questions, the decode, windows at every length, the output shape, `--details`, the dry-run schema, the guard, the kinds and rule parsing, `audit` for recognize, and every surface's entity type. `strength`, `--threshold` and the default kinds go.
- Proof: Five replayed recordings graded by `audit` against bars under the lowest measured run, three loopback tests, three edge-case tables, and twelve deliberate breaks with the row each turns red.
- Defers: Public benchmark figures to R8. `--jobs` on one text to R4b. Batching to R7. Windowed relation evidence. Standalone `relate`'s pair method. The accepted limits.
