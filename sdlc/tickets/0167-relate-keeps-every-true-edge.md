---
flow: build
priority: 167
opens: sdlc/planning/adr/0057-relate-asks-one-yes-no-question-per-pair.md sdlc/planning/adr/0019-an-answer-distribution-totals-one.md sdlc/planning/relate-design.md crates/thinkthen/src/core/relation.rs crates/thinkthen/src/core/relation crates/thinkthen/src/core/mod.rs crates/thinkthen/src/core/backend_profile.rs crates/thinkthen/src/engine/prepared_request.rs crates/thinkthen/src/engine/facade/relate.rs crates/thinkthen/src/engine/facade/recognize.rs crates/thinkthen/src/cli/relate.rs crates/thinkthen/src/cli/relate crates/thinkthen/tests/backend/relate.rs crates/thinkthen/tests/backend/relate crates/thinkthen/tests/fixtures/recognize-239 conformance specification/relate.md specification/backends.md specification/result.md specification/channels.md specification/check.md specification/fixtures/relate spec/relate.md spec/fixtures/relate-partial demos/45-map-relationships sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets CHANGELOG.md
---

# 0167: Relate keeps every true edge

Status: ready for review. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and ADR 0057, and later the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Standalone `relate` asks one yes/no question for each pair of entities a rule allows. It keeps every edge whose yes probability reaches the cut. A one-to-many fact keeps all its edges, and an unrelated entity changes no other edge. All rules share one entity state and one set of requests. The page says that an edge comes from the model's knowledge.

This fixes item 2 of `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md`, the only open severity-1 finding with no owner.

The authority is Ian's ruling of 2026-09-26. The coordinator asked him to set aside the hybrid planner of `sdlc/planning/relate-design.md`, and he answered "yes to both". The accepted text is item 5 of ticket 0147 before commit `2e0c6551`: "A relation asks one yes/no question per pair, for every concrete relation, in `recognize` and in `relate`." It adds: "No choice, no asking side and no option-ceiling fallback remain." It sets 400 questions a request. ADR 0056 and ticket 0147's deferred gaps leave standalone `relate` to "a separate ticket" under that ruling. This is that ticket. ADR 0057, filed with it, records the decision.

## What happens today

Each line is at `origin/main` `a057c594`.

- `core/relation.rs` lines 164 to 177, `plan_concrete`: a same-kind relation asks yes/no pairs. A different-kind relation asks a choice, unless the smaller side plus `none` passes 255 options.
- Lines 188 to 257, `choice_plan`: line 194 sets `reversed` when the source side is smaller, so the larger side asks, and the source side asks on equal counts. Each asking entity gets one choice over the other side plus `none`. Lines 240 to 242 word it `Which listed KIND fills the blank: Item 1 (person "Ada") works for ___? Choose none if no listed KIND does.`
- ADR 0019 makes a choice's options total one. Lines 402 to 419, `push_choice_edges`, keep every option at or above the cut. At the default cut of 0.5, one choice keeps at most one edge.
- Lines 259 to 295, `pair_plan`: a same-kind question reads `Does the relation hold from i1 to i2?`, or `between i1 and i2` under `either`.
- Lines 304 to 339, `relation_evidence`: each concrete relation's state holds every entity of the set, whatever its kind, and the one rule as `relation`.
- `engine/facade/relate.rs` lines 55 to 76: one request group per concrete relation, each with its own state. `engine/prepared_request.rs` lines 87 to 118: a choice refused by a profile's `max_options` or request-byte limit falls back to pairs for that concrete relation.
- `cli/relate/dry_run.rs` lines 69 to 79 print each concrete relation's `method`, `fallback`, `logical_questions` and `request_count` under `thinkthen.relate-plan/1`. `cli/relate/result.rs` lines 176 to 286 print choice entries with an asker, candidates and a pick.
- `relate` applies no question limit. At a loopback address, 180 `--lines` names under one bare rule send 32,220 questions in one request (`tests/backend/relate.rs`, `a_relation_of_180_names_keeps_its_reply`).
- `specification/relate.md` line 46 states the choice method. No line says where an edge comes from.

What an integrator hits, from the architect review: live, "John Lennon wrote Help!, Girl and In My Life" gave 1 of 3 edges. In replay, three persons and three songs gave no edge, because John Lennon asked and his 0.98 split three ways. Adding a fourth, unrelated song made the songs ask, and all three edges printed.

## Design

ADR 0057 decides it. In brief:

1. Every relation asks one yes/no question per ordered pair whose kinds match the rule's sides. `*` matches any kind. An `either` rule asks each unordered pair once. No entity pairs with itself.
2. Every request of a run carries one state, `{"entities":[…]}`. It holds each entity of a kind some rule names, in input order, with ids `i1`, `i2` and onward. It carries no `relation`. All rules' questions share the requests.
3. A question reads `Is it true that i1 READS i2?`. Under `either` it reads `Is it true that i1 READS i2, or that i2 READS i1?`.
4. Questions go into requests in rule order, then source order, then target order. A request holds at most 400 questions and fits under the request-size ceiling. A profile's `max_questions` lowers the 400. The limit applies at every address.
5. A yes at or above the cut is an edge. Edges print in question order. An `either` edge puts its endpoints in input order.
6. The dry run becomes `thinkthen.relate-plan/2`. The edge shape, the detail entries and exit 6 stay.
7. `relate.md` says that an edge comes from the model's knowledge.

### One path

`recognize` step 3 already asks pairs this way. `core/relation/stated.rs` `plan_stated` filters names to rule kinds, builds one state, and asks one question per allowed pair. `stated_edges` draws the edges. The build makes both generic over `RelationEntityView`, takes the text as `Option<&str>`, and takes the lead words as a two-case enum: `Stated` for `Does the text itself state that` and `Known` for `Is it true that`. That enum is the only difference between the two verbs' questions. The recognize facade's split loop, `engine/facade/recognize.rs` lines 181 to 199, moves into one helper both facades call, with its limit of 400 and the ceiling.

`relate` keeps its own execution. It keeps each failed logical answer as a failed entry and exits 6, where `recognize` fails the whole text on any failed answer. The relate facade maps each answer to its `StatedPair`, in place of today's `QuestionMap`.

What goes: `plan`, `concrete_relations`, `admitted_kinds`, `expanded_side`, `plan_concrete`, `matching`, `choice_plan`, `pair_plan`, `plan_pairs`, `relation_evidence`, `QuestionMap`, `assemble_edges` and `push_choice_edges` in `core/relation.rs`. `SettledRelation`'s fallback and `LimitKind::permits_relation_fallback`. `Method`, `finished` and the per-relation prepared requests in the relate facade. The choice entry and `leader` in `cli/relate/result.rs`. `fallback_name` in the dry run. The name `stated.rs` becomes `pairs.rs`, and `StatedPlan` and `StatedPair` become `PairPlan` and `Pair`.

### Where this ticket argues with the brief

- **ADR 0019 needs only a note.** ADR 0019's total-one rule is right for every choice. The fault lies in `relate-design.md`, which asks a many-answer relation as a choice. ADR 0057 supersedes that planner. ADR 0019 gains one note saying a relation asks no choice, and its rule stands.
- **The same-kind wording changes too.** One path means the same-kind question moves from `Does the relation hold from i1 to i2?` to the new wording. Local experiment 237 found yes/no with the words in the state, `Does this hold: …`, the cheapest and most accurate method. Local experiment 275 found that naming the relation in each question, with one shared state, matched the per-rule state within the noise at half the tokens.
- **The 400 limit has a cost.** It makes the largest sets repeat the entity list in every request. Without it, the 184-song set would cost 10% more than today, not 31%. Ian's ruling sets 400, and `recognize` step 3 uses it, so the ticket keeps it and names it for Ian to overturn.

### The shared state

The shared state of `2026-09-26-relation-requests-carry-one-rule-and-every-entity.md` falls out of the one path, so this ticket folds it in. `recognize` has had it since ticket 0147. `relate` gains it here. Local experiment 275 measured it with a text. Without a text, the saving is smaller: one entity list per request in place of one per concrete relation.

### Cost against today

ADR 0057's cost table gives the figures. Main's `relate --dry-run` at `a057c594` measured today's bytes. A script built the pair bodies with the same compact JSON and split rules. Names were 15 to 21 bytes.

| Entities | Today, bytes | Pairs, bytes | Change |
| --- | --- | --- | --- |
| 3 persons, 3 songs, `wrote` | 1,397 | 968 | -31% |
| 5 persons, 20 songs, `wrote` | 9,700 | 8,192 | -16% |
| 10 persons, 50 organizations, `works_for` | 36,590 | 44,614 | +22% |
| 5 persons, 180 songs, `wrote` | 84,353 | 91,304 | +8% |
| 4 persons, 184 songs, 13 albums, `sung_by` and `appears_on` | 244,892 | 319,692 | +31% |
| 127 persons, 128 organizations, `works_for` | 920,792 | 1,869,452 | +103% |

A typical set of a few dozen entities costs the same or less. A large set costs up to about twice as much. At relate's measured 0.516 input tokens a byte and $0.042 a million input tokens, the 184-song set moves from about 126,000 to 165,000 tokens, $0.0053 to $0.0069. The 255-entity set moves from about 475,000 to 965,000 tokens, $0.020 to $0.041.

### The entity limit and the request ceiling

- The limit of 255 entities bounds one cross-kind rule at 127 × 128 = 16,256 pairs. A one-way `*:*` rule over one kind reaches 255 × 254 = 64,770 pairs, as today's same-kind rules already do. Each extra rule adds its own pairs.
- Every request repeats the state. At 255 entities of about 48 bytes each, the state is about 12 KB. A pair question adds about 65 bytes. So 400 questions and the state come to about 38 to 46 KB, under 96,000 bytes, and the 400 limit binds first.
- The ceiling binds first only when the state passes about 70 KB, such as 255 entities whose names average over 250 bytes. The splitter then packs fewer questions a request, as today.
- A state over 96,000 bytes still goes one question a request, because the ceiling splits and never refuses. The backend refuses a request over 65,536 input tokens, about 127,000 bytes at 0.516 tokens a byte, and the run exits 4. This matches today, and it is a deferred gap.
- No question grows with the set. Today one choice over 128 options carries about 7 KB.
- Ticket 0154 carries the ceiling to every address as `--max-request-bytes`. This ticket reads whatever request size 0154 resolves.

## Retained behavior

- Inline rules, `@FILE`, `ANY`, optional file sides, `reads`, `either` and the one inclusive cut with its 0.5 default.
- Entity input: JSON, `--jsonl`, `--csv`, `--tsv`, `--lines` with kind `*`, `--field`, `--kind-field`, and a found name's `text`.
- Validation before any request, at exit 2 with zero sends: a malformed rule, a bad pointer value, a blank name or kind, a duplicate name and kind, an absent concrete kind, an incompatible line rule, and a 256th entity. The empty-input outcomes.
- The edge shape on every surface. `relation`, `source` and `target` as `{name, kind}`, and `probability`.
- `--details` under `thinkthen.result/1`, and today's yes/no entry byte for byte, `method: "yes_no"` included. Failed entries, `meta.failed_questions`, and exit 6 with partial output.
- The question digest. No method enters it, so saved bars and `audit --write` keep working.
- `--jobs`, the request-size ceiling, profiles, record, replay and cache rules.
- `recognize` request bodies, byte for byte.

## Edge cases

The model is `jev-1.13.0` in every body. Rows 1 and 2 use a loopback listener that answers 0.98 to each question whose pair is John Lennon and one of his three songs, and 0.02 to every other question.

| # | Case | Expected |
| --- | --- | --- |
| 1 | `relate wrote=person:song --jsonl` over John Lennon, Paul McCartney and George Harrison as `person`, and Help!, Girl and In My Life as `song` | 9 questions, 1 request. Exit 0. Standard output is exactly the three lines below |
| 2 | Row 1 plus Yesterday as `song` | 12 questions, 1 request. The same three lines |
| 3 | `relate works_for=person:organization --jsonl --dry-run` over Ada `person`, Paris `place`, Acme `organization` | The plan below. The body holds no `Paris` |
| 4 | `knows=person:organization --either` over Acme `organization`, then Ada `person` | One question: `Is it true that i1 knows i2, or that i2 knows i1?`. A yes prints source Acme, target Ada |
| 5 | `sang=person:song wrote=person:song` over Ada `person`, S1 and S2 `song` | 1 request, 4 questions in order: `Is it true that i1 sang i2?`, `… i1 sang i3?`, `… i1 wrote i2?`, `… i1 wrote i3?` |
| 6 | Bare `knows` over Ada `person`, Acme `organization`, Bob `person` | 6 questions, pairs in order (i1,i2), (i1,i3), (i2,i1), (i2,i3), (i3,i1), (i3,i2) |
| 7 | `knows --either` over the same three | 3 questions: (i1,i2), (i1,i3), (i2,i3) |
| 8 | `r --lines` over `a` and `b` | 2 questions: `Is it true that i1 r i2?`, `Is it true that i2 r i1?`. The state gives both kind `*` |
| 9 | 5 persons and 180 songs, `wrote`, dry run | 900 questions, 3 requests of 400, 400 and 100 |
| 10 | 127 persons and 128 organizations, `works_for`, dry run | 16,256 questions, 41 requests: 40 of 400, then 256. Each under 96,000 bytes |
| 11 | Row 9 with a profile of `max_questions` 100 | 9 requests of 100 |
| 12 | Row 9 with a profile of `max_questions` 1000 | 3 requests, as row 9 |
| 13 | Row 1 with a profile of `max_options` 2, dry run | The same requests and digests as with no profile. `backend_profile` names the profile |
| 14 | 180 `--lines` names, bare `r`, loopback | 32,220 edges, 81 requests: 80 of 400, then 220 |
| 15 | Bare `knows` over one entity | No request, no output, exit 0 |
| 16 | A yes of exactly 0.5 at the default cut | The edge prints |
| 17 | 256 entities | Exit 2, zero sends, today's sentence |
| 18 | `works_for=person:company` with no `company` entity | Exit 2, zero sends, today's sentence |
| 19 | One of two answers in a reply fails its decode | The good edge prints, exit 6. `--details` shows one failed yes/no entry |
| 20 | Row 3 under `recognize` wording | `Does the text itself state that i1 works for i2?` |

Row 1 and row 2 print exactly:

```json
{"relation":"wrote","source":{"name":"John Lennon","kind":"person"},"target":{"name":"Help!","kind":"song"},"probability":0.98}
{"relation":"wrote","source":{"name":"John Lennon","kind":"person"},"target":{"name":"Girl","kind":"song"},"probability":0.98}
{"relation":"wrote","source":{"name":"John Lennon","kind":"person"},"target":{"name":"In My Life","kind":"song"},"probability":0.98}
```

Row 3 prints exactly this plan, with the digest filled in:

```json
{"schema":"thinkthen.relate-plan/2","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","backend_profile":null,"framing":"jsonl","fields":{"name":"/name","kind":"/kind"},"entity_count":3,"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false,"logical_questions":1}],"logical_questions":1,"request_count":1,"requests":[{"digest":"<digest>","bytes":219,"body_utf8":"{\"state\":{\"entities\":[{\"id\":\"i1\",\"name\":\"Ada\",\"kind\":\"person\"},{\"id\":\"i2\",\"name\":\"Acme\",\"kind\":\"organization\"}]},\"model\":\"jev-1.13.0\",\"questions\":{\"q1\":{\"type\":\"noul\",\"instructions\":\"Is it true that i1 works for i2?\"}}}"}]}
```

`entity_count` counts the input entities. The state holds only those of a rule's kinds.

## Proof

### The recorded run

A new fixture, `specification/fixtures/relate/`, holds three entity sets of public facts and one key. Each fact has one settled answer.

- `bands.jsonl`: John Lennon, Paul McCartney, George Harrison, Ringo Starr, Linda McCartney, Denny Laine, Bob Dylan, Tom Petty, Roy Orbison and Jeff Lynne as `person`. The Beatles, Wings and Traveling Wilburys as `band`. Rule file `member-of.json`: `member_of=person:band`, reads `is a member of`. 12 true edges: the four Beatles, Paul McCartney, Linda McCartney and Denny Laine in Wings, and George Harrison, Bob Dylan, Tom Petty, Roy Orbison and Jeff Lynne in the Traveling Wilburys.
- `cities.jsonl`: Paris, Lyon, Marseille, Tokyo, Osaka, Kyoto, São Paulo and Rio de Janeiro as `city`. France, Japan, Brazil, Canada, Egypt, Kenya, Peru, Norway, Vietnam and Chile as `country`. Rule file `located-in.json`: `located_in=city:country`, reads `is located in`. 8 true edges.
- `cities-plus.jsonl`: `cities.jsonl` plus Madrid, Berlin and Rome as `city`. The same 8 true edges.
- `key.jsonl`: three audit key lines, ids 1 to 3, in `relate`'s edge shape.

Each set is built to break today's planner. In `bands`, persons outnumber bands, so each person asks and keeps at most one band. Paul McCartney and George Harrison lose one each. In `cities`, countries outnumber cities, so each country keeps at most one city, and at most 3 of 8 edges survive. In `cities-plus`, three unrelated cities make the cities ask, so all 8 can return. That is item 2's flip on facts with one answer.

One run is `relate @RULES --jsonl --details --record DIR` over the three sets in order, which writes three result lines. Three runs make the proof. `audit --match strict` grades each run's three lines against `key.jsonl`.

Bars, on every run:

- Matched at least 26 of the 28 key edges, with at most 3 extra.
- `cities` and `cities-plus` each match at least 7 of 8.
- The `cities` and `cities-plus` lines differ by at most one edge between the 8 cities and 10 countries they share.

No experiment measured this wording on these sets. Local experiment 237's yes/no arm found 89 of 89 true edges with 11 extra over seven sets. The bars leave room for one run's noise. The recording sets the first figure. The fixture's README pins each replayed audit row exactly, under "Replayed runs", and the spec rung runs it.

### Outside-in tests

1. **The recorded runs.** The fixture README's blocks replay the three runs and pin each audit row. They protect the whole path on real answers. Restoring the choice drops `cities` below 7 of 8 in any new recording. No existing test grades `relate` against a key. No hook is needed.
2. **One-to-many, loopback.** Rows 1 and 2 in `tests/backend/relate.rs`. The test pins the three lines and the request counts. It replaces `choice_and_h_edges_keep_ruled_order_and_endpoint_shape`. It protects every true edge and invariance under an unrelated entity. Restoring the choice prints no line for row 1. Today's tests pin the choice itself. It drives the real command against a listener.
3. **Plans, by dry run.** Rows 3, 5, 9 to 13 and 15, and row 14 through a loopback count, in `tests/backend/relate.rs` and `tests/backend/relate/ceiling.rs`. The test pins row 3's whole plan and the request counts of the rest. It protects the shared state, the filter to rule kinds, the 400 limit at every address, the profile rules and plan version 2. A per-rule state, a kept `Paris`, a missing limit or a limit only at the built-in address each fail a row. No existing test pins one state for several rules.
4. **Retained contracts.** The detail tests `an_h_entry_at_the_cut_is_accepted_and_bare_partial_output_exits_six`, `mixed_logical_failure_prints_details_and_exits_six`, `the_detailed_question_digest_hashes_the_printed_line_question` and `no_valid_logical_answer_exits_four_without_output` pass unchanged in their assertions, over rebuilt exchanges. So do `secrecy_relate.rs`, the empty-input test and the cache test. Ticket 0147's recognize replays pass with no recording changed.

### Edge-case table

One table test in `core/relation/tests.rs` replaces the choice tests. Each row gives entities, rules and wording, and pins every question's exact text and its pair. The rows are 4 to 8 and 20 above, and a row where an entity of an unnamed kind consumes no id.

### Deleted tests

- `core/relation/experiment_239.rs` and `tests/fixtures/recognize-239/`'s question and replay files. They replay local experiment 239's choice questions, which no longer exist. `input.json` stays for the ceiling tests.
- `fixed_option_boundary_keeps_255_and_falls_back_at_256` and `exact_relation_state_and_h_wording_use_stable_ids` in `boundary_tests.rs`. The table covers ids and wording.
- `cross_kind_choice_asks_the_larger_side_with_smaller_side_options`, `a_choice_puts_the_blank_on_the_side_the_options_fill`, `same_kind_h_asks_unordered_or_ordered_pairs_and_never_self_pairs` and `wildcards_expand_to_concrete_kinds_in_first_seen_order` in `tests.rs`. The table replaces them.
- `a_backend_profile_option_limit_falls_back_per_concrete_relation_in_the_plan`, `successful_and_failed_choice_entries_keep_the_exact_option_a_shape` and `a_target_side_asker_keeps_its_roles_and_the_declared_edge_direction`. No fallback or choice entry remains.

### Changed tests

- `a_wildcard_rule_expands_to_concrete_kinds_in_first_seen_order` becomes row 6's order.
- `dry_run_reports_the_exact_plan_and_the_digest_a_real_run_sends` pins plan version 2.
- `a_relation_of_180_names_keeps_its_reply` becomes row 14. No relation reply can now reach 1 MiB. `tests/public_controls.rs` keeps ticket 0132's reply limit.
- `tests/backend/relate/at_once.rs` reads a request's rule from its questions, because the state no longer names one.
- `tests/backend/relate/ceiling.rs` counts requests from the top-level `request_count` and pins the new byte lists, as ticket 0154 leaves the file.
- Conformance cases 51 and 52 regenerate their synthetic exchanges offline. `spec/fixtures/relate-partial/` and demo 45's recording are rebuilt offline, because their probabilities are illustrative.

### Deliberate breaks

Each break is made, run and reverted. Each must turn the named test red.

| Break | Test that turns red |
| --- | --- |
| Restore the choice for different kinds | Test 2, row 1 |
| Keep one state per rule | Test 3, row 5 |
| Keep entities of kinds no rule names | Test 3, row 3 |
| Put `relation` back in the state | Test 3, row 3 |
| Ask relate with `Does the text itself state that` | Test 3, row 3 |
| Change recognize's lead words | Test 4, ticket 0147's recognize replays |
| Drop the 400 limit | Test 3, row 9 |
| Apply the limit only at the built-in address | Test 3, row 14 |
| Let `max_questions` raise the limit | Test 3, row 12 |
| Fail the run on one failed answer | Test 4, `mixed_logical_failure_prints_details_and_exits_six` |
| Put an `either` edge in kind order | Table, row 4 |
| Pair an entity with itself | Table, row 6 |
| Keep `method` in the plan's rules | Test 3, row 3 |

## Recordings and the paid run

Ian authorizes each run before it starts. Each goes through `sdlc/scripts/live` under its token cap, on the public facts above. Costs use the dry-run bytes, 0.516 input tokens a byte and $0.042 a million input tokens.

| Run | When | Questions | Estimated input tokens | Cap | Estimated cost |
| --- | --- | --- | --- | --- | --- |
| A. Today's choice, three runs, from main's binary | Before the build starts | 31 a run | about 26,000 | 60,000 tokens | about $0.001 |
| B. The pairs, three runs, recorded | After tests 2 to 4 pass | 220 a run | about 33,000 | 60,000 tokens | about $0.0014 |

The total cap is 120,000 tokens, about $0.005. Run A is local experiment evidence. Its answers stay unpushed, and the build record cites its figures. It measures today's recall and tokens on the same sets, as ADR 0054 item 5 asks. Run B's recordings join the fixture. The build record also reports run B's input tokens against its request bytes. No other recording is re-made, because every other relate exchange in the repository is synthetic.

## Pages

In the commit that changes each behavior:

- `specification/relate.md`: line 5 gains the sentence "An edge comes from the model's knowledge of the names, not from any text; for edges a text states, use `recognize --relation`." Lines 44 and 46 state the pair method, the order and the shared state. Lines 50 to 54 state plan version 2. Line 60 drops the choice entry.
- `specification/backends.md`: line 17 drops the relate choice fallback, and says relation requests hold at most 400 questions at every address. Line 21 drops "It never turns a choice into yes/no questions."
- `specification/result.md` line 42 and `specification/channels.md` line 115: no choice entries, no method or fallback, plan version 2.
- `specification/check.md` line 29: choice questions of `recognize` only.
- `specification/fixtures/relate/README.md`: the sets, the key, their sources and the replayed rows.
- `spec/relate.md` and demo 45: plan version 2 and rebuilt exchanges.
- `sdlc/planning/relate-design.md`: the four sections ADR 0057 lists gain a superseded marker.
- ADR 0019: one note after its amendment, saying relations ask no choice since ADR 0057, so a choice's total no longer limits relation edges.
- ADR 0057: its status names the build.
- `CHANGELOG.md`: one line naming the pair method, plan version 2 and the changed request bodies.
- One issue for the marketing lead. The site's Beatles `relate` examples and the bench's relate example replay old exchanges and will miss. The bench's relate F1 of 0.72 measures today's choice.

## Order with other tickets

- **Ticket 0154** opens `engine/prepared_request.rs`, `cli/relate.rs`, `tests/backend/relate/ceiling.rs`, `relate.md` and `backends.md`. The build starts after 0154 lands, or rebases on it before it touches those files. The page edits land after 0154's.
- **Ticket 0155** opens `conformance`, `backends.md` and `check.md`. **Ticket 0156** opens `channels.md`. The coordinator orders each shared page.
- Ticket 0147 has landed. Ticket 0165 has landed, and `diff` reads `relate` lines by their edge shape, which stays.

## Ratchet

Main's ceiling is 72,168 lines at `a057c594`. The build reads it when it starts. The estimate is -525 against it, from -750 to -300.

- Shrinks: the choice planner and its expansion (about 150 lines), the choice assembler (about 35), the fallback (about 30), the relate facade's per-relation plumbing (about 30), the choice detail entry (about 90), the dry-run fallback (about 12), and the choice and fallback tests, experiment 239's replay included (about 480).
- Grows: the generic pair planner and its wording enum (about 20), the shared split helper (about 10), the relate facade on pairs (about 20), the edge-case table (about 90), tests 2 and 3 (about 150), and the rewritten tests (about 10).
- Before adding, the builder looks for duplication between the two facades' pair handling and removes it.

## Stop rules

1. Stop if the build would raise the ceiling above main's ceiling when the build starts. Report what grew.
2. Stop before run A or run B until Ian authorizes it.
3. Stop if run B misses a bar. Report the audit rows and the missed and extra edges, grouped by set. Do not tune the wording to the key.
4. Stop if any recognize recording or request body changes.
5. Stop if a deliberate break stays green.
6. Stop if 0154 has not landed and the build needs a file in its list.
7. Stop if a design rule here needs a ruling ADR 0057 does not give. Report it with options.

## Routing

Owner and builder: Claude. Reviewer: a fresh read-only Claude session for the design and for the diff. The change alters public output and request bodies, so the diff review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 2; proof 2; cost of error 2; total 9. The risk is a changed request body across every surface. The conformance cases and the recorded bars guard it.

## Deferred gaps

- **A large set's cost.** A 255-entity cross-kind set costs about twice today's tokens. Nothing narrows candidates before asking. `2026-09-25-recognize-and-relate-scale-and-shape.md` keeps that work.
- **The 400 limit's repeated state.** Dropping it would save 10% to 20% on the largest sets. Ian's ruling holds it.
- **A state over the backend's token limit.** 255 long names can pass 65,536 input tokens in one request and exit 4, as today.
- **Precision on one-target facts.** Local experiment 237 found the choice more precise where each source has one target, 0.90 against 0.74 at 0.5 on 32 edges. At each method's best cut they were level. `audit` suggests a cut from a key, and the page points to it. No setting brings back the choice.
- **The `either` wording.** No experiment measured it for knowledge questions, and the recorded sets use no `either` rule.
- **Mentions and sentence distance.** `2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md` concerns `recognize` and stays open. `relate` has no mentions.

## Decisions

The owner's calls, which ADR 0057 lists. Ian can overturn each.

1. **One path for both verbs.** `recognize` step 3 and `relate` share the planner, the state and the split. Their questions differ only in lead words.
2. **The shared state folds in.** It falls out of the one path, and local experiment 275 measured it.
3. **The wording `Is it true that i1 READS i2?`.** It mirrors `recognize` step 3's sentence. No experiment measured it. Run B measures it.
4. **The same-kind question changes.** One path leaves no second wording.
5. **A profile lowers the 400 and never raises it**, as `recognize` step 3 does. The ruling's text says "replaces".
6. **The limit applies at every address**, as `recognize` step 3 applies it.
7. **Plan version 2, and `method: "yes_no"` stays in detail entries.** Rules share requests, so a rule's `request_count` has no meaning. Detail entries keep their shape so consumers keep working.
8. **ADR 0019 gains a note and keeps its rule.** The coordinator asked for an amendment to ADR 0019. ADR 0057 amends `relate-design.md`, which holds the planner.
9. **Run A measures today's choice before the build.** It costs about $0.001 and answers ADR 0054 item 5.

## What Ian can overturn

- ADR 0057 as a whole, and each owner call it lists.
- His ruling of 2026-09-26 as it reaches `relate`: pairs for every relation, and 400 questions a request.
- Decisions 1 to 9.
- The fixture's facts, the bars of run B, and the two paid runs with their caps.

## Closes

- Item 2 of `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md`. The issue stays open for its severity-2 and severity-3 items.
- `sdlc/issues/2026-09-26-relate-should-say-its-edges-come-from-the-models-knowledge.md`.
- `sdlc/issues/2026-09-26-relation-requests-carry-one-rule-and-every-entity.md`. Ticket 0147 settled its `recognize` half, and this ticket settles `relate`'s.

## Evidence

- Starts from: Ian's ruling of 2026-09-26, item 5 of ticket 0147 before commit `2e0c6551`. ADR 0056 and ticket 0147, which built the pair path for `recognize` step 3. Item 2 of the architect review 10 issue, with its live and replayed failures. Local experiment 237, where yes/no per pair found 89 of 89 true edges against 79 for the choice. Local experiment 275, where one shared state cut input tokens by 51% to 59% within the noise. Local experiment 284's step-3 figures, 66 to 68 of 70 stated edges, in `sdlc/records/2026-09-26-recognize-three-step-evidence.md`. Main's dry runs at `a057c594` for today's bytes, and relate's 0.516 tokens a byte in `specification/backends.md`.
- Keeps: Rules, entity input, validation, the cut, the edge shape on every surface, the yes/no detail entry, exit 6, the question digest, `--jobs`, the ceiling, profiles, record, replay, cache, and every recognize request body.
- Changes: Every relation asks yes/no pairs. One state per run holds only rule kinds. Questions name their relation with `Is it true that`. Requests hold at most 400 questions at every address. The fallback and choice entries go. The dry run becomes version 2. The page says an edge comes from the model's knowledge.
- Proof: Three recorded runs over public facts graded by `audit` against bars. A loopback one-to-many test with an unrelated entity. Dry-run plans pinned for the state, the limit and the profile rules. Retained detail, secrecy and recognize replay tests. One edge-case table. Thirteen deliberate breaks, each with the test it turns red.
- Defers: A large set's cost, the 400 limit's repeated state, a state over the backend's token limit, precision on one-target facts, the `either` wording, and recognize's mention and distance limits.
