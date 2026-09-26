---
flow: build
priority: 147
opens: sdlc/planning/adr/0050-recognize-reads-words-confirms-runs-and-windows-long-texts.md sdlc/planning/adr/0040-split-requests-under-backend-limits.md sdlc/tickets/0080-build-recognize.md sdlc/planning/recognize-design.md sdlc/planning/relate-design.md specification/recognize.md specification/question-file.md specification/result.md specification/channels.md specification/records.md specification/relate.md specification/backends.md specification/audit.md specification/settings.md sdlc/records sdlc/tickets
---

# 0147: Record the recognize rulings in one ADR

Status: ready for review. The coordinator accepted an earlier version on 2026-09-26. This version answers the two severity 1 findings of architect review 10, and a fresh read-only reviewer re-accepts it before it is built. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Ian's recognize rulings of 2026-09-26 become one accepted ADR, and the Settled pages they contradict say so. Every later recognize ticket then cites one ADR item in place of the design issue.

The authority is `sdlc/issues/2026-09-26-recognize-design.md`, which Ian sent to the main builder on 2026-09-26 with the review findings and his rulings applied. This is its ticket R0. The R0 row lists the scope: word rules, `confirm` as the default, stated relations, pieces and windows for long texts, the hard cap, `recognize --jobs` for one document, and the recognize batch shape. The row says R0 lands after batching B0. B0 landed as ticket 0139, ADR 0048. Ian can overturn each ruling, as the design's "Open items" list says.

The ADR also records two architect's calls. They answer items 1 and 2 of `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md`, whose detail sits in local experiment 273, report 10. Item 1 found that one kind gives that kind to every name. Item 2 found that a different-kind relation keeps at most one edge per asking name. Both are wrong answers at high confidence with no warning, so both break Ian's goal of a first-class recognizer. The calls are ADR items 4 and 5 and decisions 10 and 11 below. Ian did not rule on them, and he can overturn each.

Ian's goal is a first-class name recognizer, and speed wins over accuracy. Batching fills each request. R0 writes no code. It changes no behavior, help text, fixture, schema or digest. `recognize` and `relate` behave the same after it lands.

## What happens today

The pages and records below state what the rulings and the two calls change. Each line is at `origin/main` `c490f082`.

| Where | Line | What it says today |
| --- | --- | --- |
| ADR 0040 | 12 | "Each chunk repeats the evidence" |
| ADR 0040 | 20 | "Records, evidence text, options, and unrelated calls are never combined or divided." ADR 0048 amended the records part |
| ADR 0040 | 24 | "Other plans and other addresses keep no ceiling." ADR 0048 amended it for batched record plans |
| Ticket 0080 | 53 | Decision 2: "Keep all recognition policy internal … One maximal contiguous run of `IN` words is one candidate" |
| Ticket 0080 | 54 | Decision 3: "Preserve the exact source text through every request" |
| `sdlc/planning/recognize-design.md` | 176 | The 2026-09-21 ruling's table: "A caller who wants names without kinds gives one kind" |
| `sdlc/planning/recognize-design.md` | 186 | The 2026-09-23 ruling ships the maximal-run baseline |
| `sdlc/planning/relate-design.md` | 86 | Different-kind relations ask a choice from the larger side, and "Every non-`none` option at or above the cut becomes an edge" |
| `sdlc/planning/relate-design.md` | 90 to 94 | "Exact option and backend-profile fallback": a choice over 255 options or over a profile's limit falls back to yes/no pairs |
| `sdlc/planning/relate-design.md` | 150 | "Several real candidates may be accepted" |
| `specification/recognize.md` | 7 | "One kind is assigned locally with probability one." "Long text keeps its complete source context" |
| `specification/recognize.md` | 21 | Today's splitter and the maximal-run rule |
| `specification/recognize.md` | 27 | A run takes 1 through 20 distinct nonblank kinds, with no reserved kind name |
| `specification/recognize.md` | 31 | A relation keeps an edge at its model probability, with no statement about the text |
| `specification/recognize.md` | 39 | Different-kind relations use one choice per member of the larger side, and "Every option at or above the cut becomes an edge" |
| `specification/recognize.md` | 45, 47 | `--details` fields, and a failed question failing its input |
| `specification/recognize.md` | 53 | "`request_count` is the exact recognition request count" |
| `specification/question-file.md` | 9 | "Recognition policy has no command or file keys." |
| `specification/question-file.md` | 133 | The recognize canonical order: `verb`, `kinds`, optional `relations`, `threshold`, `relation_threshold`, optional `profile` |
| `specification/result.md` | 44 | `recognize --details` carries no word rules and no `answer.confirm` |
| `specification/channels.md` | 32 | The option lists name no recognize option |
| `specification/channels.md` | 113 | `recognize --dry-run` plans "the first record" |
| `specification/records.md` | 129 | "Another command refuses it outside record mode", so `recognize` on one text refuses `--jobs` |
| `specification/relate.md` | 5 | Nothing says where `relate`'s answers come from |
| `specification/relate.md` | 46 | "Different-kind relations use a choice from the larger side to the smaller side plus `none`" |
| `specification/relate.md` | 60 | `answer.questions` holds choice entries with candidates and a `pick` |
| `specification/backends.md` | 17 | "Recognition keeps the complete source text in every request", and a cross-kind relation choice falls back to yes/no pairs |
| `specification/audit.md` | 165 | `--write` needs each line's digest to match the file's |
| `specification/settings.md` | 90 to 96 | "Settings on the way" names each recognize setting by the design's section, with no ADR |

## The two review findings, checked on main

The coordinator asked for both findings to be checked against main's code with no paid call. The check built main's binary at `c490f082` and replayed forged answers offline with the review's own scripts. It sent nothing and read no key.

- **One kind.** `kind_questions` returns no question when the run has one kind (`core/recognize.rs:149`). `token_answers` then gives that kind probability 1.0 for every word (`engine/facade/recognize.rs:219` to `231`). The detection question names every kind of named entity (`core/recognize.rs:7`), so every detected name takes the one kind. Replaying the review's real detection answers for "Ringo Starr wrote Octopus's Garden on a boat off Sardinia, and the band recorded it at Abbey Road Studios for the album Abbey Road." under `recognize person` printed all five names as `person`, at strengths 0.98 to 1.0. The plan held 26 detection questions and 0 kind questions.
- **One edge per asking name.** `choice_plan` asks from the larger side, and from the source side on equal counts (`core/relation.rs:194`). A choice's probabilities total one, by ADR 0019. `relate wrote=person:song` over 3 persons and 3 songs, with answers that split 0.98 evenly over John Lennon's three true songs, printed no edge. Adding a fourth, unrelated song made the song side ask, and all three edges printed at 0.98.

Dry runs of the same binary priced both fixes. They measure bytes, not tokens.

| Run | Questions | Requests | Bytes |
| --- | --- | --- | --- |
| `recognize person`, 26 words | 26 | 1 | 16,700 |
| `recognize person organization` | 52 | 1 | 28,759 |
| `recognize person organization place` | 52 | 1 | 29,097 |
| `recognize person organization place song` | 52 | 1 | 29,409 |
| `relate wrote=person:song`, 3 by 3, choice | 3 | 1 | 1,271 |
| Same, forced to yes/no pairs by a profile's `max_options` | 9 | 1 | 1,100 |
| 5 by 20, choice then pairs | 20, then 100 | 1, then 1 | 8,705, then 9,020 |
| 10 by 50, choice then pairs | 50, then 500 | 1, then 1 | 30,584, then 42,245 |
| 5 by 180, choice then pairs | 180, then 900 | 1, then 1 | 75,998, then 79,856 |

## Where main stands for R4b and R7

The coordinator asked for the design's R4b and R7 rows to be checked against main.

- **R4b.** Its row depends on R4 and batching J1. J1 landed as ticket 0143. `Engine::ask_chunks` now sends one text's pieces together at the engine's width. `recognize` over one text still refuses `--jobs` through `schedule::jobs_of` (`cli/recognize.rs:76`), with "--jobs bounds the requests in flight, and a single text sends one request" (`cli/failure.rs:440`). A profile's `max_questions` already splits a text at a loopback address today. So R4b only needs the flag, and it needs R0 alone, not R4. Ticket 0143's decision 5 names R4b as the owner of the flag. Design section 6's sentence "Today `Engine::ask_chunks` sends them one after another" is stale.
- **R7.** Its row depends on R3 and batching B4 and B5. B3 landed as ticket 0144 and plans batches in `core/batch.rs`. Its `Batcher` gives each record one quoted question: `The text is ` and the record. A recognize record brings many word questions and needs the prefix `In record K of the list. `. So R7 reuses the close rule and the evidence object, and extends the planner for a record that carries many questions. B4 is in flight as ticket 0146. It moves `batch` into `settings.md`'s table and says `THINKTHEN_BATCH` is ignored on every verb but `decide`, `filter` and `rank`, so R7 updates that row. B5 has no ticket yet. Ticket 0145, S1, lists `recognize` with a floor of 2 requests and owner R7. The R7 row stands.

## Design

### The ADR

R0 writes `sdlc/planning/adr/0050-recognize-reads-words-confirms-runs-and-windows-long-texts.md`. Main's last ADR is 0053. ADRs 0051 and 0052 sit on the branches of tickets 0154 and 0155, and no branch holds 0050.

Status line: accepted 2026-09-26 on Ian's rulings and two architect's calls, through ticket 0147, built by the recognize tickets it names, Ian can overturn each item. Its sections:

1. **Context.** Today's splitter reaches 141 of the key's 168 names (evidence section 11). Unstated edges scored 0.55 to 0.80, and asked about the text they scored 0.02 to 0.05 (section 10). Each word costs about 300 input tokens and 1,150 request bytes, Jev refuses a request over 65,536 input tokens, and a text over about 200 words fails today (section 13). Local experiment 273, report 10, found that one kind labels every name that kind and that a different-kind relation keeps at most one edge per asking name. Ticket 0147 checked both on main's code. Ian's rulings. "Section N" means that section of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`.
2. **Decision.** One numbered item per rule, each copied from the design section it cites or from this ticket's decision:
   1. One request finds and labels the names: a yes/no question and a pick-one kind question per word, each with a five-word snippet. No BILOU scheme, no span questions (design section 1). Item 4 makes the kind question apply at one kind too.
   2. The word rules: the five lists and their defaults, the fixed rules, each list adding to its default, `defaults: false`, the MUC-7, ACE and CoNLL convention, and the limits of 100 entries of at most 32 characters with no white space. A bad entry exits 2 on the command line and 5 in a file, by `question-file.md` line 33. `keep` and `infixes` reach every surface. `prefixes`, `suffixes`, `trim` and `defaults` live only in the question file. It lifts the policy-key ban for the lists, `boundary` and `window` (section 2).
   3. The `boundary` setting: `confirm` by default with its measured question, `ONE` and `SPLIT`, one `confirm` request per piece after the word answers, a run belonging to its first word's piece, and `confirm` questions splitting over the ceiling. `run` keeps the 2026-09-23 maximal-run baseline (section 3). `none` counts as a kind for `confirm`, so a run whose words change between a kind and `none` is asked.
   4. Kinds decline. Every run asks the kind question, at one kind as at several. Its options are the caller's kinds in order, then `none`, described as "Part of a name of none of the listed kinds." A name whose kind comes out `none`, by the vote and its fallback, is not printed. The kind vote, `strength` and the 0.5 cut stay (section 4), with `none` counted as one more kind. `none` is reserved: a kind named `none` exits 2 on the command line and 5 in a file, and the message echoes no text. A caller who wants every name gives the defaults, or one kind whose description covers every name. This is decision 10.
   5. A relation asks one yes/no question per pair, for every concrete relation, in `recognize` and in `relate`. A different-kind relation asks each source and target pair in the declared direction, as a same-kind relation already does, and `--either` asks each pair once. No choice, no asking side and no option-ceiling fallback remain. `relate --dry-run` keeps the `method` and `fallback` keys, always `yes_no` and null. A recognize relation means the text states it: its pair question reads "Does the text state that the relation holds from i1 to i2?", the whole text is its evidence, and a relation request over Jev's evidence limit exits 4. Standalone `relate` sends no text and keeps "Does the relation hold from i1 to i2?". R5 records test 7 and test 13 before it lands, and stops and reports to Ian if either fails (section 5 and decision 11).
   6. Pieces and windows: the ceiling or a profile's limits fill a piece, a window of up to 200 words each side, `--window N` from 0 to 5,000, the evidence as a byte range of the original text, names forming after every request, a failed request failing the text, offsets unchanged, and one piece at an address with no ceiling and no profile. The known risk of a far clue (section 6).
   7. The hard cap: 600,000 bytes, exit 2 before any request at every address, no option, the message naming size and limit and echoing no text. In record mode it refuses that record, as a record over 16 MiB is refused (section 6).
   8. `recognize --jobs N` on one document, 1 to 32, default 4. Ticket 0143 already runs the pieces at once, and only the flag is missing (section 6, "Concurrency").
   9. The recognize batch shape: ADR 0048 items 2 to 6 and 13 apply. Evidence `{"records":[T1,…,TN]}`, the word-question prefix `In record K of the list. `, no quote prefix, a batch of one sending today's bytes, a text too big for a batch of one going alone and splitting by item 6, and until R4 lands going alone as one request with the whole text, a batch's `confirm` questions in one request after its word answers, relations per text, batching on by default, design test 9 reporting and gating nothing. `recognize` keeps one document as its default input, and `--field` with no framing flag reads one JSON document (section 7 and "Many short texts, one per line").
   10. Question identity: the new canonical order, `words` holding the five lists in order, every recognize digest changing, `audit --write` refusing old runs with its existing sentence, and `batch` and `jobs` staying out of the digest ("Defaults and what a caller can change"). `none` and the pair method are fixed, so they enter no question digest. They change request bodies, so old recordings and cache entries miss.
   11. Output: bare output unchanged, `question.words`, `question.boundary`, `question.window`, `answer.confirm`, `none` among each word's kind probabilities in `answer.tokens`, and `meta.batch` and `--facts` by ADR 0048 items 9 and 10 ("Output and run facts"). Every `relate --details` question entry is a yes/no entry.
   12. Dry run: `request_count` counts the requests formed before any answer, `confirm_questions_upper_bound` after it, at 0 under `run` and otherwise floor((W + 1) / 3) for each text, at any kind count, summed over a batch's texts, each split request naming its evidence offsets, and the first batch under a framing flag ("Output and run facts").
   13. Secrecy: no new failure line echoes a text, list entry, name or key, and the `Debug` rules ("Secrecy").
3. **What this amends.** The table under "What happens today", with the amendment each line gets.
4. **Which ticket builds each item.** One row per item, naming the design's labels. R2 builds item 2 and item 4, because R2 already changes every recognize request body and re-records once. R3 and R4 build items 3 and 6 on the command and question file. R6 carries `keep`, `infixes`, `boundary` and `window` to the libraries and SQL. R5 builds item 5 in `recognize` and `relate` and removes both pages' markers. R4 builds item 7. R4b builds item 8 and depends on R0 alone. R7 builds item 9 after batching B4 and B5. Items 10 to 13 split across R2, R3, R4, R5 and R7 by the key or path each adds.
5. **What Ian can overturn.** The design's fifteen "Open items", in its order, plus ticket 0147's calls: decisions 3 to 7, 10 and 11, item 7's record-mode refusal, and item 9's too-big text going alone, as one whole-text request until R4 lands. The status line says Ian can overturn each item and every rule the ADR copies.

The ADR copies each rule. It does not re-argue it. The design issue stays the argument for Ian's rulings, this ticket stays the argument for items 4 and 5, and the evidence record stays the measurement.

### The two new tests

The design numbers its tests 1 to 11. The ADR adds two, and the ticket table above places them.

12. **One kind declines, one recorded live run in R2.** Run the key with `recognize person` and every other default. `audit --match strict` reports precision of at least 85%, the bar test 6 sets. Today's build prints every detected name as `person`. A loopback test pins the mechanics: a word whose kind answer is `none` leaves its name unprinted, and a kind named `none` exits 2 with zero sends. The run costs about $0.012, the price of one key run in design test 6.
13. **Every stated edge, loopback and one recorded live run in R5.** A loopback run of `relate wrote=person:song` over 3 persons and 3 songs, answering yes for John Lennon's three songs, prints three edges. The same run with a fourth, unrelated song prints the same three edges. One recorded live run of `recognize person song --relation wrote=person:song` on "John Lennon wrote Help!, Girl and In My Life." prints all three `wrote` edges. It costs under $0.001.

Test 5 counts a name the run drops as `none` as a loss only when its key kind is among the run's kinds.

### The amendments

Accepted ADRs keep their text. Each named ADR line gains a marker, and ADR 0040 gains one section `## Amendment, 2026-09-26: ADR 0050 windows recognize texts` of at most four sentences. Settled pages keep today's sentence and gain the new rule after it as `Not built yet, by ADR 0050 item N: …`, the form ADR 0048 set. Each page's status line adds `amended by ADR 0050`. The ticket that builds item N deletes the old sentence and the marker in the same commit.

Items 4 and 5 fix wrong answers that ship today. Until they are built, `recognize.md` and `relate.md` also state each limit plainly, in a sentence that begins `Until ADR 0050 item N is built,`. The ticket that builds the item deletes that sentence.

Build rule: place each amendment by the sentence the table under "What happens today" quotes, not by its line number. Merge `origin/main` before editing and re-read every line number, because ticket 0146 may land first and move lines.

| Where | Line | Amendment |
| --- | --- | --- |
| ADR 0040 | 12 | Marker `(Amended by ADR 0050, below.)` |
| ADR 0040 | 20, 24 | The existing markers become `(Amended by ADRs 0048 and 0050, below.)` |
| ADR 0040 | new section at the end | The built-in ceiling reaches recognize word and `confirm` questions. A recognize piece carries its words and a window in place of the whole text. Relation requests keep the whole text. Ian can overturn this |
| Ticket 0080 | 53, 54 | A trailing marker on each: decision 2 lifted for the word lists, `boundary` and `window` by ADR 0050 items 2 and 3, and decision 3 replaced for word and `confirm` requests by item 6 |
| `sdlc/planning/recognize-design.md` | 176 | A trailing marker: one kind now declines names of other kinds by ADR 0050 item 4, and a caller who wants every name gives the defaults or a kind described to cover every name |
| `sdlc/planning/recognize-design.md` | 186 | A trailing marker: kept as `boundary: run` by ADR 0050 item 3, and `confirm` is the default |
| `sdlc/planning/relate-design.md` | 86 | A trailing marker: every concrete relation asks yes/no pairs by ADR 0050 item 5 |
| `sdlc/planning/relate-design.md` | 90 | A marker under the heading: relations reach no option ceiling once ADR 0050 item 5 is built |
| `sdlc/planning/relate-design.md` | 150 | A trailing marker: choice entries end with ADR 0050 item 5 |
| `recognize.md` | 3 | Status adds ADR 0050 |
| `recognize.md` | new paragraph after 7 | `Until ADR 0050 item 4 is built, one kind gives every detected name that kind, whatever the name is.` Then the markers for item 4, the kind question with `none` at every kind count, and items 6, 7 and 8, pieces with windows, the hard cap, and `--jobs N` on one text |
| `recognize.md` | new paragraph after 21 | Items 2 and 3: the five word lists with `--word-keep` and `--word-infix`, and `--boundary confirm` as the default with `run` keeping the rule above |
| `recognize.md` | new paragraph after 27 | Item 4: the kind name `none` is reserved and refused |
| `recognize.md` | new paragraph after 31 | Item 5: a relation means the text states it |
| `recognize.md` | new paragraph after 39 | `Until ADR 0050 item 5 is built, a different-kind relation keeps at most one edge per asking name at the default cut, because a choice's probabilities total one. The side that asks depends on how many names of each kind the text holds, so an unrelated name can change which edges come back.` Then the item 5 marker: one yes/no question per pair for every concrete relation |
| `recognize.md` | new paragraph after 45 | Items 9 and 11: record modes fill each request, `--batch 1` sends one text a request, and the new `--details` fields, `none` among them |
| `recognize.md` | 47 | Items 3 and 6: a failed `confirm` question or split request fails the whole text |
| `recognize.md` | new paragraph after 53 | Item 12: the new `request_count` rule, `confirm_questions_upper_bound`, and `words` under the effective rules |
| `question-file.md` | 3 | Status adds ADR 0050 |
| `question-file.md` | new paragraph after 9 | Items 2, 3, 4 and 6: `recognize.words`, `recognize.boundary` and `recognize.window`, and a kind named `none` refused at exit 5. The ban lifts for these keys only |
| `question-file.md` | new indented line under 133 | Item 10: the new recognize order, and every recognize digest changes |
| `result.md` | 3 | Status adds ADR 0050 |
| `result.md` | new paragraph after 44 | Item 11: `question.words`, `question.boundary`, `question.window`, `answer.confirm`, and `none` among the kind probabilities |
| `channels.md` | 3 | Status adds ADR 0050 |
| `channels.md` | new paragraph after 33 | Items 2, 3, 6 and 8: `recognize` takes `--word-keep`, `--word-infix`, `--boundary` and `--window N`, and `--jobs N` on one document |
| `channels.md` | 113 | Item 12: the first batch under a framing flag, and each split request's evidence offsets |
| `records.md` | 3 | Status adds ADR 0050 |
| `records.md` | 129 | Item 8: `recognize` also accepts `--jobs` for one text, whose pieces are distinct requests |
| `relate.md` | 3 | Status adds ADR 0050 |
| `relate.md` | new paragraph after 5 | One sentence outside the marker, because it is true today: `relate` sends no text, so its answers come from what the model knows of the world. Then the marker, item 5: a `recognize` relation means the text states it, and `relate` keeps its wording. R5 removes the marker |
| `relate.md` | new paragraph after 46 | `Until ADR 0050 item 5 is built, a different-kind relation keeps at most one edge per asking entity at the default cut, because a choice's probabilities total one. The side that asks depends on how many entities of each kind the set holds, so an unrelated entity can change which edges come back.` Then the item 5 marker: one yes/no question per pair for every concrete relation, `method` always `yes_no` and `fallback` always null |
| `relate.md` | 60 | Item 5: choice entries no longer occur, and every entry is a yes/no entry |
| `backends.md` | 3 | Status adds ADR 0050 |
| `backends.md` | 17 | Item 6: word and `confirm` requests carry a piece's words and window, and the built-in ceiling splits them. Relation requests keep the complete text. Item 5: no relation uses a choice, so none falls back |
| `audit.md` | 3 | Status adds ADR 0050 |
| `audit.md` | 165 | Item 10: `--write` refuses a `recognize` run made before the word rules. Rerun and write from the new run |
| `settings.md` | 90 to 96 | Each recognize line ends `Not built yet, by ADR 0050 item N.` The `window` line names tickets R4 and R6 |

`grep -rn -e "Not built yet, by ADR 0050" -e "Until ADR 0050" specification` lists every leftover. The last recognize ticket to land requires it to come back empty.

## Decisions

Each is the owner's call under Ian's rulings. Ian can overturn any of them.

1. **One ADR, 0050, for every recognize rule.** The R0 row asks for one. Later tickets each cite an item.
2. **Markers and amendments take ADR 0048's form.** Pages keep today's sentence beside `Not built yet, by ADR 0050 item N: …`, and accepted ADRs gain a marker and a dated section. One fixed phrase lets one `grep` list every leftover. A page stays true today.
3. **The recognize settings stay under "Settings on the way" and are not table rows.** `sdlc/scripts/settings` fails a table row whose flag is in no command's help, and `--word-keep`, `--word-infix`, `--boundary` and `--window` are in none yet. Plant (a) proves it. `settings.md` line 85 already rules that the ticket that lands a setting moves its line into the table. R0 adds the ADR item to each line. A table row marked not built would need the check to learn an exception, which weakens it.
4. **R4b depends on R0 alone.** Ticket 0143 already sends a split text's pieces at once, and a profile splits a text today. The ADR's ticket table says so. R4b's proof, a loopback count reaching the job count on a split text, works with a `max_questions` profile before R4 lands. The design issue's R4b row is left as Ian sent it, and the ADR governs.
5. **R6 carries `window` to the libraries and SQL.** The design's settings table gives `window=` and a `window` key on every surface, and no ticket row builds them. R6 already carries the other recognize settings there.
6. **The design's open details get one spelling each.** A batch's `confirm` request carries the batch's evidence object, and each question takes its record's `In record K of the list. ` prefix, as the word questions do. `confirm_questions_upper_bound` follows `request_count` in the plan's key order. It is 0 under `run`. Otherwise it is floor((W + 1) / 3) for each text, where W is the text's word count under the effective rules. That is the most runs of two or more words, each separated by another word, that W words can hold. It counts per text, not per piece, because a run belongs to one piece, and a batch sums it over its texts. It no longer drops to 0 at one kind, because item 4 lets a run change between a kind and `none`. A table test in R3 can pin it: W of 1, 2, 4 and 5 give 0, 1, 1 and 2. R4 names the two evidence-offset keys of a split request, because the design leaves them open.
7. **Each canonical key enters the digest with the ticket that builds it.** The order is fixed now. R2 adds `words`, R3 `boundary`, R4 `window`. The digests change more than once before 0.1, and no release ships between those tickets.
8. **The design issue stays open and unedited.** It closes when its last ticket lands. Its section 4 says kinds stay as today, and its R5 row names only the wording. ADR items 4 and 5 govern.
9. **R0 edits no line ticket 0146 edits.** Ticket 0146, B4, opens `records.md`, `backends.md`, `channels.md`, `question-file.md`, `result.md` and `settings.md`. It changes the ADR 0048 markers at `channels.md` 32 and 99, `records.md` 81, 85, 91, 109 and 135, `backends.md` near 21, `result.md` 102 and 103, the `question-file.md` settings table and precedence paragraph, and `settings.md` 87. R0 inserts its `channels.md` paragraph after the blank line 33, not on line 32, and leaves `settings.md` 85 and 87 alone.
10. **The kind question always offers `none`.** This answers review item 1. One kind is the most natural first call, and today it labels a company, a place and a song `person` at 0.98 to 0.99. A first-class recognizer must be able to say a name is not of the kind asked.
    - Cost. At one kind, each word gains its kind question. On the 26-word sentence above, the request grows from 16,700 to 28,759 bytes, 72% more. Input tokens roughly double, from about 150 to about 300 a word. That estimate splits section 13's 300 tokens a word evenly over its two questions, and no run measured one kind alone. At $0.042 a million input tokens, a 1,000-word text costs about $0.006 more. A one-kind run then costs what any run with two or more kinds costs today. A short text sends no added request, and a long text needs about twice as many pieces. At two or more kinds, `none` adds one option: about 12 bytes a word bare, measured as 312 bytes over 26 words, plus its description of about 45 bytes. Every recording with a kind question changes, so R2 re-records demo 44, the 40 harvest cases and the recognize conformance cases in the run it already makes. At about 2,665 input tokens a short case, that is about $0.005.
    - Speed. No request waits on another, so no round trip is added.
    - Accuracy. `choose.md` records that a sixth label fitting nothing changed 0 of 60 picks. The key's kinds include the catch-all `thing`, so `none` should rarely take a key name in test 6. That is a reason to expect little loss, not a measurement. Test 6 and test 12 measure it in R2 and R3, and a failed bar stops the ticket for Ian.
    - Why at every kind count. A name of an unlisted kind at two or more kinds is forced into a listed kind today, which is the same wrong answer. One rule is simpler than a special case for one kind. `none` at one kind only would save about 1% of bytes and some re-recording, and would keep that wrong answer.
    - Rejected: a yes/no "is this a KIND?" question per name after detection, because it adds a round trip to every run. Folding the kind into the detection question at one kind, because that changes the detection wording experiment 270 measured, and review item 3 needs its own measurement first. Refusing one kind or warning, because it refuses the most natural first call.
    - It overturns one clause of the 2026-09-21 ruling at `recognize-design.md` line 176: "A caller who wants names without kinds gives one kind." That caller now gives the defaults, or one kind described to cover every name. The ruling's substance, no depth option, stands. Ian can restore the clause.
    - `none` is reserved because the kind names are the wire labels. A caller kind named `none` is refused, not renamed. The name is rare, and a silent rename would change the caller's output.
11. **Every concrete relation asks yes/no pairs.** This answers review item 2. It is fixed in R5, not deferred. A choice's probabilities total one, so at the default cut one asking name passes at most one option, or two in an exact tie. The live run on "John Lennon wrote Help!, Girl and In My Life." kept 1 of 3 stated edges. The side that asks depends on counts, so adding an unrelated name changed 0 edges into 3 in the replay above. A yes/no pair question has neither fault. Each pair gets its own probability, and adding a name adds questions without changing any other pair's question.
    - Cost. Questions grow from the larger side's count to the product of both sides. Bytes grow little, because a choice question lists every option with its description while a pair question names two ids. The dry runs above moved by −13% to +38% and stayed at one request. Packed yes/no questions billed about 73 input tokens each in experiment 260 (section 2). By that rate, 10 persons and 50 songs add 450 questions, about 33,000 input tokens, about $0.0014. That is an estimate. A paragraph with a few names adds a few hundred tokens, small beside 300 a word for finding the names.
    - Speed. No round trip is added. A plan splits only when it passes the ceiling, and `--jobs` sends the split requests at once, by ticket 0143.
    - Simpler. One method replaces two. The choice planner, the asking-side rule and the option-ceiling fallback go. `relate --dry-run` keeps `method` and `fallback` so its schema holds.
    - Wording. The recognize pair question is the form experiment 265 measured: stated edges 0.84 to 0.99 and unstated edges 0.02 to 0.05 (section 10). The unmeasured pick-one wording in design section 5 is dropped, so R5 measures one wording, not two.
    - Accuracy risk. Independent yes/no answers may pass more false edges than a pick-one. Test 7 guards precision for `recognize`. Standalone `relate` has no key in this repository, so its precision after R5 is unmeasured. The bench's relate figure measured recall held down by this limit, and its owner reruns it after R5.
    - Rejected: a declared one-to-many or one-to-one mark per rule, because it adds a public key `relate-design.md` line 88 rules out, and a wrong mark still loses edges silently. The `tag` shape, because `tag.md` sends each label as its own yes/no question, so it is pairs with more words. A runner-up question, because it adds a round trip, and `relate-design.md` line 46 keeps no runner-up question. A lower cut, because three true options at 0.33 each sit where noise sits. Stating the limit and deferring the fix, because a first-class tool cannot drop true edges silently. The pages state the limit anyway until R5 lands.
    - It changes `relate-design.md` lines 86, 90 to 94 and 150, which ticket 0088 settled. Ian can restore the choice.

## Edge cases

| Case | What R0 does |
| --- | --- |
| Ticket 0146 lands first | The builder merges `origin/main` before the final run, and each R0 hunk sits on a line 0146 does not touch. A conflict means stop rule 6 |
| `settings.md` table | Untouched. The recognize lines stay under "Settings on the way", by decision 3 |
| `question-file.schema.json` and `recognize_file.rs` key lists | Untouched. `sdlc/scripts/settings` reads both, and R2 adds the keys with their parser |
| The design issue and the architect review 10 issue | Untouched, by decision 8. The review issue stays open for its other items |
| A rule the design states and Ian's rulings do not cover | The ADR copies it. The ADR's status line says Ian can overturn each item and every rule it copies, and the rule appears among the design author's calls or ticket 0147's calls |
| A number in the ADR | It names its evidence section, experiment or this ticket's check |
| A marked sentence a building ticket forgets to delete | The grep lists it. The ADR's ticket table names the ticket that removes it |
| `recognize.md` line 7 or 21, which several items change | One marker paragraph after each line names every item that reaches it |
| Another branch takes ADR 0050 first | Stop rule 5 |
| The word "decision model" | It never appears. `the_specification_defines_unresolved_once_and_keeps_the_closed_wording` bans it on every specification page |
| A caller already uses a kind named `none` | Item 4 refuses it once R2 lands. The ADR says so, and R2's changelog line names it |
| A same-kind relation | Item 5 keeps its pair questions and wording in `relate`. `recognize` gains the stated-text wording |
| ADR 0051 on ticket 0154's branch | It says every address gains a ceiling, which retires item 6's "one piece at an address with no ceiling". Its "Overlap with the recognize ADR" section already orders the two. R0 copies the design's rule as it stands |

## Proof

R0 adds no test. It changes no behavior, so a new test would check prose. The building tickets carry the design's tests 1 to 11 and the ADR's tests 12 and 13, each against the real command.

The proof is the review plus the checks that already guard these pages:

- The reviewer reads the ADR against the design's sections, Ian's rulings and decisions 10 and 11, item by item, and each amended line against the amendment table. Every changed page keeps today's sentence beside its marked rule.
- `grep -rn -e "Not built yet, by ADR 0050" -e "Until ADR 0050" specification` gives one hit for each marked passage and limit sentence in the amendment table and no other. The reviewer checks each hit names the right item.
- The diff touches only the files in `opens`. Nothing under `crates`, `libraries`, `databases`, `conformance`, `spec`, `site`, `demos` or `specification/fixtures` changes, and `question-file.schema.json` does not change.
- `sdlc/scripts/lint`, which runs the ticket check and the private-name check.
- `sdlc/scripts/settings` after `sdlc/scripts/install`, run as `env -u THINKTHEN_API_KEY python3 sdlc/scripts/settings`, reads the edited `settings.md` against the installed help.
- `the_specification_defines_unresolved_once_and_keeps_the_closed_wording`, run alone as `flock -o /run/user/1000/thinkthen-heavy.lock env -u THINKTHEN_API_KEY cargo test -p thinkthen --test version the_specification_defines_unresolved_once`.
- `grep -rnF -e "complete source text" -e "Recognition policy has no command" -e "exact recognition request count" -e "refuses it outside record mode" -e "larger side" spec` comes back empty before and after. No executable page quotes an amended sentence.

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

- ADR 0050: at most 140 nonblank lines.
- ADR 0040: at most 8 added or changed lines.
- Ticket 0080, `recognize-design.md` and `relate-design.md`: at most 7 changed lines in all.
- The nine specification pages: at most 60 added or changed lines in all.
- The build record: at most 40 nonblank lines.
- Nothing else changes. No code, test, fixture, schema, ratchet or dependency. The `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget.
2. Stop if an amendment needs a ruling Ian did not give and neither the design nor this ticket states. Report it with options.
3. Stop if a plant stays green, or a rung goes red for a reason page text cannot fix.
4. Stop before touching `specification/settings.md`'s table, line 85 or line 87, `question-file.schema.json`, `recognize_file.rs`, `site/`, the design issue or the architect review 10 issue.
5. Stop if another branch claims ADR 0050 before this one lands. The coordinator renumbers.
6. Stop if ticket 0146 or any other in-flight ticket edits a line R0 edits. The coordinator orders the two.

## Scope and exclusions

Excluded: any code. The `settings.md` table rows, by decision 3. The schema and the recognize file's key lists, which R2 changes with its parser. The design issue and the architect review 10 issue. The recognize how-to page, which R8 writes. `site/`, which the website agent owns. Review 10's other items: kinds steering detection, the published accuracy, and its severity 3 items stay in its issue.

## Routing

Owner and builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for the diff. No ceiling moves and no public surface widens.

## Complexity

Contract 3; state and timing 0; reach 2; proof 0; cost of error 1; total 6. Final level: 1. The risk is an amendment that misstates a ruling or a call. The item-by-item review guards it.

## Deferred gaps

- The marked sentences and limit sentences stay on the pages until the tickets that build their items land. The last recognize ticket empties the grep.
- The recognize settings move into the `settings.md` table with R2, R3, R4, R4b and R6.
- `question-file.schema.json` and `recognize_file.rs` gain `words`, `boundary` and `window` with R2, R3 and R4.
- The design issue's R4b row still names R4 as a dependency, and its section 6 still says pieces go one at a time. Its section 4 and R5 row predate items 4 and 5. The ADR's items and ticket table govern.
- The evidence-offset key names of a split request wait for R4.
- `recognize` over records under B4's `batch` row waits for R7.
- Windowed relations stay a known gap, as the design says.
- Standalone `relate` precision under pair questions is unmeasured until someone runs a key. The bench's relate figure changes with R5, and its owner reruns it.
- One kind's token cost is an estimate until test 12's recorded run reports it.

## What Ian can overturn

- Every ruling and author's call the ADR copies. The design's "Open items" list names fifteen.
- Decision 3: the recognize settings under "Settings on the way", not as marked table rows.
- Decision 4: R4b depending on R0 alone.
- Decision 5: R6 carrying `window`.
- Decision 6: the spellings of the batched `confirm` request and the dry-run bound.
- Decision 7: each digest key entering with its building ticket.
- Decision 10: the kind question offering `none` at every kind count, `none` reserved, and the one-kind clause of the 2026-09-21 ruling set aside.
- Decision 11: every concrete relation asking yes/no pairs in `recognize` and `relate`, which replaces the choice that ticket 0088 settled.
- ADR item 7: a text over the cap in record mode refuses that record.
- ADR item 9: a text too big for a batch of one goes alone, as one whole-text request until R4 lands. R7 keeps its dependencies.

## Closes

None. `sdlc/issues/2026-09-26-recognize-design.md` stays open until its last ticket lands. `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md` stays open. R2 answers its item 1 and R5 its item 2.

## Evidence

- Starts from: Ian's rulings and the R0 row in `sdlc/issues/2026-09-26-recognize-design.md`, sent 2026-09-26. Items 1 and 2 of `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md`, from local experiment 273, report 10. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` sections 2 and 10 to 13, from experiments 260, 265, 267, 270 and 271. Ticket 0139 and ADR 0048, which set the form, and ADR 0048's "What recognize R0 may rely on". Tickets 0143 and 0144 on main, and tickets 0145 and 0146 read from their branches, for the R4b and R7 check. The page lines in "What happens today" at `origin/main` `c490f082`. This ticket's offline replay and dry runs of main's binary at that commit, which sent nothing.
- Keeps: Every behavior, help line, fixture, schema, digest and test. Every accepted ADR sentence, marked and not deleted. Every Settled page's current sentence, beside its marked replacement. The `settings.md` table.
- Changes: A new ADR 0050 holds the recognize rulings and two architect's calls: kinds decline through `none`, and every relation asks yes/no pairs. ADR 0040, ticket 0080, the 2026-09-21 and 2026-09-23 rulings and the relate design carry markers, and ADR 0040 gains a dated amendment section. Nine specification pages state each new rule beside today's, under the marker, and state the one-kind and one-edge limits plainly until they are fixed. The `settings.md` recognize lines cite their ADR items.
- Proof: The item-by-item review against the design, the rulings, decisions 10 and 11 and the amendment table. The marker grep and the `spec/` grep. `lint`, the settings check and the specification wording test, with plants (a) to (c) each turning one red.
- Defers: The table rows to R2, R3, R4, R4b and R6. The schema and key lists to R2 to R4. The evidence-offset key names to R4. Removing each marker and limit sentence to the ticket that builds its item. Windowed relations. Standalone `relate` precision under pairs.
