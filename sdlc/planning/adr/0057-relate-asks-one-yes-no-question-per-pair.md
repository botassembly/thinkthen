# ADR 0057: Relate asks one yes/no question per pair

- Status: Accepted 2026-09-27 by the coordinator after a fresh read-only review, on Ian's ruling of 2026-09-26. Ian can overturn it. Ticket 0167 builds it
- Date: 2026-09-27

This ADR moves standalone `relate` from the hybrid planner of `sdlc/planning/relate-design.md` to one yes/no question per pair a rule allows. It builds item 5 of the version of ticket 0147 that Ian accepted on 2026-09-26. ADR 0056 left that item to "a separate ticket". Ticket 0167 is that ticket. It adds a note to ADR 0019 and changes no rule of ADR 0019.

## Context

`relate` asks a different-kind relation as a choice. `core/relation.rs` `choice_plan` asks one question per entity of the larger side. Its options are the entities of the smaller side plus `none`. On equal counts the source side asks. ADR 0019 makes the options of one choice total one. So at the default cut of 0.5, one question keeps at most one edge.

This gives wrong answers with no warning. Item 2 of `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md` found it at severity 1. Live, "John Lennon wrote Help!, Girl and In My Life" gave 1 of 3 edges. In replay, three persons and three songs gave no edge. Adding a fourth, unrelated song made the song side ask, and all three edges came back. So any one-to-many fact loses edges, and an unrelated name changes which edges print.

ADR 0019 is right for `choose`, `score`, `find` and the kind and edge questions of `recognize`. Each of those has one right answer. A relation can have many. The fault lies in asking a many-answer question as a choice. The fix therefore changes the relate planner and leaves ADR 0019's rule alone.

Ian ruled on 2026-09-26. The coordinator asked him to set aside "Current final hybrid planner — authoritative" in `relate-design.md`, and he answered "yes to both". The accepted text, item 5 of ticket 0147 before commit `2e0c6551`, reads: "A relation asks one yes/no question per pair, for every concrete relation, in `recognize` and in `relate`." It adds: "No choice, no asking side and no option-ceiling fallback remain." It sets a limit of 400 questions a request. ADR 0054 item 1, accepted the same day, asks only about pairs a rule allows. Ticket 0147 built that path for `recognize` step 3 in `core/relation/stated.rs`. Standalone `relate` kept its planner.

### Evidence

Local experiment 237 compared relation methods on seven labeled sets with 89 true edges. One yes/no per pair, with the shared words in the state, found 89 of 89 with 11 false edges and the fewest input tokens. One choice per record, keeping every option above the cut, found 79 of 89 with 4 false edges. In a cluster of three duplicates, each record's two true matches split one probability, and three true pairs fell under 0.5. Where each record had exactly one target, the choice was more precise: 0.90 against 0.74 on the Beatles set at 0.5. At each method's best cut the two were level, and the set held 32 edges.

Local experiment 275 measured one shared state per text against one state per rule, with the text as evidence. The shared state cut input tokens by 59% and 51% on two sets, and its answers matched within the noise. Ticket 0147 built that shared state for `recognize`.

No experiment has measured standalone `relate` with pair questions in this ADR's wording. Ticket 0167's recorded run measures it.

### Cost

Dry runs of main at `a057c594` give today's figures. A script that builds this ADR's request bodies with the same compact JSON and the same split rules gives the pair figures. Names are 15 to 21 bytes. The build's dry runs pin the exact numbers.

| Entities | Rules | Today: questions, requests, bytes | Pairs: questions, requests, bytes | Bytes |
| --- | --- | --- | --- | --- |
| 3 persons, 3 songs | `wrote` | 3, 1, 1,397 | 9, 1, 968 | -31% |
| 5 persons, 20 songs | `wrote` | 20, 1, 9,700 | 100, 1, 8,192 | -16% |
| 10 persons, 50 organizations | `works_for` | 50, 1, 36,590 | 500, 2, 44,614 | +22% |
| 5 persons, 180 songs | `wrote` | 180, 1, 84,353 | 900, 3, 91,304 | +8% |
| 4 persons, 184 songs, 13 albums | `sung_by`, `appears_on` | 368, 3, 244,892 | 3,128, 8, 319,692 | +31% |
| 127 persons, 128 organizations | `works_for` | 128, 10, 920,792 | 16,256, 41, 1,869,452 | +103% |

A small set costs less with pairs. Each choice option repeats its entity's kind and name as a description, and each choice repeats the asking entity. A large set costs more, because pairs grow with the product of the two sides. The token figures for pair bodies are unmeasured. `specification/backends.md` records relate's JSON at 0.516 input tokens a byte, measured on choice bodies. Pair questions are short in bytes and long in tokens. Local experiment 260 measured about 73 input tokens a packed yes/no question. At $0.042 a million input tokens, today's 184-song set costs about $0.0053 a run and the 255-entity set about $0.020. At 73 tokens a question plus the state, the pair plans come to about $0.011 and $0.056. Ticket 0167's recorded run measures the pair rate.

The 400-question limit causes part of the growth, because every request repeats the entity list. With the byte ceiling alone, the last three rows would be 72,708, 269,280 and 1,485,228 bytes: -14%, +10% and +61%, a saving of 16% to 21%. The limit guards the backend's limit of 65,536 input tokens. At 73 tokens a question, 900 pair questions in one request come to about 65,700 tokens, and ticket 0147's accepted history set 400 for that reason. 400 questions come to about 29,200 tokens. So this ADR keeps it.

## Decision

1. **One yes/no question per pair a rule allows.** Every relation asks one yes/no question for each ordered pair of entities whose kinds match the rule's source and target. `*` matches any kind. An `either` rule asks each unordered pair once. No entity pairs with itself. No relation asks a choice. No side asks, and no option limit changes the method.
2. **One state per entity set.** Every request of a run carries the same state: `{"entities":[…]}`, holding each entity of a kind some rule names, in input order, with ids `i1`, `i2` and onward. The state carries no `relation` key. Every rule's questions share the requests. This is `recognize` step 3's state with no text.
3. **The question names its relation.** A question reads `Is it true that i1 READS i2?`, where READS is the rule's `reads`. Under `either` it reads `Is it true that i1 READS i2, or that i2 READS i1?`. `recognize` step 3 asks the same sentence with `Does the text itself state that` in place of `Is it true that`. The lead words are the only difference, and one planner builds both.
4. **Requests hold at most 400 questions.** Questions go into requests in rule order, then source order, then target order. A request holds at most 400 questions and fits under the request-size ceiling of ADR 0040 and ticket 0123. A profile's `max_questions` lowers the 400, as it does for `recognize` step 3. The limit applies at every address. Ian's accepted text reads "A relation request at the built-in address holds at most 400 questions, and a profile's `max_questions` replaces that number." The coordinator ruled on 2026-09-27 to apply it at every address, as `recognize` step 3 does.
5. **An edge is a yes at or above the cut.** The cut stays inclusive, with its 0.5 default. Edges print in question order. A directed edge keeps the rule's direction. An `either` edge puts its endpoints in input order.
6. **The edge shape stays.** Bare output, library edges and SQL rows keep `relation`, `source`, `target` and `probability`. `--details` keeps `thinkthen.result/1`, and every entry of `answer.questions` is today's yes/no entry. Partial output at exit 6 stays.
7. **The dry run keeps version 1.** `thinkthen.relate-plan/1` keeps its keys, as Ian's accepted text says. Each entry of `relations` is one rule as given. Its `method` is always `yes_no`, and its `fallback` always null. Its `logical_questions` counts its questions, and its `request_count` counts the requests that carry them. Rules share requests, so the entries' counts can add to more than the top-level `request_count`.
8. **The page says where an edge comes from.** `relate` sends no text, so an edge comes from the model's knowledge of the names. `specification/relate.md` says so, and points to `recognize --relation` for edges a text states.

## Consequences

- A one-to-many or many-to-many fact keeps every edge the model believes. An unrelated entity adds questions and changes no other pair's question.
- Every relate request body changes. Old relate recordings and cache entries miss. The question digest does not change, because no method enters it.
- `recognize` request bodies stay byte for byte. Ticket 0147's recordings prove it.
- `max_options` no longer affects `relate`. The fallback and the choice detail entries go. The plan keeps `method` and `fallback` as constants.
- A plan entry is now a rule as given, not a concrete kind pair. A reader that expected one entry per kind pair under a wildcard sees one entry.
- Edges of a wildcard rule print in question order, where they printed grouped by concrete kind pair.
- A large set costs up to about twice today's request bytes, and its tokens are unmeasured. The entity limit of 255 bounds one rule at 16,256 cross-kind pairs. Today's same-kind rules already reach 64,770 pairs.

## What this amends

| Where | What changes |
| --- | --- |
| `relate-design.md`, "Current final hybrid planner — authoritative" | Superseded. Every relation asks yes/no pairs |
| `relate-design.md`, "Exact option and backend-profile fallback" | Superseded. No fallback remains |
| `relate-design.md`, "Exact relation request state" | One state per run with no `relation`, and the question wording of item 3 |
| `relate-design.md`, "Exact dry-run schema" | Amended. The schema stays version 1. Each entry is a rule as given, `method` is always `yes_no`, `fallback` is always null, and `request_count` counts the requests carrying the rule's questions |
| `relate-design.md`, "Ruled Option A detailed result" | Choice entries no longer occur. Yes/no entries stay |
| ADR 0019 | A note: relations ask no choice, so a choice's total no longer limits relation edges. The rule itself stands |
| Ticket 0123 and `backends.md` | The ceiling's sentence "It never turns a choice into yes/no questions" loses its object, and the relate fallback sentence goes |

## What Ian can overturn

Ian's ruling of 2026-09-26:

1. Pairs for every relation, with no choice, no asking side and no fallback.
2. The limit of 400 questions a request.

Owner calls, proposed with ticket 0167:

3. One shared state per run in `relate`, from local experiment 275, in place of one state per rule.
4. The wording `Is it true that i1 READS i2?` and its `either` form.
5. A profile's `max_questions` lowering the 400 and never raising it, as `recognize` step 3 does. The ruling's text says "replaces".
6. Keeping `method: "yes_no"` in detail entries so that entries keep their shape.

The coordinator's rulings of 2026-09-27:

7. The limit at every address, as `recognize` step 3 applies it. It departs from the accepted text, which limits requests "at the built-in address".
8. Plan schema version 1 with one entry per rule, as the accepted text keeps `method` and `fallback`.

## Amendment, 2026-09-30: single-answer menus (ticket 0342)

A relate rule the caller marks `"single": true` asks one `choice` per source name. Its options are every other name the target side admits, then `none`, each described as the old choice planner described them. The top label makes the one edge when it is a target at the cut; `none` on top, or an exact tie at the top, makes none. Every rule without `single` keeps items 1 to 8 unchanged, and so does `recognize`, which refuses `single`. A profile's `max_options` counts a menu's options. Detail entries for a menu carry `method:"choice"` and a nullable `target`, and the plan reports `method:"choice"` for the rule.

This ADR's fault was a many-answer relation asked as a choice. A `single` rule is one the caller declares to have at most one target per source, so that fault does not arise. Experiment 237 found the choice more precise where each record had one target, 0.90 against 0.74, and the Beatles Bench of 2026-09-30 measured pairs at precision 0.420 on 182 songs and 0.296 where the right album is missing. The coordinator set the menu as the relate precision default on 2026-09-30. It departs from the text of Ian's ruling of 2026-09-26, "for every concrete relation", so Ian can overturn it. A capped paid Beatles Bench run decides whether it stays: it must beat F1 0.523 on the 182 songs and precision 0.296 on the missing-album sets. The run of 2026-09-30 passed; see the amendment for ticket 0353.

## Amendment, 2026-09-30: both-ways edges say so (ticket 0344)

Item 6 keeps `relation`, `source`, `target` and `probability`. An edge of an `either` rule now also ends with `"either":true`, on the command, `recognize`, the Rust library, the C door and every binding; the SQL `thinkthen_relate` tables and relation rows gain a last `either` column. Item 5 stands: the ends stay in input order. A directed edge writes no `either` member, so its bytes stay. No question, request or cache key changes. The coordinator chose the flag over a `pair` array, which would give one-way and both-ways edges two shapes; Ian can overturn it.

## Amendment, 2026-09-30: the decision run settles the default (ticket 0353)

The capped paid Beatles Bench run of 2026-09-30 passed the bar on the default backend at the 0.5 cut, with song to album marked `single`: edge F1 0.635 against 0.523 on the 182 songs, and precision 0.541 against 0.296 on the 16 missing-album sets. So `single` is the recommended form for a relation where each source has at most one target. The run settled this default, and Ian can overturn it. `single` stays opt-in: a rule without it asks yes/no pairs under items 1 to 8, which remains the way to ask per pair for a many-answer relation or where recall matters more. The relate page states the cost. Recall on the 182 songs fell from 0.693 to 0.551, and at the tuned cut of 0.47 the menu tied pairs on solo sets, 0.680 against 0.689. The Liquid d1 cell was not run and stays open.
