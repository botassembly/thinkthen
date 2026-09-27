# 0147: Build recognize in three steps

Status: built 2026-09-27, awaiting code review. Owner: Claude.

Branch `ticket/0147-recognize-adr`, built in lane 3. The ticket is `sdlc/tickets/0147-recognize-adr.md`, and the authority is ADR 0056. The build merged `origin/main` at `0e8e4ae8` first. Ian can overturn every decision the ticket lists, and the coordinator's two calls below.

## Result

- Step 1 splits the text into pieces and asks one q3 BILOU question per piece, at most 40 a request, with six pieces of window each side. A Viterbi decode over the whole text finds the names. Offsets count Unicode scalar values.
- Step 2 sends one request per step-1 request that holds a name's first piece. It asks each name's kind, with `none of these`, and its edge question. With no kinds it asks only edge questions, and a request with no questions is not sent.
- Step 3 asks one yes/no question per ordered pair a rule allows, about what the text itself states. It shares the relation state and edge path with `relate`.
- Each name prints `text`, `start`, `end`, `length`, `kind` and `strength`. `strength` is P(kind) times P(span) from one forward-backward pass, at four places, and the cut reads the printed value.
- `--details` carries `answer.pieces`, `answer.names` and `answer.pairs`. The dry run prints `thinkthen.recognize-plan/2`.
- `--max-text-bytes`, default 600,000, refuses a longer text before any request.
- Kinds may be absent, and every name then has the kind `ENTITY`. `none of these`, `ENTITY` and `ANY` are reserved in any ASCII case. A bare rule means any kind to any kind, `ANY` spells `*`, and a file rule may leave out a side. `relate` gains the `ANY` alias and the optional sides.
- `audit` grades a recognize line with no kinds under the `ENTITY` rule.
- Every library and database surface returns the six fields in the same order, in its own offset unit. The shared conformance cases 41 to 50 are captured from live recordings.
- `sdlc/scripts/policy.py` refuses any header, credential key or bearer value in a tracked recording. A planted `Authorization` header in a copy of a recorded entry turned it red.
- The Unicode tables for categories P, S, Mn, Me and Cf are generated from Unicode 15.0.0 and embedded in `core/recognize/categories.rs`.
- The 40 harvest cases in `crates/thinkthen/tests/fixtures/recognize-225` are gone. `recognize-239` stays, because `relate`'s choice-planner tests replay it.
- Pages: `specification/recognize.md`, `question-file.md` and its schema, `result.md`, `audit.md`, `channels.md`, `settings.md`, `backends.md`, `relate.md`, the fixtures README, `spec/recognize.md`, `spec/audit.md`, demo 44, the design issue's R0 to R8 markers and `CHANGELOG.md`. Two issues hand the site's recognize and relate samples to the marketing lead.

## Recordings

One job through `sdlc/scripts/live`, capped at 4,000,000 input tokens, recorded every run. It used 1,963,437 input tokens, about $0.082 at $0.042 a million. The budget was $0.30. No other paid call ran. Every recording entry holds no header.

| Run | Row | F1 | Bar | Met |
| --- | --- | --- | --- | --- |
| Test 1, five kinds | matched 292, extra 36, missed 55 | 0.865 | 0.82 | yes |
| Test 2, no kinds | matched 336, extra 52, missed 36 | 0.884 | 0.86 | yes |
| Test 3, `person` | matched 80, extra 19, missed 10 | 0.847 | 0.70 | yes |
| Test 4, stated edges | matched 20, extra 6, missed 7 | 0.755 | 19 of 27 | yes |
| Test 4, unstated edges | matched 0, extra 26, missed 5 | 0.000 | exact row | yes |
| Test 5, long text | matched 60, extra 2, missed 3 | 0.960 | 0.88 | yes |

Test 5's dry run plans 1,183 pieces in 30 step-1 requests of at most 40 pieces. Its replay used 262,504 input tokens for 1,018 words, 258 a word, under the bar of 450.

Test 4's bar was 21. The run found 20, and stop rule 2 stopped the build. The coordinator ruled on 2026-09-27 to set the bar at 19, one under the recorded 20. Ian can overturn that call. No wording was tuned. Four misses are `Help!`, on c06, c07, c09 and c23, each with a matching `Help` extra. Step 3 said no on c08, c16 and c29. The other two extras are the optional edges of c10 and c12. In c23 step 1 also found a stray `"` as a song.

Demo 44's run gives Maria Chen 0.9987 as `PER`, Northwind Freight 0.997 as `ORG` and Chicago 1.0 as `LOC`.

## Tests

- Tests 1 to 5 and test 8 replay the recordings on `specification/fixtures/recognize/README.md` under "Replayed runs", which the `spec` rung runs. Each pins its exact grade row.
- Test 6 is `tests/backend/recognize.rs`, over a loopback that counts requests and keeps bodies.
- Test 7 is `tests/backend/recognize/rules.rs`. Its question and digest come from `--details`, because the plan schema holds no question.
- Test 9 is `tests/backend/secrecy_recognize.rs`.
- The five edge-case tables are `src/core/recognize/tables.rs`.
- `tests/audit_sets.rs` grades demo 44's new recording. Every name there scores above 0.99, so the test lowers the name the key leaves out to 0.6 before `--write`, which then writes 0.61.

## Deliberate breaks

Each break was planted in the working tree, run against its named check, and reverted. The crate was clean after each revert.

| Break | Check | Result |
| --- | --- | --- |
| Split U+02BC as punctuation | Pieces | Red: `Drakeʼs` gave three pieces, not one |
| Let `BEGIN` be followed by `OUT` | Decode | Red: `BEGIN then OUT` decoded no name |
| Drop the probability floor | Decode | Red: the floor row decoded no name |
| Offsets in bytes | Pieces | Red: red at the first row with a multi-byte scalar, `Drakeʼs`, which gave `(0, 8)`. The table stops at its first failing row, so `👍🏽` sits after it and fails the same way |
| Drop `none of these` | Test 3 | Red: the replay missed its recording |
| Ask the kind question with no kinds | Test 6, no kinds | Red: red |
| Send the edge question in its own request | Test 6, `Ada met Acme.` | Red: the request count grew |
| Send the whole text as step-1 evidence | Test 5, the dry run | Red: a step-1 request carried the whole text |
| Refuse at 600,000 bytes | Test 6, the guard | Red: the 600,000-byte text exited 2 |
| Let a declined name enter the pair list | Test 7 | Red: `Nobody` entered the state, with its pairs |
| Read `ANY` as a concrete kind | Test 7 | Red: `knows=ANY:ANY` exited 2 |
| Print the text in the guard's message | Test 9 | Red: the secret reached standard error |
| Drop `strength` from a name | Test 8 | Red: audit refused the lines |
| Score with the lowest tag probability, today's form | Score | Red: red at `three pieces`, 0.72 against 0.8927, the first row it changes |
| Keep a name under the cut | Score | Red: `at 0.49` printed its name |
| Cut on the raw product, not the printed strength | Score | Red: `raw 0.49996` dropped its name |
| Compute P(span) over the widened stretch | Score | Red: `widened by the edge pick` dropped `Help!` |
| Drop the `ENTITY` rule in audit | Test 2 | Red: audit refused the key |
| Put descriptions in step 1 | Test 6, descriptions | Red: a description reached step 1 |
| Drop descriptions from step 2 | Test 6, descriptions | Red: the step-2 option lost its description |
| Chunk at 41 pieces | Requests | Red: 41 pieces made one request |
| Use a five-piece window | Requests | Red: the evidence became `[70,81)` |

All 22 went red. None stayed green, so stop rule 5 did not fire.

## Ratchet

The ceiling was 70,310 when the build started. The first full build measured 71,673, over the ticket's +900 stop, and stop rule 1 stopped the build. The coordinator ruled to trim first, then raise by at most 1,200 with reasons. The trim removed the duplicated relation state and edge path between `core/relation.rs` and `relation/stated.rs`, shared the relation ceiling, folded repeated test setup into helpers, and derived the aggregate's default. The edge-case tables stay whole, and the generated Unicode tables stay embedded, because `ratchet.mjs` has no generated-file marking. The ceiling is now 71,510, +1,200, in two commits that say what grew. Ian can overturn the raise.

## Rungs

All four ran with `THINKTHEN_API_KEY` unset and exited 0 on the final commit: `lint`, `test`, `spec` and `surfaces`. The `lint` run printed its expected `Killed` line. The `test` rung runs the doctests under `--cfg thinkthen_internal_doctest`, and they pass there. A plain `cargo test --doc` without that flag fails by design.

## Deviations

1. The Unicode tables are embedded, with no new crate. The ratchet counts them, because `ratchet.mjs` has no generated-file marking.
2. Test 7 reads the question and its digest through `--details`, because the plan schema has no question.
3. `recognize-239` stays, because `relate`'s planner tests replay it.
4. `text_of_64000_bytes_keeps_its_reply` is deleted. The guard and test 5 cover long text.
5. `sdlc/scripts/inventory` reads the new `RecognizedEntity::text` and `length` from the ticket's "Changed declarations" block, and drops `name`.
6. The conformance core's captured-exchange check reads any committed recording under `demos/` or `specification/`, in place of one pinned file.
7. `relations_are_self_contained_and_absent_without_a_rule` folded into `rules.rs`.
8. The speed probe counts 24 recognize requests for 12 items, because each text sends a step-1 and a step-2 request.
9. `tests/backend/recognize.rs` passed the 500-line ceiling, so its profile, cache, recording, details and worker tests moved to `recognize/stores.rs`.

## Review fixes

Two code reviews of d1ceea41 found the items below. Each fix is its own commit.

1. A name across a line feed or a tab put a control character in its edge label. The label check refused it, and the run exited 2 with the kinds message after step 1 was paid. The coordinator ruled that each edge label shows every white-space run as one space. The description keeps the real snippet. A step-2 label error now maps to a defect. The edge table gains `"Maria\nChen."` and `"Maria\tChen!"` rows. Ian can overturn the ruling.
2. The decode and the forward pass share one helper for the tags that may come before a tag. The total and a span's `before` share one `closed` sum. `Odds::leader` calls `best_of`. `audit` and the reserved kinds import `ENTITY` and `NONE_OF_THESE` from `recognize`. The ceiling fell to 71,503, the measured total after fixes 1 and 2.
3. Ruby's `recognize` gave each kind in a plain list a description equal to its name. ADR 0056 forbids a default description, so each such kind now carries none, as in TypeScript.
4. Renaming `name` to `text` broke the handoff from `recognize` to `relate` on the command and in Python, Ruby, TypeScript, R and C. The coordinator ruled that `relate` reads `text` as the name when `name` is absent, and `name` wins when both are present. The command falls back only at the default `/name` field. R's `tt_relate` again reads `tidyr::unnest()` of `tt_recognize`, and its README and comment say so again. Each surface gained one test that feeds `recognize` output into `relate`. The command test replays demo 44 and plans a `relate` run from its names. A planted break that turned off the command's fallback turned that test red. The DuckDB, PostgreSQL and SQLite `relate` functions read columns by place, so they needed no change. The line ceiling rose by 75 to 71,578, for the command's fallback and its test. The surface ceilings rose to their measured totals: C +34, Python +8 and +20, R +7, Ruby +18, and TypeScript +1, +13 and +4.
5. The recognize keys' page says each key line takes `audit`'s key shape, and that `audit` reads only the offsets and the kind. `backends.md` now begins its fallback sentence with "In `relate`".

## Left for later

- The accepted limits in the ticket's "Deferred gaps" stand, `Help!` among them.
- The flat relation tables of R, DuckDB and PostgreSQL carry only text and kind for each end.
- `site/` changes through the two marketing issues.
