---
flow: build
priority: 147
opens: sdlc/planning/adr/0050-recognize-reads-words-confirms-runs-and-windows-long-texts.md sdlc/planning/adr/0040-split-requests-under-backend-limits.md sdlc/tickets/0080-build-recognize.md sdlc/planning/recognize-design.md specification/recognize.md specification/question-file.md specification/result.md specification/channels.md specification/records.md specification/relate.md specification/backends.md specification/audit.md specification/settings.md sdlc/records sdlc/tickets
---

# 0147: Record the recognize rulings in one ADR

Status: ready. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Ian's recognize rulings of 2026-09-26 become one accepted ADR, and the Settled pages they contradict say so. Every later recognize ticket then cites one ADR item in place of the design issue.

The authority is `sdlc/issues/2026-09-26-recognize-design.md`, which Ian sent to the main builder on 2026-09-26 with the review findings and his rulings applied. This is its ticket R0. The R0 row lists the scope: word rules, `confirm` as the default, stated relations, pieces and windows for long texts, the hard cap, `recognize --jobs` for one document, and the recognize batch shape. The row says R0 lands after batching B0. B0 landed as ticket 0139, ADR 0048. Ian can overturn each ruling, as the design's "Open items" list says.

Ian's goal is a first-class name recognizer, and speed wins over accuracy. R0 writes no code. It changes no behavior, help text, fixture, schema or digest. `recognize` behaves the same after it lands.

## What happens today

The pages and records below state what the rulings change. Each line is at `origin/main` `eba3a72a`.

| Where | Line | What it says today |
| --- | --- | --- |
| ADR 0040 | 12 | "Each chunk repeats the evidence" |
| ADR 0040 | 20 | "Records, evidence text, options, and unrelated calls are never combined or divided." ADR 0048 amended the records part |
| ADR 0040 | 24 | "Other plans and other addresses keep no ceiling." ADR 0048 amended it for batched record plans |
| Ticket 0080 | 53 | Decision 2: "Keep all recognition policy internal … One maximal contiguous run of `IN` words is one candidate" |
| Ticket 0080 | 54 | Decision 3: "Preserve the exact source text through every request" |
| `sdlc/planning/recognize-design.md` | 186 | The 2026-09-23 ruling ships the maximal-run baseline |
| `specification/recognize.md` | 7 | "Long text keeps its complete source context" |
| `specification/recognize.md` | 21 | Today's splitter and the maximal-run rule |
| `specification/recognize.md` | 31 | A relation keeps an edge at its model probability, with no statement about the text |
| `specification/recognize.md` | 45, 47 | `--details` fields, and a failed question failing its input |
| `specification/recognize.md` | 53 | "`request_count` is the exact recognition request count" |
| `specification/question-file.md` | 9 | "Recognition policy has no command or file keys." |
| `specification/question-file.md` | 133 | The recognize canonical order: `verb`, `kinds`, optional `relations`, `threshold`, `relation_threshold`, optional `profile` |
| `specification/result.md` | 44 | `recognize --details` carries no word rules and no `answer.confirm` |
| `specification/channels.md` | 32 | The option lists name no recognize option |
| `specification/channels.md` | 113 | `recognize --dry-run` plans "the first record" |
| `specification/records.md` | 129 | "Another command refuses it outside record mode", so `recognize` on one text refuses `--jobs` |
| `specification/relate.md` | 5 | Nothing says where `relate`'s answers come from |
| `specification/backends.md` | 17 | "Recognition keeps the complete source text in every request" |
| `specification/audit.md` | 165 | `--write` needs each line's digest to match the file's |
| `specification/settings.md` | 90 to 96 | "Settings on the way" names each recognize setting by the design's section, with no ADR |

## Where main stands for R4b and R7

The coordinator asked for the design's R4b and R7 rows to be checked against main.

- **R4b.** Its row depends on R4 and batching J1. J1 landed as ticket 0143. `Engine::ask_chunks` now sends one text's pieces together at the engine's width. `recognize` over one text still refuses `--jobs` through `schedule::jobs_of` (`cli/recognize.rs:76`), with "--jobs bounds the requests in flight, and a single text sends one request" (`cli/failure.rs:440`). A profile's `max_questions` already splits a text at a loopback address today. So R4b only needs the flag, and it needs R0 alone, not R4. Ticket 0143's decision 5 names R4b as the owner of the flag. Design section 6's sentence "Today `Engine::ask_chunks` sends them one after another" is stale.
- **R7.** Its row depends on R3 and batching B4 and B5. B3 landed as ticket 0144 and plans batches in `core/batch.rs`. Its `Batcher` gives each record one quoted question: `The text is ` and the record. A recognize record brings many word questions and needs the prefix `In record K of the list. `. So R7 reuses the close rule and the evidence object, and extends the planner for a record that carries many questions. B4 is in flight as ticket 0146. It moves `batch` into `settings.md`'s table and says `THINKTHEN_BATCH` is ignored on every verb but `decide`, `filter` and `rank`, so R7 updates that row. B5 has no ticket yet. Ticket 0145, S1, lists `recognize` with a floor of 2 requests and owner R7. The R7 row stands.

## Design

### The ADR

R0 writes `sdlc/planning/adr/0050-recognize-reads-words-confirms-runs-and-windows-long-texts.md`. Main's last ADR is 0049, and no branch holds 0050.

Status line: accepted 2026-09-26 on Ian's rulings, through ticket 0147, built by the recognize tickets it names, Ian can overturn each item. Its sections:

1. **Context.** Today's splitter reaches 141 of the key's 168 names (evidence section 11). Unstated edges scored 0.55 to 0.80, and asked about the text they scored 0.02 to 0.05 (section 10). Each word costs about 300 input tokens and 1,150 request bytes, Jev refuses a request over 65,536 input tokens, and a text over about 200 words fails today (section 13). Ian's rulings. "Section N" means that section of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`.
2. **Decision.** One numbered item per rule, each copied from the design section it cites:
   1. One request finds and labels the names: a yes/no question and, with two or more kinds, a pick-one kind question per word, each with a five-word snippet. No BILOU scheme, no span questions (design section 1).
   2. The word rules: the five lists and their defaults, the fixed rules, each list adding to its default, `defaults: false`, the MUC-7, ACE and CoNLL convention, and the limits of 100 entries of at most 32 characters with no white space. A bad entry exits 2 on the command line and 5 in a file, by `question-file.md` line 33. `keep` and `infixes` reach every surface. `prefixes`, `suffixes`, `trim` and `defaults` live only in the question file. It lifts the policy-key ban for the lists, `boundary` and `window` (section 2).
   3. The `boundary` setting: `confirm` by default with its measured question, `ONE` and `SPLIT`, one `confirm` request per piece after the word answers, a run belonging to its first word's piece, and `confirm` questions splitting over the ceiling. `run` keeps the 2026-09-23 maximal-run baseline (section 3).
   4. Kinds, the kind vote, `strength` and the 0.5 cut stay (section 4).
   5. A recognize relation means the text states it: the two new wordings, the whole text as relation evidence, exit 4 over Jev's evidence limit, and standalone `relate` keeping its wording. R5 measures the pick-one wording before it lands and stops and reports to Ian if design test 7 fails (section 5).
   6. Pieces and windows: the ceiling or a profile's limits fill a piece, a window of up to 200 words each side, `--window N` from 0 to 5,000, the evidence as a byte range of the original text, names forming after every request, a failed request failing the text, offsets unchanged, and one piece at an address with no ceiling and no profile. The known risk of a far clue (section 6).
   7. The hard cap: 600,000 bytes, exit 2 before any request at every address, no option, the message naming size and limit and echoing no text. In record mode it refuses that record, as a record over 16 MiB is refused (section 6).
   8. `recognize --jobs N` on one document, 1 to 32, default 4. Ticket 0143 already runs the pieces at once, and only the flag is missing (section 6, "Concurrency").
   9. The recognize batch shape: ADR 0048 items 2 to 6 and 13 apply. Evidence `{"records":[T1,…,TN]}`, the word-question prefix `In record K of the list. `, no quote prefix, a batch of one sending today's bytes, a text too big for a batch of one going alone and splitting by item 6, and until R4 lands going alone as one request with the whole text, a batch's `confirm` questions in one request after its word answers, relations per text, batching on by default, design test 9 reporting and gating nothing. `recognize` keeps one document as its default input, and `--field` with no framing flag reads one JSON document (section 7 and "Many short texts, one per line").
   10. Question identity: the new canonical order, `words` holding the five lists in order, every recognize digest changing, `audit --write` refusing old runs with its existing sentence, and `batch` and `jobs` staying out of the digest ("Defaults and what a caller can change").
   11. Output: bare output unchanged, `question.words`, `question.boundary`, `question.window`, `answer.confirm`, and `meta.batch` and `--facts` by ADR 0048 items 9 and 10 ("Output and run facts").
   12. Dry run: `request_count` counts the requests formed before any answer, `confirm_questions_upper_bound` after it, at 0 with one kind or under `run` and otherwise floor((W + 1) / 3) for each text, summed over a batch's texts, each split request naming its evidence offsets, and the first batch under a framing flag ("Output and run facts").
   13. Secrecy: no new failure line echoes a text, list entry, name or key, and the `Debug` rules ("Secrecy").
3. **What this amends.** The table under "What happens today", with the amendment each line gets.
4. **Which ticket builds each item.** One row per item, naming the design's labels. R2, R3 and R4 build items 2, 3 and 6 on the command and question file. R6 carries `keep`, `infixes`, `boundary` and `window` to the libraries and SQL. R5 builds item 5 and removes the `relate.md` marker. R4 builds item 7. R4b builds item 8 and depends on R0 alone. R7 builds item 9 after batching B4 and B5. Items 10 to 13 split across R2, R3, R4 and R7 by the key or path each adds.
5. **What Ian can overturn.** The design's fifteen "Open items", in its order, plus ticket 0147's calls: decisions 3 to 7, item 7's record-mode refusal, and item 9's too-big text going alone, as one whole-text request until R4 lands. The status line says Ian can overturn each item and every rule the ADR copies.

The ADR copies each rule. It does not re-argue it. The design issue stays the argument, and the evidence record stays the measurement.

### The amendments

Accepted ADRs keep their text. Each named ADR line gains a marker, and ADR 0040 gains one section `## Amendment, 2026-09-26: ADR 0050 windows recognize texts` of at most four sentences. Settled pages keep today's sentence and gain the new rule after it as `Not built yet, by ADR 0050 item N: …`, the form ADR 0048 set. Each page's status line adds `amended by ADR 0050`. The ticket that builds item N deletes the old sentence and the marker in the same commit.

Build rule: place each amendment by the sentence the table under "What happens today" quotes, not by its line number. Merge `origin/main` before editing and re-read every line number, because ticket 0146 may land first and move lines.

| Where | Line | Amendment |
| --- | --- | --- |
| ADR 0040 | 12 | Marker `(Amended by ADR 0050, below.)` |
| ADR 0040 | 20, 24 | The existing markers become `(Amended by ADRs 0048 and 0050, below.)` |
| ADR 0040 | new section at the end | The built-in ceiling reaches recognize word and `confirm` questions. A recognize piece carries its words and a window in place of the whole text. Relation requests keep the whole text. Ian can overturn this |
| Ticket 0080 | 53, 54 | A trailing marker on each: decision 2 lifted for the word lists, `boundary` and `window` by ADR 0050 items 2 and 3, and decision 3 replaced for word and `confirm` requests by item 6 |
| `sdlc/planning/recognize-design.md` | 186 | A trailing marker: kept as `boundary: run` by ADR 0050 item 3, and `confirm` is the default |
| `recognize.md` | 3 | Status adds ADR 0050 |
| `recognize.md` | new paragraph after 7 | Items 6, 7 and 8: pieces with windows, the hard cap, and `--jobs N` on one text |
| `recognize.md` | new paragraph after 21 | Items 2 and 3: the five word lists with `--word-keep` and `--word-infix`, and `--boundary confirm` as the default with `run` keeping the rule above |
| `recognize.md` | new paragraph after 31 | Item 5: a relation means the text states it |
| `recognize.md` | new paragraph after 45 | Items 9 and 11: record modes fill each request, `--batch 1` sends one text a request, and the new `--details` fields |
| `recognize.md` | 47 | Items 3 and 6: a failed `confirm` question or split request fails the whole text |
| `recognize.md` | new paragraph after 53 | Item 12: the new `request_count` rule, `confirm_questions_upper_bound`, and `words` under the effective rules |
| `question-file.md` | 3 | Status adds ADR 0050 |
| `question-file.md` | new paragraph after 9 | Items 2, 3 and 6: `recognize.words`, `recognize.boundary` and `recognize.window`. The ban lifts for these keys only |
| `question-file.md` | new indented line under 133 | Item 10: the new recognize order, and every recognize digest changes |
| `result.md` | 3 | Status adds ADR 0050 |
| `result.md` | new paragraph after 44 | Item 11: `question.words`, `question.boundary`, `question.window` and `answer.confirm` |
| `channels.md` | 3 | Status adds ADR 0050 |
| `channels.md` | new paragraph after 33 | Items 2, 3, 6 and 8: `recognize` takes `--word-keep`, `--word-infix`, `--boundary` and `--window N`, and `--jobs N` on one document |
| `channels.md` | 113 | Item 12: the first batch under a framing flag, and each split request's evidence offsets |
| `records.md` | 3 | Status adds ADR 0050 |
| `records.md` | 129 | Item 8: `recognize` also accepts `--jobs` for one text, whose pieces are distinct requests |
| `relate.md` | 3 | Status adds ADR 0050 |
| `relate.md` | new paragraph after 5 | One sentence outside the marker, because it is true today: `relate` sends no text, so its answers come from what the model knows of the world. Then the marker, item 5: a `recognize` relation means the text states it, and `relate` keeps its wording. R5 removes the marker |
| `backends.md` | 3 | Status adds ADR 0050 |
| `backends.md` | 17 | Item 6: word and `confirm` requests carry a piece's words and window, and the built-in ceiling splits them. Relation requests keep the complete text |
| `audit.md` | 3 | Status adds ADR 0050 |
| `audit.md` | 165 | Item 10: `--write` refuses a `recognize` run made before the word rules. Rerun and write from the new run |
| `settings.md` | 90 to 96 | Each recognize line ends `Not built yet, by ADR 0050 item N.` The `window` line names tickets R4 and R6 |

`grep -rn "Not built yet, by ADR 0050" specification` lists every leftover. The last recognize ticket to land requires it to come back empty.

## Decisions

Each is the owner's call under Ian's rulings. Ian can overturn any of them.

1. **One ADR, 0050, for every recognize rule.** The R0 row asks for one. Later tickets each cite an item.
2. **Markers and amendments take ADR 0048's form.** Pages keep today's sentence beside `Not built yet, by ADR 0050 item N: …`, and accepted ADRs gain a marker and a dated section. One fixed phrase lets one `grep` list every leftover. A page stays true today.
3. **The recognize settings stay under "Settings on the way" and are not table rows.** `sdlc/scripts/settings` fails a table row whose flag is in no command's help, and `--word-keep`, `--word-infix`, `--boundary` and `--window` are in none yet. Plant (a) proves it. `settings.md` line 85 already rules that the ticket that lands a setting moves its line into the table. R0 adds the ADR item to each line. A table row marked not built would need the check to learn an exception, which weakens it.
4. **R4b depends on R0 alone.** Ticket 0143 already sends a split text's pieces at once, and a profile splits a text today. The ADR's ticket table says so. R4b's proof, a loopback count reaching the job count on a split text, works with a `max_questions` profile before R4 lands. The design issue's R4b row is left as Ian sent it, and the ADR governs.
5. **R6 carries `window` to the libraries and SQL.** The design's settings table gives `window=` and a `window` key on every surface, and no ticket row builds them. R6 already carries the other recognize settings there.
6. **The design's open details get one spelling each.** A batch's `confirm` request carries the batch's evidence object, and each question takes its record's `In record K of the list. ` prefix, as the word questions do. `confirm_questions_upper_bound` follows `request_count` in the plan's key order. It is 0 with one kind or under `run`. Otherwise it is floor((W + 1) / 3) for each text, where W is the text's word count under the effective rules. That is the most runs of two or more words, each separated by another word, that W words can hold. It counts per text, not per piece, because a run belongs to one piece, and a batch sums it over its texts. A table test in R3 can pin it: W of 1, 2, 4 and 5 give 0, 1, 1 and 2. R4 names the two evidence-offset keys of a split request, because the design leaves them open.
7. **Each canonical key enters the digest with the ticket that builds it.** The order is fixed now. R2 adds `words`, R3 `boundary`, R4 `window`. The digests change more than once before 0.1, and no release ships between those tickets.
8. **The design issue stays open and unedited.** It closes when its last ticket lands.
9. **R0 edits no line ticket 0146 edits.** Ticket 0146, B4, opens `records.md`, `backends.md`, `channels.md`, `question-file.md`, `result.md` and `settings.md`. It changes the ADR 0048 markers at `channels.md` 32 and 99, `records.md` 81, 85, 91, 109 and 135, `backends.md` near 21, `result.md` 102 and 103, the `question-file.md` settings table and precedence paragraph, and `settings.md` 87. R0 inserts its `channels.md` paragraph after the blank line 33, not on line 32, and leaves `settings.md` 85 and 87 alone.

## Edge cases

| Case | What R0 does |
| --- | --- |
| Ticket 0146 lands first | The builder merges `origin/main` before the final run, and each R0 hunk sits on a line 0146 does not touch. A conflict means stop rule 6 |
| `settings.md` table | Untouched. The recognize lines stay under "Settings on the way", by decision 3 |
| `question-file.schema.json` and `recognize_file.rs` key lists | Untouched. `sdlc/scripts/settings` reads both, and R2 adds the keys with their parser |
| The design issue | Untouched, by decision 8 |
| A rule the design states and Ian's rulings do not cover | The ADR copies it. The ADR's status line says Ian can overturn each item and every rule it copies, and the rule appears among the design author's calls or ticket 0147's calls |
| A number in the ADR | It names its evidence section or experiment |
| A marked sentence a building ticket forgets to delete | The grep lists it. The ADR's ticket table names the ticket that removes it |
| `recognize.md` line 7 or 21, which several items change | One marker paragraph after each line names every item that reaches it |
| Another branch takes ADR 0050 first | Stop rule 5 |
| The word "decision model" | It never appears. `the_specification_defines_unresolved_once_and_keeps_the_closed_wording` bans it on every specification page |

## Proof

R0 adds no test. It changes no behavior, so a new test would check prose. The building tickets carry the design's tests 1 to 11, each against the real command.

The proof is the review plus the checks that already guard these pages:

- The reviewer reads the ADR against the design's sections and rulings, item by item, and each amended line against the amendment table. Every changed page keeps today's sentence beside its marked rule.
- `grep -rn "Not built yet, by ADR 0050" specification` gives one hit for each marked passage in the amendment table and no other. The reviewer checks each hit names the right item.
- The diff touches only the files in `opens`. Nothing under `crates`, `libraries`, `databases`, `conformance`, `spec`, `site`, `demos` or `specification/fixtures` changes, and `question-file.schema.json` does not change.
- `sdlc/scripts/lint`, which runs the ticket check and the private-name check.
- `sdlc/scripts/settings` after `sdlc/scripts/install`, run as `env -u THINKTHEN_API_KEY python3 sdlc/scripts/settings`, reads the edited `settings.md` against the installed help.
- `the_specification_defines_unresolved_once_and_keeps_the_closed_wording`, run alone as `flock -o /run/user/1000/thinkthen-heavy.lock env -u THINKTHEN_API_KEY cargo test -p thinkthen --test version the_specification_defines_unresolved_once`.
- `grep -rnF -e "complete source text" -e "Recognition policy has no command" -e "exact recognition request count" -e "refuses it outside record mode" spec` comes back empty before and after. No executable page quotes an amended sentence.

| Guard | Planted fault that turns it red |
| --- | --- |
| `sdlc/scripts/settings` | (a) Move the `keep` line into the table as a row whose flag cell is `--word-keep`. The check fails it with `--word-keep is in a row and in no help` |
| The specification wording test | (b) Write "decision model" into the new `recognize.md` paragraph after line 21 |
| The ticket check | (c) Drop the `- Defers:` item from this ticket's Evidence section. This plant tests the ticket file, not the ADR or the pages |

The four questions, for the guards R0 relies on and adds nothing to:

- **What behavior does each protect?** The settings check keeps unbuilt settings out of the table, so the table names only what the help offers. The wording test keeps banned terms off the pages R0 edits. The ticket check keeps this ticket's Evidence section whole.
- **What credible regression fails it?** Writing the recognize settings as table rows before their flags exist, as the coordinator's first request read. A banned term in a new marker. A ticket missing an Evidence part.
- **Why does no existing test catch it?** Each is an existing guard. R0 adds nothing a new test would reach.
- **Does it need a test-only hook?** No.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- ADR 0050: at most 120 nonblank lines.
- ADR 0040: at most 8 added or changed lines.
- Ticket 0080 and `recognize-design.md`: at most 3 changed lines in all.
- The nine specification pages: at most 40 added or changed lines in all.
- The build record: at most 40 nonblank lines.
- Nothing else changes. No code, test, fixture, schema, ratchet or dependency. The `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget.
2. Stop if an amendment needs a ruling Ian did not give and the design does not state. Report it with options.
3. Stop if a plant stays green, or a rung goes red for a reason page text cannot fix.
4. Stop before touching `specification/settings.md`'s table, line 85 or line 87, `question-file.schema.json`, `recognize_file.rs`, `site/`, or the design issue.
5. Stop if another branch claims ADR 0050 before this one lands. The coordinator renumbers.
6. Stop if ticket 0146 or any other in-flight ticket edits a line R0 edits. The coordinator orders the two.

## Scope and exclusions

Excluded: any code. The `settings.md` table rows, by decision 3. The schema and the recognize file's key lists, which R2 changes with its parser. The design issue. The recognize how-to page, which R8 writes. `site/`, which the website agent owns.

## Routing

Owner and builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for the diff. No ceiling moves and no public surface widens.

## Complexity

Contract 2; state and timing 0; reach 2; proof 0; cost of error 1; total 5. Final level: 1. The risk is an amendment that misstates a ruling. The item-by-item review guards it.

## Deferred gaps

- The marked sentences stay on the pages until the tickets that build their items land. The last recognize ticket empties the grep.
- The recognize settings move into the `settings.md` table with R2, R3, R4, R4b and R6.
- `question-file.schema.json` and `recognize_file.rs` gain `words`, `boundary` and `window` with R2, R3 and R4.
- The design issue's R4b row still names R4 as a dependency, and its section 6 still says pieces go one at a time. The ADR's ticket table governs.
- The evidence-offset key names of a split request wait for R4.
- `recognize` over records under B4's `batch` row waits for R7.
- Windowed relations stay a known gap, as the design says.

## What Ian can overturn

- Every ruling and author's call the ADR copies. The design's "Open items" list names fifteen.
- Decision 3: the recognize settings under "Settings on the way", not as marked table rows.
- Decision 4: R4b depending on R0 alone.
- Decision 5: R6 carrying `window`.
- Decision 6: the spellings of the batched `confirm` request and the dry-run bound.
- Decision 7: each digest key entering with its building ticket.
- ADR item 7: a text over the cap in record mode refuses that record.
- ADR item 9: a text too big for a batch of one goes alone, as one whole-text request until R4 lands. R7 keeps its dependencies.

## Closes

None. `sdlc/issues/2026-09-26-recognize-design.md` stays open until its last ticket lands.

## Evidence

- Starts from: Ian's rulings and the R0 row in `sdlc/issues/2026-09-26-recognize-design.md`, sent 2026-09-26. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` sections 10 to 13, from experiments 265, 267, 270 and 271. Ticket 0139 and ADR 0048, which set the form, and ADR 0048's "What recognize R0 may rely on". Tickets 0143 and 0144 on main, and tickets 0145 and 0146 read from their branches, for the R4b and R7 check. The page lines in "What happens today" at `origin/main` `eba3a72a`.
- Keeps: Every behavior, help line, fixture, schema, digest and test. Every accepted ADR sentence, marked and not deleted. Every Settled page's current sentence, beside its marked replacement. The `settings.md` table.
- Changes: A new ADR 0050 holds the recognize rulings. ADR 0040, ticket 0080 and the 2026-09-23 ruling carry markers, and ADR 0040 gains a dated amendment section. Nine specification pages state each new rule beside today's, under the marker. The `settings.md` recognize lines cite their ADR items.
- Proof: The item-by-item review against the design and the amendment table. The marker grep and the `spec/` grep. `lint`, the settings check and the specification wording test, with plants (a) to (c) each turning one red.
- Defers: The table rows to R2, R3, R4, R4b and R6. The schema and key lists to R2 to R4. The evidence-offset key names to R4. Removing each marker to the ticket that builds its item. Windowed relations.
