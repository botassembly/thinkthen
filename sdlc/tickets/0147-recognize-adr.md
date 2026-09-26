---
flow: build
priority: 147
opens: sdlc/planning/adr/0050-recognize-reads-words-confirms-runs-and-windows-long-texts.md sdlc/planning/adr/0040-split-requests-under-backend-limits.md sdlc/tickets/0080-build-recognize.md sdlc/planning/recognize-design.md sdlc/planning/relate-design.md specification/recognize.md specification/question-file.md specification/result.md specification/channels.md specification/records.md specification/relate.md specification/backends.md specification/audit.md specification/settings.md sdlc/records sdlc/tickets
---

# 0147: Record the recognize rulings in one ADR

Status: ready for review. The coordinator accepted an earlier version on 2026-09-26. This version answers the two severity 1 findings of architect review 10 and the eleven findings of the re-review. A fresh read-only reviewer re-accepts it before it is built. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Ian's recognize rulings of 2026-09-26 become one accepted ADR, and the Settled pages they contradict say so. Every later recognize ticket then cites one ADR item in place of the design issue.

The authority is `sdlc/issues/2026-09-26-recognize-design.md`, which Ian sent to the main builder on 2026-09-26 with the review findings and his rulings applied. This is its ticket R0. The R0 row lists the scope: word rules, `confirm` as the default, stated relations, pieces and windows for long texts, the hard cap, `recognize --jobs` for one document, and the recognize batch shape. The row says R0 lands after batching B0. B0 landed as ticket 0139, ADR 0048. Ian can overturn each ruling, as the design's "Open items" list says.

The ADR also carries two proposals. They answer items 1 and 2 of `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md`, whose detail sits in local experiment 273, report 10. Item 1 found that one kind gives that kind to every name. Item 2 found that a different-kind relation keeps at most one edge per asking name. Both give wrong answers at high confidence with no warning.

Each proposal contradicts one of Ian's rulings, so neither is decided here:

- **ADR item 4, decision 10.** It contradicts the ruling "Ruled 2026-09-21: where the design page and the method page disagree, this page wins on what a user sees" in `sdlc/planning/recognize-design.md`. That ruling's `depth` row, line 176, says "A caller who wants names without kinds gives one kind."
- **ADR item 5, decision 11.** It contradicts "Current final hybrid planner — authoritative" in `sdlc/planning/relate-design.md`, lines 72 to 74, which says "This is the only current method ruling." Line 200 records that "Ian's later final direction combines cross-kind choice with same-kind H. The all-H method is superseded." It also changes "Ruled Option A detailed result — exact public schema", lines 130 to 150, whose choice entries would no longer occur.

The coordinator has asked Ian to rule on both. Until he rules, ADR items 4 and 5 read "proposed, awaiting Ian's ruling". R2 builds item 4, and R5 builds item 5, only on Ian's word. Each item states a fallback. If Ian keeps his ruling, the item is withdrawn and the page states the limit plainly as settled behavior.

Ian's goal is a first-class name recognizer, and speed wins over accuracy. Batching fills each request. R0 writes no code. It changes no behavior, help text, fixture, schema or digest. `recognize` and `relate` behave the same after it lands.

## What happens today

The pages and records below state what the rulings and the two proposals change. Each line is at `origin/main` `c490f082`.

| Where | Line | What it says today |
| --- | --- | --- |
| ADR 0040 | 12 | "Each chunk repeats the evidence" |
| ADR 0040 | 20 | "Records, evidence text, options, and unrelated calls are never combined or divided." ADR 0048 amended the records part |
| ADR 0040 | 24 | "Other plans and other addresses keep no ceiling." ADR 0048 amended it for batched record plans |
| Ticket 0080 | 53 | Decision 2: "Keep all recognition policy internal … One maximal contiguous run of `IN` words is one candidate" |
| Ticket 0080 | 54 | Decision 3: "Preserve the exact source text through every request" |
| `sdlc/planning/recognize-design.md` | 176 | The 2026-09-21 ruling's `depth` row: "A caller who wants names without kinds gives one kind" |
| `sdlc/planning/recognize-design.md` | 186 | The 2026-09-23 ruling ships the maximal-run baseline |
| `sdlc/planning/relate-design.md` | 60 | "`method` is `choice` or `yes_no`. `fallback` is null, `max_options`, or `max_request_bytes`." |
| `sdlc/planning/relate-design.md` | 72 to 74 | "Current final hybrid planner — authoritative. This is the only current method ruling." |
| `sdlc/planning/relate-design.md` | 86 | Different-kind relations ask a choice from the larger side, and "Every non-`none` option at or above the cut becomes an edge" |
| `sdlc/planning/relate-design.md` | 90 to 94 | "Exact option and backend-profile fallback": a choice over 255 options or over a profile's limit falls back to yes/no pairs |
| `sdlc/planning/relate-design.md` | 130 to 150 | "Ruled Option A detailed result": choice entries, and "Several real candidates may be accepted" |
| `sdlc/planning/relate-design.md` | 200 | The all-H proposal is superseded by Ian's final hybrid direction |
| `specification/recognize.md` | 7 | "One kind is assigned locally with probability one." "Long text keeps its complete source context" |
| `specification/recognize.md` | 21 | Today's splitter and the maximal-run rule |
| `specification/recognize.md` | 27 | A run takes 1 through 20 distinct nonblank kinds, with no reserved kind name |
| `specification/recognize.md` | 31 | A relation keeps an edge at its model probability, with no statement about the text |
| `specification/recognize.md` | 39 | Different-kind relations use one choice per member of the larger side, and "Every option at or above the cut becomes an edge" |
| `specification/recognize.md` | 45, 47 | `--details` fields, and a failed question failing its input |
| `specification/recognize.md` | 53 | "`request_count` is the exact recognition request count" |
| `specification/question-file.md` | 9 | "Recognition policy has no command or file keys." |
| `specification/question-file.md` | 133 | The recognize canonical order: `verb`, `kinds`, optional `relations`, `threshold`, `relation_threshold`, optional `profile` |
| `specification/result.md` | 42 | `relate --details` keeps "each choice or yes/no relation question, including its asker or endpoints, candidates, … pre-threshold pick" |
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

Dry runs of the same binary priced both proposals. They measure bytes, not tokens.

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

Status line: accepted 2026-09-26 on Ian's rulings, through ticket 0147, built by the recognize tickets it names, Ian can overturn each item. Items 4 and 5 are proposed, awaiting Ian's ruling. If Ian rules before R0 is built, the builder writes his ruling in place of the proposal and drops the mark. Its sections:

1. **Context.** Today's splitter reaches 141 of the key's 168 names (evidence section 11). Unstated edges scored 0.55 to 0.80, and asked about the text they scored 0.02 to 0.05 (section 10). Each word costs about 300 input tokens and 1,150 request bytes, Jev refuses a request over 65,536 input tokens, and a text over about 200 words fails today (section 13). Local experiment 273, report 10, found that one kind labels every name that kind and that a different-kind relation keeps at most one edge per asking name. Ticket 0147 checked both on main's code. Ian's rulings. "Section N" means that section of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`.
2. **Decision.** One numbered item per rule, each copied from the design section it cites or from this ticket's decision:
   1. One request finds and labels the names: a yes/no question per word and, with two or more kinds, a pick-one kind question per word, each with a five-word snippet. No BILOU scheme, no span questions (design section 1). If Ian accepts item 4, the kind question applies at one kind too.
   2. The word rules: the five lists and their defaults, the fixed rules, each list adding to its default, `defaults: false`, the MUC-7, ACE and CoNLL convention, and the limits of 100 entries of at most 32 characters with no white space. A bad entry exits 2 on the command line and 5 in a file, by `question-file.md` line 33. `keep` and `infixes` reach every surface. `prefixes`, `suffixes`, `trim` and `defaults` live only in the question file. It lifts the policy-key ban for the lists, `boundary` and `window` (section 2).
   3. The `boundary` setting: `confirm` by default with its measured question, `ONE` and `SPLIT`, one `confirm` request per piece after the word answers, a run belonging to its first word's piece, and `confirm` questions splitting over the ceiling. `run` keeps the 2026-09-23 maximal-run baseline (section 3). If Ian accepts item 4, `none` counts as a kind for `confirm`, so a run whose words change between a kind and `none` is asked.
   4. **Proposed, awaiting Ian's ruling.** Kinds decline. Every run asks the kind question, at one kind as at several. Its options are the caller's kinds in order, then `none`, described as "Part of a name of none of the listed kinds." A name whose kind comes out `none`, by the vote and its fallback, is not printed. The kind vote, `strength` and the 0.5 cut stay (section 4), with `none` counted as one more kind. A kind that equals `none` in any ASCII case is refused: exit 2 on the command line and 5 in a file, with a message that echoes no text. Twenty kinds with `none` make 21 options. A profile whose `max_options` is under the run's option count refuses the run at exit 2 before any request, naming the count and the limit. A caller who wants every name gives the defaults, or one kind whose description covers every name. **Fallback if Ian keeps the 2026-09-21 ruling:** the item is withdrawn, one kind keeps labelling every detected name that kind, and `recognize.md` states that as settled behavior in one plain sentence.
   5. **Proposed, awaiting Ian's ruling.** A relation asks one yes/no question per pair, for every concrete relation, in `recognize` and in `relate`. A different-kind relation asks each source and target pair in the declared direction, as a same-kind relation already does. No choice, no asking side and no option-ceiling fallback remain. `relate --dry-run` keeps the `method` and `fallback` keys, always `yes_no` and null. A relation request at the built-in address holds at most 400 questions, and a profile's `max_questions` replaces that number. **Fallback if Ian keeps the hybrid planner:** the item is withdrawn, the choice stays, and `recognize.md` and `relate.md` state the one-edge limit as settled behavior in plain sentences.
   6. A recognize relation means the text states it. Under item 5, its pair question reads "Does the text state that the relation holds from i1 to i2?", and under `--either` it reads "Does the text state that the relation holds between i1 and i2?". Under the fallback, the pick-one question takes design section 5's wording. The whole text is relation evidence, and a relation request over Jev's evidence limit exits 4. Standalone `relate` sends no text and keeps "Does the relation hold from i1 to i2?" and "Does the relation hold between i1 and i2?". R5 records test 7 before it lands, and stops and reports to Ian if it fails (section 5).
   7. Pieces and windows: the ceiling or a profile's limits fill a piece, a window of up to 200 words each side, `--window N` from 0 to 5,000, the evidence as a byte range of the original text, names forming after every request, a failed request failing the text, offsets unchanged, and one piece at an address with no ceiling and no profile. The known risk of a far clue (section 6).
   8. The hard cap: 600,000 bytes, exit 2 before any request at every address, no option, the message naming size and limit and echoing no text. In record mode it refuses that record, as a record over 16 MiB is refused (section 6).
   9. `recognize --jobs N` on one document, 1 to 32, default 4. Ticket 0143 already runs the pieces at once, and only the flag is missing (section 6, "Concurrency").
   10. The recognize batch shape: ADR 0048 items 2 to 6 and 13 apply, as ADR 0053 amends them. Evidence `{"records":[T1,…,TN]}`, the word-question prefix `In record K of the list. `, no quote prefix, a batch of one sending today's bytes, a text too big for a batch of one going alone and splitting by item 7, and until R4 lands going alone as one request with the whole text, a batch's `confirm` questions in one request after its word answers, relations per text, batching on by default, design test 9 reporting and gating nothing. ADR 0053 item 5's trust statement reaches recognize batches: the texts of one batch are evidence for each other, and `--batch 1` is the defence for texts from different people or untrusted sources. `recognize.md` states it with R7. `recognize` keeps one document as its default input, and `--field` with no framing flag reads one JSON document (section 7 and "Many short texts, one per line").
   11. Question identity: the new canonical order, `words` holding the five lists in order, every recognize digest changing, `audit --write` refusing old runs with its existing sentence, and `batch` and `jobs` staying out of the digest ("Defaults and what a caller can change"). `none` and the pair method are fixed rules, so they enter no question digest. They change request bodies, so old recordings and cache entries miss.
   12. Output: bare output unchanged, `question.words`, `question.boundary`, `question.window`, `answer.confirm`, and `meta.batch` and `--facts` by ADR 0048 items 9 and 10 ("Output and run facts"). Under item 4, `none` appears among each word's kind probabilities in `answer.tokens`. Under item 5, every `relate --details` question entry is a yes/no entry.
   13. Dry run: `request_count` counts the requests formed before any answer, `confirm_questions_upper_bound` after it, and each split request names its evidence offsets. The bound is 0 under `run`. Otherwise it is floor((W + 1) / 3) for each text, summed over a batch's texts. Under item 4 that holds at any kind count, and under its fallback the bound is 0 at one kind. The first batch under a framing flag ("Output and run facts").
   14. Secrecy: no new failure line echoes a text, list entry, name or key, and the `Debug` rules ("Secrecy").
3. **What this amends.** The table under "What happens today", with the amendment each line gets.
4. **Which ticket builds each item.** One row per item, naming the design's labels. R2 builds item 2 and, on Ian's word, item 4. R3 and R4 build items 3 and 7 on the command and question file. R6 carries `keep`, `infixes`, `boundary` and `window` to the libraries and SQL, and adds one conformance case per surface for item 4's refusal. R5 builds item 6 and, on Ian's word, item 5, in `recognize` and `relate`, and removes both pages' markers. R4 builds item 8. R4b builds item 9 and depends on R0 alone. R7 builds item 10 after batching B4 and B5. Items 11 to 14 split across R2, R3, R4, R5 and R7 by the key or path each adds.
5. **What Ian can overturn.** The design's fifteen "Open items", in its order, plus ticket 0147's calls: decisions 3 to 7 and 12, item 8's record-mode refusal, and item 10's too-big text going alone, as one whole-text request until R4 lands. Items 4 and 5 wait for his ruling. The status line says Ian can overturn each item and every rule the ADR copies.

The ADR copies each rule. It does not re-argue it. The design issue stays the argument for Ian's rulings, this ticket stays the argument for items 4 and 5, and the evidence record stays the measurement.

### Paid runs

Each paid run below is its own run. Ian authorizes each one before it starts, and it runs only from `sdlc/scripts/live` under a token cap. The costs use section 13's rates: about 2,665 input tokens a short case and $0.042 a million input tokens.

| Ticket | Run | Why | Estimated cost |
| --- | --- | --- | --- |
| R2, on Ian's word for item 4 | Re-record demo 44's one recognize request | Item 4 adds `none` to every kind question | under $0.001 |
| R2, on Ian's word for item 4 | Re-record 38 harvest cases in `recognize-225`, beyond C06 and C16 | Same | about $0.004 |
| R2, on Ian's word for item 4 | Re-record the recognition requests of 10 conformance cases, 41 to 50 | Same | about $0.001 |
| R3 | Test 6, the key under `confirm` with the key's kinds | The design's existing run | about $0.012 |
| R3, on Ian's word for item 4 | Test 12, the key under `recognize person` | Item 4's bar | about $0.012 |
| R5, on Ian's word for item 5 | Test 13, one sentence | Item 5's proof and token rate | under $0.001 |

The design already has R2 re-record harvest cases C06 and C16 and replay the rest. Its R5 row already re-records the relation fixtures in `recognize-239` and the relation requests of conformance cases 42 to 50. Test 7 is the design's existing R5 run. The six runs above are new work that items 4 and 5 add, except test 6.

### The two new tests

The design numbers its tests 1 to 11. The ADR adds two, each built only on Ian's word.

12. **One kind declines. Loopback in R2, and one recorded live run in R3.** R2's loopback test sends a one-kind run whose recorded kind answer for one name's words is `none`, and checks that the name does not print. The same test refuses kinds `none` and `NONE` at exit 2, and a listener counts zero requests. R3's run takes the key under `recognize person` with every other default, at R3's build, beside test 6. `audit --match strict` over the whole key gives precision, and the bar is at least 85%, the bar test 6 sets. A printed `person` name whose key kind is not `person` counts against precision. The same run reports person recall from `audit` over the key filtered to `person` lines with `jq`. That filter is a `jq` step, not a scoring helper.
    - What behavior does it protect? A run with one kind prints only names of that kind.
    - What credible regression fails it? A change that drops the `none` option or skips the kind question at one kind, as main does today.
    - Why does no existing test catch it? Every recognize test with one kind expects every name to take that kind.
    - Does it need a test-only hook? No. It drives the real command at a loopback address and a recorded run.
13. **Every stated edge. Loopback and one recorded live run in R5.** A loopback run of `relate wrote=person:song` over 3 persons and 3 songs, answering yes for John Lennon's three songs, prints three edges. The same run with a fourth, unrelated song prints the same three edges. A dry run of 5 persons by 180 songs at the built-in address shows 3 requests of 400, 400 and 100 questions. One recorded live run of `recognize person song --relation wrote=person:song` on "John Lennon wrote Help!, Girl and In My Life." prints all three `wrote` edges and reports input tokens a pair question.
    - What behavior does it protect? A one-to-many fact keeps every stated edge, and an unrelated name changes no edge.
    - What credible regression fails it? Restoring the choice for different-kind relations, which prints 0 or 1 of the 3 edges.
    - Why does no existing test catch it? The planner tests pin the choice method and its asking side.
    - Does it need a test-only hook? No. It uses the real command, a loopback listener and a recorded run.

Test 5 compares names under a run's kinds. Under item 4, a name dropped as `none` is a loss only when its key kind is among the run's kinds. The builder filters the key to the run's kinds with `jq` before `audit` grades it.

### The amendments

Accepted ADRs keep their text. Each named ADR line gains a marker, and ADR 0040 gains one section `## Amendment, 2026-09-26: ADR 0050 windows recognize texts` of at most four sentences. Settled pages keep today's sentence and gain the new rule after it as `Not built yet, by ADR 0050 item N: …`, the form ADR 0048 set. For items 4 and 5, the marker reads `Proposed by ADR 0050 item N, awaiting Ian's ruling: …`. If Ian accepts, that marker becomes `Not built yet, by ADR 0050 item N: …`. If he keeps his ruling, the marker goes and the limit sentence stays. Each page's status line adds `amended by ADR 0050`. The ticket that builds item N deletes the old sentence and the marker in the same commit.

Items 4 and 5 answer wrong answers that ship today. `recognize.md` and `relate.md` therefore state each limit plainly now, in a sentence that stays true whatever Ian rules. The ticket that builds the item deletes that sentence. Under the fallback, the sentence stays as settled behavior.

Build rule: place each amendment by the sentence the table under "What happens today" quotes, not by its line number. Merge `origin/main` before editing and re-read every line number, because ticket 0146 may land first and move lines.

| Where | Line | Amendment |
| --- | --- | --- |
| ADR 0040 | 12 | Marker `(Amended by ADR 0050, below.)` |
| ADR 0040 | 20, 24 | The existing markers become `(Amended by ADRs 0048 and 0050, below.)` |
| ADR 0040 | new section at the end | The built-in ceiling reaches recognize word and `confirm` questions. A recognize piece carries its words and a window in place of the whole text. Relation requests keep the whole text. Ian can overturn this |
| Ticket 0080 | 53, 54 | A trailing marker on each: decision 2 lifted for the word lists, `boundary` and `window` by ADR 0050 items 2 and 3, and decision 3 replaced for word and `confirm` requests by item 7 |
| `sdlc/planning/recognize-design.md` | 176 | A trailing marker: ADR 0050 item 4 proposes that one kind declines names of other kinds, awaiting Ian's ruling |
| `sdlc/planning/recognize-design.md` | 186 | A trailing marker: kept as `boundary: run` by ADR 0050 item 3, and `confirm` is the default |
| `sdlc/planning/relate-design.md` | 60 | A trailing marker: ADR 0050 item 5 proposes `method` always `yes_no` and `fallback` always null, awaiting Ian's ruling |
| `sdlc/planning/relate-design.md` | 74 | A trailing marker: ADR 0050 item 5 proposes yes/no pairs for every concrete relation, awaiting Ian's ruling |
| `sdlc/planning/relate-design.md` | 86 | A trailing marker naming ADR 0050 item 5, proposed |
| `sdlc/planning/relate-design.md` | 90 | A marker under the heading: under ADR 0050 item 5, relations reach no option ceiling |
| `sdlc/planning/relate-design.md` | 150 | A trailing marker: under ADR 0050 item 5, choice entries no longer occur |
| `sdlc/planning/relate-design.md` | 200 | A trailing marker: ADR 0050 item 5 proposes the pair method again, for the reason in ticket 0147 decision 11, awaiting Ian's ruling |
| `recognize.md` | 3 | Status adds ADR 0050 |
| `recognize.md` | new paragraph after 7 | `One kind gives every detected name that kind, whatever the name is.` Then the proposed marker for item 4, and the markers for items 7, 8 and 9, pieces with windows, the hard cap, and `--jobs N` on one text |
| `recognize.md` | new paragraph after 21 | Items 2 and 3: the five word lists with `--word-keep` and `--word-infix`, and `--boundary confirm` as the default with `run` keeping the rule above |
| `recognize.md` | new paragraph after 27 | Item 4, proposed: a kind equal to `none` in any case is refused |
| `recognize.md` | new paragraph after 31 | Item 6: a relation means the text states it |
| `recognize.md` | new paragraph after 39 | `At the default cut, a different-kind relation keeps at most one edge per asking name, because a choice's probabilities total one. The side that asks depends on how many names of each kind the text holds, so an unrelated name can change which edges come back.` Then the proposed marker for item 5 |
| `recognize.md` | new paragraph after 45 | Items 10 and 12: record modes fill each request, `--batch 1` sends one text a request and keeps texts from being evidence for each other, and the new `--details` fields |
| `recognize.md` | 47 | Items 3 and 7: a failed `confirm` question or split request fails the whole text |
| `recognize.md` | new paragraph after 53 | Item 13: the new `request_count` rule, `confirm_questions_upper_bound`, and `words` under the effective rules |
| `question-file.md` | 3 | Status adds ADR 0050 |
| `question-file.md` | new paragraph after 9 | Items 2, 3 and 7: `recognize.words`, `recognize.boundary` and `recognize.window`. The ban lifts for these keys only. Item 4, proposed: a kind equal to `none` is refused at exit 5 |
| `question-file.md` | new indented line under 133 | Item 11: the new recognize order, and every recognize digest changes |
| `result.md` | 3 | Status adds ADR 0050 |
| `result.md` | 42 | Item 5, proposed: every `relate --details` entry is a yes/no entry |
| `result.md` | new paragraph after 44 | Item 12: `question.words`, `question.boundary`, `question.window` and `answer.confirm`. Item 4, proposed: `none` among the kind probabilities |
| `channels.md` | 3 | Status adds ADR 0050 |
| `channels.md` | new paragraph after 33 | Items 2, 3, 7 and 9: `recognize` takes `--word-keep`, `--word-infix`, `--boundary` and `--window N`, and `--jobs N` on one document |
| `channels.md` | 113 | Item 13: the first batch under a framing flag, and each split request's evidence offsets |
| `records.md` | 3 | Status adds ADR 0050 |
| `records.md` | 129 | Item 9: `recognize` also accepts `--jobs` for one text, whose pieces are distinct requests |
| `relate.md` | 3 | Status adds ADR 0050 |
| `relate.md` | new paragraph after 5 | One sentence outside the marker, because it is true today: `relate` sends no text, so its answers come from what the model knows of the world. Then the marker, item 6: a `recognize` relation means the text states it, and `relate` keeps its wording. R5 removes the marker |
| `relate.md` | new paragraph after 46 | `At the default cut, a different-kind relation keeps at most one edge per asking entity, because a choice's probabilities total one. The side that asks depends on how many entities of each kind the set holds, so an unrelated entity can change which edges come back.` Then the proposed marker for item 5: one yes/no question per pair, at most 400 a request, `method` always `yes_no` and `fallback` always null |
| `relate.md` | 60 | Item 5, proposed: choice entries no longer occur |
| `backends.md` | 3 | Status adds ADR 0050 |
| `backends.md` | 17 | Item 7: word and `confirm` requests carry a piece's words and window, and the built-in ceiling splits them. Relation requests keep the complete text. Item 5, proposed: no relation uses a choice, and a relation request holds at most 400 questions |
| `audit.md` | 3 | Status adds ADR 0050 |
| `audit.md` | 165 | Item 11: `--write` refuses a `recognize` run made before the word rules. Rerun and write from the new run |
| `settings.md` | 90 to 96 | Each recognize line ends `Not built yet, by ADR 0050 item N.` The `window` line names tickets R4 and R6 |

`grep -rn -e "Not built yet, by ADR 0050" -e "Proposed by ADR 0050" specification` lists every leftover marker. `grep -rn -e "keeps at most one edge per asking" -e "One kind gives every detected name" specification` lists the two limit sentences. The last recognize ticket to land requires the first grep to come back empty.

## Decisions

Each is the owner's call under Ian's rulings, except decisions 10 and 11, which wait for his ruling. Ian can overturn any of them.

1. **One ADR, 0050, for every recognize rule.** The R0 row asks for one. Later tickets each cite an item.
2. **Markers and amendments take ADR 0048's form.** Pages keep today's sentence beside `Not built yet, by ADR 0050 item N: …`, and accepted ADRs gain a marker and a dated section. One fixed phrase lets one `grep` list every leftover. A page stays true today.
3. **The recognize settings stay under "Settings on the way" and are not table rows.** `sdlc/scripts/settings` fails a table row whose flag is in no command's help, and `--word-keep`, `--word-infix`, `--boundary` and `--window` are in none yet. Plant (a) proves it. `settings.md` line 85 already rules that the ticket that lands a setting moves its line into the table. R0 adds the ADR item to each line. A table row marked not built would need the check to learn an exception, which weakens it.
4. **R4b depends on R0 alone.** Ticket 0143 already sends a split text's pieces at once, and a profile splits a text today. The ADR's ticket table says so. R4b's proof, a loopback count reaching the job count on a split text, works with a `max_questions` profile before R4 lands. The design issue's R4b row is left as Ian sent it, and the ADR governs.
5. **R6 carries `window` to the libraries and SQL.** The design's settings table gives `window=` and a `window` key on every surface, and no ticket row builds them. R6 already carries the other recognize settings there.
6. **The design's open details get one spelling each.** A batch's `confirm` request carries the batch's evidence object, and each question takes its record's `In record K of the list. ` prefix, as the word questions do. `confirm_questions_upper_bound` follows `request_count` in the plan's key order. It is 0 under `run`. Otherwise it is floor((W + 1) / 3) for each text, where W is the text's word count under the effective rules. That is the most runs of two or more words, each separated by another word, that W words can hold. It counts per text, not per piece, because a run belongs to one piece, and a batch sums it over its texts. At one kind it is 0 unless Ian accepts item 4, which lets a run change between a kind and `none`. A table test in R3 can pin it: W of 1, 2, 4 and 5 give 0, 1, 1 and 2. R4 names the two evidence-offset keys of a split request, because the design leaves them open.
7. **Each canonical key enters the digest with the ticket that builds it.** The order is fixed now. R2 adds `words`, R3 `boundary`, R4 `window`. The digests change more than once before 0.1, and no release ships between those tickets.
8. **The design issue stays open and unedited.** It closes when its last ticket lands. Its section 4 says kinds stay as today, and its R5 row names only the wording. ADR items 4 and 5 govern once Ian accepts them.
9. **R0 edits no line ticket 0146 edits.** Ticket 0146, B4, opens `records.md`, `backends.md`, `channels.md`, `question-file.md`, `result.md` and `settings.md`. It changes the ADR 0048 markers at `channels.md` 32 and 99, `records.md` 81, 85, 91, 109 and 135, `backends.md` near 21, `result.md` 102 and 103, the `question-file.md` settings table and precedence paragraph, and `settings.md` 87. R0 inserts its `channels.md` paragraph after the blank line 33, not on line 32, and leaves `settings.md` 85 and 87 alone.
10. **Proposed: the kind question always offers `none`.** This answers review item 1 and waits for Ian's ruling, because it sets aside the one-kind clause of the 2026-09-21 ruling. One kind is the most natural first call, and today it labels a company, a place and a song `person` at 0.98 to 0.99. A first-class recognizer must be able to say a name is not of the kind asked.
    - Cost at one kind. Each word gains its kind question. On the 26-word sentence above, the request grows from 16,700 to 28,759 bytes, 72% more. Input tokens roughly double, from about 150 to about 300 a word. That estimate splits section 13's 300 tokens a word evenly over its two questions, and no run measured one kind alone. At $0.042 a million input tokens, a 1,000-word text costs about $0.006 more. A one-kind run then costs what any run with two or more kinds costs today. A short text sends no added request, and a long text needs about twice as many pieces.
    - Cost at two or more kinds. `none` adds its label and its description of about 45 bytes to every word's kind question. That is about 57 bytes a word, about 5% of section 13's 1,150 bytes a word. The dry runs above show the label's share: one more bare kind added 312 bytes over 26 words.
    - Recordings. Every recording with a kind question changes. That is new paid work, listed under "Paid runs": demo 44, 38 harvest cases and 10 conformance cases, about $0.005 in all. The site's recognize recordings change too, by decision 12.
    - Speed. No request waits on another, so no round trip is added.
    - Accuracy. `choose.md` records that a sixth label fitting nothing changed 0 of 60 picks. The key's kinds include the catch-all `thing`, so `none` should rarely take a key name in test 6. That is a reason to expect little loss, not a measurement. Tests 6 and 12 measure it at R3's build, and a failed bar stops the ticket for Ian.
    - Why at every kind count. A name of an unlisted kind at two or more kinds is forced into a listed kind today, which is the same wrong answer. One rule is simpler than a special case for one kind. `none` at one kind only would save about 5% of bytes at two or more kinds and some re-recording, and would keep that wrong answer.
    - `none` is reserved in any ASCII case. The kind names are the wire labels, and two options that differ only in case would confuse the model, as `choose.md` warns for options that overlap. `None` and `NONE` are therefore refused too. A silent rename would change the caller's output, so the tool refuses.
    - Surfaces. The refusal lives in the core check every surface calls, so R2 brings it to the libraries and SQL. R6 owns one conformance case per surface that pins it.
    - Option count. Twenty kinds plus `none` make 21 options, under the fixed ceiling of 255. A profile with `max_options` of 20 or less refuses such a run at exit 2 before any request. Today that run passes, so the change is named in R2's changelog line.
    - Rejected: a yes/no "is this a KIND?" question per name after detection, because it adds a round trip to every run. Folding the kind into the detection question at one kind, because that changes the detection wording experiment 270 measured, and review item 3 needs its own measurement first. Refusing one kind or warning, because it refuses the most natural first call.
    - The ruling it sets aside. "Ruled 2026-09-21: where the design page and the method page disagree, this page wins on what a user sees", `depth` row, line 176: "A caller who wants names without kinds gives one kind." That caller would give the defaults, or one kind described to cover every name. The ruling's substance, no depth option, stands.
    - Fallback if Ian keeps the ruling. Item 4 is withdrawn. `recognize.md` keeps "One kind gives every detected name that kind, whatever the name is." as settled behavior. R2 builds nothing for it, test 12 and its runs drop, and the three re-recordings drop.
11. **Proposed: every concrete relation asks yes/no pairs.** This answers review item 2 and waits for Ian's ruling, because it contradicts his hybrid planner. A choice's probabilities total one, so at the default cut one asking name passes at most one option, or two in an exact tie. The live run on "John Lennon wrote Help!, Girl and In My Life." kept 1 of 3 stated edges. The side that asks depends on counts, so adding an unrelated name changed 0 edges into 3 in the replay above. A yes/no pair question has neither fault. Each pair gets its own probability, and adding a name adds questions without changing any other pair's question.
    - Why the superseded method comes back. Line 200 of `relate-design.md` records that Ian's final direction replaced the all-H proposal with the hybrid. The record gives no measurement behind that choice. The review's live run and replay show the cost the hybrid pays for one-to-many facts, which the bench's relate recall of 0.614 also shows on duets. This is new evidence for Ian to weigh, not a claim that his ruling was wrong.
    - Cost. Questions grow from the larger side's count to the product of both sides. Bytes grow little, because a choice question lists every option with its description while a pair question names two ids. The dry runs above moved by −13% to +38% and stayed at one request. Packed yes/no questions billed about 73 input tokens each in experiment 260 (section 2). By that rate, 10 persons and 50 songs add 450 questions, at most about 33,000 input tokens and $0.0014. It is an upper bound, because the 50 choice questions it removes also cost tokens. A paragraph with a few names adds a few hundred tokens, small beside 300 a word for finding the names.
    - The token limit. A pair question is short in bytes and long in tokens. The 5-by-180 pair plan is 900 questions in 79,856 bytes, under the 96,000-byte ceiling, so today's rules send it as one request. At 73 tokens a question that is about 65,700 input tokens, over Jev's 65,536. So item 5 adds a question ceiling: at most 400 questions in a relation request at the built-in address, and a profile's `max_questions` replaces it. 400 questions at 73 tokens is about 29,200 tokens. That leaves room for the fixed part and for evidence up to Jev's 32,000-token evidence limit (section 7). The ceiling uses the existing splitter, so it needs no new mechanism. ADR 0051's halving would also answer it, but it is not on main and it pays for a refused request first.
    - The ceiling's proof. Test 13's dry run shows the 5-by-180 plan split into 400, 400 and 100 questions. Test 13's live run reports tokens a pair question. If 400 times that rate, plus 32,000 and 300, passes 65,536, R5 lowers the ceiling to fit before it lands and records why.
    - Speed. No round trip is added. A plan splits only at the ceilings, and `--jobs` sends the split requests at once, by ticket 0143.
    - Simpler. One method replaces two. The choice planner, the asking-side rule and the option-ceiling fallback go. `relate --dry-run` keeps `method` and `fallback` so its schema holds.
    - Wording. The recognize pair question is the form experiment 265 measured: stated edges 0.84 to 0.99 and unstated edges 0.02 to 0.05 (section 10). The unmeasured pick-one wording in design section 5 is not needed, so R5 measures one wording, not two.
    - Accuracy risk. Independent yes/no answers may pass more false edges than a pick-one. Test 7 guards precision for `recognize`. Standalone `relate` has no key in this repository, so its precision after R5 is unmeasured. The bench's relate figure measured recall held down by this limit, and its owner reruns it after R5.
    - Rejected: a declared one-to-many or one-to-one mark per rule, because it adds a public key `relate-design.md` line 88 rules out, and a wrong mark still loses edges silently. The `tag` shape, because `tag.md` sends each label as its own yes/no question, so it is pairs with more words. A runner-up question, because it adds a round trip, and `relate-design.md` line 46 keeps no runner-up question. A lower cut, because three true options at 0.33 each sit where noise sits.
    - Fallback if Ian keeps the hybrid planner. Item 5 is withdrawn, and the choice, its asking side and its fallback stay. `recognize.md` and `relate.md` keep the one-edge limit sentence as settled behavior. R5 builds only item 6, with design section 5's pick-one wording, and test 13 drops.
12. **R2 and R5 each file one issue for the marketing lead.** The marketing lead owns `site/`, so neither ticket edits it. On Ian's word for item 4, R2 files an issue in `sdlc/issues/` naming `site/recordings/008f06758743d354febec20448e67f0f6c5151bae64995491d1b175aa8f46e84.json` and the same file under `site/examples/beatles/bench/examples/recognize/recording/`. Their kind questions gain `none`. On Ian's word for item 5, R5 files an issue naming the two files under `site/examples/beatles/bench/examples/relate/recording/`. Their choice questions become pair questions, and the bench's relate figure changes. The marketing lead re-records them under its own authorized run.

## Edge cases

| Case | What R0 does |
| --- | --- |
| Ticket 0146 lands first | The builder merges `origin/main` before the final run, and each R0 hunk sits on a line 0146 does not touch. A conflict means stop rule 6 |
| `settings.md` table | Untouched. The recognize lines stay under "Settings on the way", by decision 3 |
| `question-file.schema.json` and `recognize_file.rs` key lists | Untouched. `sdlc/scripts/settings` reads both, and R2 adds the keys with their parser |
| The design issue and the architect review 10 issue | Untouched, by decision 8. The review issue stays open for its other items |
| Ian rules on items 4 or 5 before R0 is built | The builder writes his ruling: an accepted item with `Not built yet` markers, or a withdrawn item with its limit sentence as settled text |
| Ian rules after R0 lands | One follow-up commit, owned by the queue owner, swaps each `Proposed by ADR 0050` marker for `Not built yet` or deletes it, and updates the ADR's item |
| A rule the design states and Ian's rulings do not cover | The ADR copies it. The ADR's status line says Ian can overturn each item and every rule it copies, and the rule appears among the design author's calls or ticket 0147's calls |
| A number in the ADR | It names its evidence section, experiment or this ticket's check |
| A marked sentence a building ticket forgets to delete | The grep lists it. The ADR's ticket table names the ticket that removes it |
| `recognize.md` line 7 or 21, which several items change | One marker paragraph after each line names every item that reaches it |
| Another branch takes ADR 0050 first | Stop rule 5 |
| The word "decision model" | It never appears. `the_specification_defines_unresolved_once_and_keeps_the_closed_wording` bans it on every specification page |
| A caller already uses a kind named `none`, `None` or `NONE` | Under item 4, R2 refuses it. The ADR says so, and R2's changelog line names it |
| Twenty kinds under a profile with `max_options` 20 | Under item 4, exit 2 before any request, naming 21 options and the limit |
| A same-kind relation | Item 5 keeps its pair questions and wording in `relate`. `recognize` gains the stated-text wording by item 6 |
| `--either` on a different-kind relation | Under item 5, one pair question between the two names. `recognize` asks "Does the text state that the relation holds between i1 and i2?" |
| ADR 0051 on ticket 0154's branch | It says every address gains a ceiling, which retires item 7's "one piece at an address with no ceiling" and would carry item 5's question ceiling to every address. Its "Overlap with the recognize ADR" section already orders the two. R0 copies the design's rule as it stands |

## Proof

R0 adds no test. It changes no behavior, so a new test would check prose. The building tickets carry the design's tests 1 to 11 and, on Ian's word, the ADR's tests 12 and 13, each against the real command.

The proof is the review plus the checks that already guard these pages:

- The reviewer reads the ADR against the design's sections, Ian's rulings and decisions 10 and 11, item by item, and each amended line against the amendment table. Every changed page keeps today's sentence beside its marked rule. Items 4 and 5 carry the proposed mark and their fallbacks.
- `grep -rn -e "Not built yet, by ADR 0050" -e "Proposed by ADR 0050" specification` gives one hit for each marked passage in the amendment table and no other. The limit grep gives one hit on `recognize.md` for one kind, and one each on `recognize.md` and `relate.md` for one edge. The reviewer checks each hit names the right item.
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

- ADR 0050: at most 160 nonblank lines.
- ADR 0040: at most 8 added or changed lines.
- Ticket 0080, `recognize-design.md` and `relate-design.md`: at most 10 changed lines in all.
- The nine specification pages: at most 70 added or changed lines in all.
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

Excluded: any code. The `settings.md` table rows, by decision 3. The schema and the recognize file's key lists, which R2 changes with its parser. The design issue and the architect review 10 issue. The recognize how-to page, which R8 writes. `site/`, which the marketing lead owns. Ian's rulings on items 4 and 5, which the coordinator asks for. Review 10's other items: kinds steering detection, the published accuracy, and its severity 3 items stay in its issue.

## Routing

Owner and builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for the diff. No ceiling moves and no public surface widens.

## Complexity

Contract 3; state and timing 0; reach 2; proof 0; cost of error 1; total 6. Final level: 1. The risk is an amendment that misstates a ruling or a proposal. The item-by-item review guards it.

## Deferred gaps

- The marked sentences and limit sentences stay on the pages until the tickets that build their items land, or as settled text if Ian keeps his rulings. The last recognize ticket empties the marker grep.
- Ian's rulings on items 4 and 5.
- The recognize settings move into the `settings.md` table with R2, R3, R4, R4b and R6.
- `question-file.schema.json` and `recognize_file.rs` gain `words`, `boundary` and `window` with R2, R3 and R4.
- The design issue's R4b row still names R4 as a dependency, and its section 6 still says pieces go one at a time. Its section 4 and R5 row predate items 4 and 5. The ADR's items and ticket table govern.
- The evidence-offset key names of a split request wait for R4.
- `recognize` over records under B4's `batch` row waits for R7.
- Windowed relations stay a known gap, as the design says.
- Standalone `relate` precision under pair questions is unmeasured until someone runs a key.
- One kind's token cost and a pair question's token rate are estimates until tests 12 and 13 report them.
- The site's recognize and relate recordings wait for the marketing lead, by decision 12.

## What Ian can overturn

- Every ruling and author's call the ADR copies. The design's "Open items" list names fifteen.
- Decision 3: the recognize settings under "Settings on the way", not as marked table rows.
- Decision 4: R4b depending on R0 alone.
- Decision 5: R6 carrying `window`.
- Decision 6: the spellings of the batched `confirm` request and the dry-run bound.
- Decision 7: each digest key entering with its building ticket.
- Decision 12: the two issues for the marketing lead.
- ADR item 8: a text over the cap in record mode refuses that record.
- ADR item 10: a text too big for a batch of one goes alone, as one whole-text request until R4 lands. R7 keeps its dependencies.

Ian decides decisions 10 and 11 outright. Each sets aside a ruling of his, and each states its fallback.

## Closes

None. `sdlc/issues/2026-09-26-recognize-design.md` stays open until its last ticket lands. `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md` stays open. On Ian's word, R2 answers its item 1 and R5 its item 2. Under a fallback, the plain limit sentence answers the item.

## Evidence

- Starts from: Ian's rulings and the R0 row in `sdlc/issues/2026-09-26-recognize-design.md`, sent 2026-09-26. Items 1 and 2 of `sdlc/issues/2026-09-26-architect-review-10-recognize-and-relate.md`, from local experiment 273, report 10, and the re-review of this ticket. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` sections 2, 7 and 10 to 13, from experiments 260, 265, 267, 270 and 271. Ticket 0139 and ADR 0048, which set the form, and ADR 0053, which amends it. Tickets 0143 and 0144 on main, and tickets 0145 and 0146 read from their branches, for the R4b and R7 check. The page lines in "What happens today" at `origin/main` `c490f082`. This ticket's offline replay and dry runs of main's binary at that commit, which sent nothing.
- Keeps: Every behavior, help line, fixture, schema, digest and test. Every accepted ADR sentence, marked and not deleted. Every Settled page's current sentence, beside its marked replacement. The `settings.md` table. Ian's 2026-09-21 ruling and hybrid planner, until he rules.
- Changes: A new ADR 0050 holds the recognize rulings and two proposals awaiting Ian's ruling: kinds decline through `none`, and every relation asks yes/no pairs under a question ceiling. ADR 0040, ticket 0080, the 2026-09-21 and 2026-09-23 rulings and the relate design carry markers, and ADR 0040 gains a dated amendment section. Nine specification pages state each new rule beside today's, under the marker, and state the one-kind and one-edge limits plainly. The `settings.md` recognize lines cite their ADR items.
- Proof: The item-by-item review against the design, the rulings, decisions 10 and 11 and the amendment table. The marker grep, the limit grep and the `spec/` grep. `lint`, the settings check and the specification wording test, with plants (a) to (c) each turning one red.
- Defers: Ian's rulings on items 4 and 5. The table rows to R2, R3, R4, R4b and R6. The schema and key lists to R2 to R4. The evidence-offset key names to R4. Removing each marker and limit sentence to the ticket that builds its item. Windowed relations. Standalone `relate` precision under pairs. The site recordings, to the marketing lead.
