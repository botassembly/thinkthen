# ADR 0056: Recognize is three steps: BILOU boundaries, labels, then stated relations

- Status: Accepted 2026-09-27 by the coordinator after five rounds of fresh read-only reviews, on Ian's rulings of 2026-09-26. Ian can overturn it. Ticket 0147 built it, landed 2026-09-27
- Date: 2026-09-26

This ADR replaces the method in `sdlc/issues/2026-09-26-recognize-design.md` sections 1 to 4 and the pieces and windows of its section 6. It keeps that design's stated relations, its `--jobs` ruling and its batch rules. Its hard cap of 600,000 bytes becomes a default the caller can raise, which changes the design issue author's call 6, and the refusal's message changes to name the new option. Ian can overturn that change. It keeps Ian's four kinds rulings of 2026-09-26 that ticket 0147 recorded. Local experiments 278 to 288 hold the measurements, and `sdlc/records/2026-09-26-recognize-three-step-evidence.md` copies the tables this ADR cites.

## Context

`recognize` on main asks two questions per word. One asks whether the word is part of a name. The other asks which kind. Word rules decide what a word is, and a run of name words becomes one name. The design issue planned five word lists, a trim rule, a keep rule and a `confirm` question on top of that. It rejected a BILOU scheme because experiment 270's three-way begin, inside or outside question lost titles that start with `The`.

Ian ruled on 2026-09-26: "No rule-based systems. Only use the agent to figure out where the boundaries of the starting and ending of entities are." He also ruled: "A simple approach is worth at least 3-5% over a complex, brittle idea. Don't overfit!"

Local experiment 278 then measured a five-option BILOU question per piece with no word rules. It beat the rule-based baseline on the new half of the 200-sentence key by 5.3 to 12.4 F1 points, and it tied on the first half. It returned no stray `The` name in any run. Experiment 270's three-way question returned 19 over three runs.

Ian ruled on the result: "Dame Judi Dench, a doctor, a junior, and all these other things are fine as possible cases. The model can't do better as long as we're splitting smartly, getting solid counts, labeling them, joining them, and getting the clean entity name that spans location: the start, the offset, the length, and the kind. Then I'm good. I just want to make sure it's doing a reasonably good job, it's simple, and it's the three-step process using BILU labeling and relationships."

## Decision

### Step 1: boundaries

1. **The text splits into pieces.** White space separates pieces. Each punctuation or symbol character, Unicode general category P or S, is a piece of its own. A run of characters of category Mn, Me or Cf joins the piece that ends right before it, whatever that piece is. A run after white space or at the text's start begins a piece, as a letter does. There are no word lists, no trim and no keep rule.
2. **Each piece gets one pick-one question.** Its options are `BEGIN`, `INSIDE`, `END`, `SINGLE` and `OUT`. The question names the caller's kinds. With no kinds it names no kind. The wording is local experiment 279's q3 form, labelled when the caller gives kinds and generic when not. The snippet shows six pieces on each side of the marked piece.
3. **Names come from the most likely valid sequence.** A Viterbi decode over the five tags keeps only valid sequences: `OUT` and `SINGLE` stand alone, and a `BEGIN` runs through any `INSIDE` pieces to one `END`. A probability under one in a million counts as one in a million. The decode runs over the whole text after every step-1 request returns, so a name may cross a request edge. No repair follows the decode.

### Step 2: labels and edges

4. **Each found name gets one pick-one kind question.** The options are the caller's kinds in order, then `none of these`. A name whose answer is `none of these` is dropped. With no kinds this question is not asked, and every name takes the kind `ENTITY`.
5. **A caller may describe each kind in one line.** `--kind KIND=DESCRIPTION` already carries it, and the question file's `recognize.kinds` object already holds it. The description goes into the step-2 option for that kind. Step 1 names the kinds only. Descriptions added 2.2 to 2.7 points on the new half and changed nothing on the first half (local experiment 279). On the key's own names at five kinds they lifted held-out F1 from 92.2 to 94.0, better in all three runs, fixing 13 names and breaking 2. They failed local experiment 282's keep rule at three and ten kinds. The key's descriptions are its own label lines, so part of the lift restates the key. They tied on public text (local experiment 284). Bare kinds stay the default, and the product writes no default descriptions.
6. **The same request asks one edge question per found name.** Its options are the name as found, the name with a touching punctuation or symbol piece added at either end, and the name with a punctuation or symbol piece removed from either end. The options come only from piece adjacency. A name with one option gets no edge question. The picked stretch becomes the name, and it keeps its step-2 kind. The wording is local experiment 279's edge wording a.
7. **The step-2 wording is local experiment 278's `none2`.** Local experiment 284 tested a broader wording and a second way out on the key's own names. The broader wording fell on the held-out half at three and five kinds and added 3.7 and 9.7 extra names a run there. It gained at ten kinds, 93.4 against 92.5, but that gain sat in one sentence about drinks. The second way out fell 2.0 to 4.1 held-out points. So `none2` stays with its one way out.
8. **No hidden kinds.** `web address`, `email address` and `social handle` are ordinary kinds. A caller may ask for them, and they get no special handling. A run that does not ask for them does not offer them. Asked for directly, they took the right kind 92% to 100% of the time (local experiment 286).

### Step 3: relations

9. **Only pairs a relation rule allows are asked.** Each ordered pair of kept names whose kinds match some rule's source and target gets one yes/no question: "Does the text itself state that i1 READS i2?", where READS is the rule's reading. A text's pair questions share one request, with the whole text as evidence. A request splits only at ADR 0040's ceiling and at 400 questions. A pair at or above the relation cut of 0.5 becomes an edge. Names with equal text and kind are asked once, and the edge names the first.
10. **The kinds rulings stand.** No kinds means every name has the kind `ENTITY` and only detection runs. A bare rule `knows` means `knows=*:*`. `*:ORG`, `PERSON:*` and the alias `ANY` work, and `*` and `ANY` expand only to the kinds of kept names. `none of these`, `ENTITY` and `ANY` are reserved kind names in any ASCII case.

### Windows, output and the guard

11. **Every text uses one fixed window of six pieces each side.** A step-1 request holds at most 40 piece questions. Its evidence is the original text from six pieces before its first piece to six pieces after its last. A step-2 request covers the names whose first piece sits in one step-1 request. Its evidence runs from six pieces before its first name to six pieces after its last, and each question shows six pieces each side. No overlap or merge rule runs. The same rule holds at every text length, so a paragraph and a book follow one path.
12. **Each name prints its text, place, kind and strength.** Bare output is `{"entities":[{"text":…,"start":…,"end":…,"length":…,"kind":…,"strength":…}]}`. `start` and `end` count Unicode scalar values into the original text. `end` is exclusive, and `length` is `end` minus `start`. `--details` keeps every tag, kind, edge and pair probability.
13. **Each name keeps one ranking score, `strength`.** `strength` is P(kind) times P(span), rounded to four decimal places as today. P(kind) is step 2's probability of the chosen kind. P(span) belongs to the step-1 stretch of pieces, before any edge pick. It is the summed weight of the valid BILOU paths that tag exactly this stretch as one name, divided by the summed weight of all valid paths. Each tag probability is first floored at one in a million, with no renormalization, and a path's weight is the product of its floored tag probabilities. This is local experiment 287's form, computed by one forward-backward pass in log space. With no kinds, `strength` is P(span). When the edge pick leaves two names with the same start, end and kind, the one with the higher strength stays. `--threshold` keeps a name whose printed strength is at or above the cut, so `audit` rescoring a saved line matches a live run. The default cut is 0.5. The cut stays above 0, and a low cut such as 0.01 keeps nearly every name. `strength` ranks names well, but it is not calibrated across kinds of text, so the pages call it a ranking score and not a probability. The field keeps today's name, so the libraries, SQL and `audit` keep their contract.
14. **A text over 600,000 bytes is refused by default.** It exits 2 before any request, and the message names the size and the limit and echoes no text. The guard caps spending. A caller raises it with `--max-text-bytes N`.

## Measured results

Figures are F1 on exact offsets and kind. Key figures are means of three runs with their min-to-max range unless marked. The public-set rows ran once, and their ranges are bootstrap 95% ranges of that one run. Each public-set row names its step-1 wording and snippet. "The shipped setup" is q3 at six pieces, step 2 `none2`, the edge question and the `strength` cut at 0.5. "267" is the key's first 100 sentences and "277 new" its second 100.

| Measure | Result | Local experiment |
| --- | --- | --- |
| Three steps against the rule-based baseline, new half | +5.3 at `person`, +12.4 at three kinds, +12.0 at five, +12.0 at ten, +7.9 with no kinds | 278 |
| Three steps against the baseline, first half | +3.0 with no kinds, +1.2 at three, -0.7 at five, +0.9 at ten | 278 |
| Edge question added, first half | +4.5 at three kinds, +3.4 at five, +3.3 with no kinds. It fixed `U.S.`, `Jr.` and `Inc.` and broke no name | 279 |
| Edge question added, `person` on the first half | 91.7 (89.8 to 92.6), against the baseline's 91.5 | 279 |
| Full public test split, Universal NER English EWT, 2,077 sentences and 1,088 names, the shipped setup, one run | 80.1, bootstrap 95% range of the one run 77.8 to 82.3, at 308 input tokens a word, against 77.7 for the rule-based baseline. The published fine-tuned XLM-R Large scores 85.8 (Mayhew et al., NAACL 2024), trained on 12,543 labelled sentences | 288, baseline 281 |
| WNUT-17 test set, six kinds, the shipped setup, one run | 59.1 entity F1, bootstrap 95% range of the one run 56.2 to 61.9, at 308 input tokens a word. The best 2017 shared-task system scored 41.86 and zero-shot gpt-3.5 46.61. Fine-tuned encoders score 56.5 to 57.1 | 288, published figures 281 |
| The same two sets with p1 at six pieces, otherwise the shipped setup, one run | Full public split 79.9 (77.6 to 82.2), paired difference from q3 -0.2 (-1.3 to +1.0). WNUT-17 59.6 (56.6 to 62.2), paired difference +0.4 (-0.8 to +1.7). 383 to 385 input tokens a word | 288 |
| Context: p1 on the whole sentence, one run | Full public split 78.6 with no cut and 81.0 at the 0.5 cut. WNUT-17 53.4 with no cut and 60.1 at the cut | 281, 287 |
| Six public documents of 371 to 678 words | p1 at six pieces 83.1, mean of three runs, range 83.0 to 83.2, at 395 input tokens a word. q3 at six pieces 83.9, range 83.5 to 84.3, at 322 tokens a word. The whole text 81.6, one run, at 1,563 tokens a word. p1 on one sentence at a time 85.6, one run, at 409 tokens a word, using the gold sentence breaks | 285 |
| Relations on 30 sentences with 27 stated edges | 22 of 27 stated edges found from the pipeline's own names, 26 of 27 from the key's names. None of the five unstated edges passed | 278 |
| Relations on 60 new sentences with 70 stated edges, from the key's own names | 66 to 68 found, one inferred edge a run, and no negated or hypothetical edge | 284 |
| `strength` cut at 0.5, against no cut. One run of each public set, means of three runs for the key and the documents | Full public split, p1 on the whole sentence, 78.6 to 81.0 (paired bootstrap +1.6 to +3.4). WNUT-17, p1 on the whole sentence, 53.4 to 60.1. Long documents, p1 at six pieces, 83.1 to 84.3. The 200-sentence key, p1 on the whole sentence, 87.3 to 87.2 | 287 |
| `strength` calibration | Expected calibration error 6.7 to 19.5 across the four sets. Names scored 0.5 to 0.6 were right 26% to 76% of the time, by set | 287 |

The shipped setup scores 80.1 on the full public split against the rule-based baseline's 77.7, with no string rule. The two ran in different experiments, so no paired range compares them. The six-piece window sits about 1 point below the whole sentence on the full public split and 0.5 below on WNUT-17, comparing local experiment 288's p1 run with local experiment 287's whole-sentence figures across runs. On local experiment 285's documents, one sentence at a time sat 2.5 points above p1 at six pieces, on one run. Errors land near request edges at the rate pieces sit there, so request edges cost no measured accuracy (local experiment 285).

## Rejected

- **Experiment 270's three-way begin, inside or outside question.** It returned 19 lone `The` names over three runs and lost titles that start with `The`.
- **Word rules and a trim step (local experiment 276's design).** They are string rules, which Ian ruled out. White-space pieces with a model trim lost 5.0 to 8.6 points on the new half, by kind set (local experiment 279).
- **Labelled detection alone, with no step 2.** At one bare kind it lost beyond the noise (local experiment 274). A labelled step 1 still passes names of unasked kinds (local experiment 279), so step 2 declines them.
- **A span check that widens the edge question (local experiment 283).** Its best arm gained 0.9 points on the held-out half against its paired control. Run 1 sat inside the control's range, the gain rested on two names, and it cost 3 to 4% more tokens. The simplicity rule decided it.
- **The longer p1 step-1 wording.** At six pieces it ties q3 on F1 on both full public sets. It scored 79.9 against 80.1 on the full public split, a paired difference of -0.2 (-1.3 to +1.0), and 59.6 against 59.1 on WNUT-17, +0.4 (-0.8 to +1.7). It costs 20% more input tokens, 383 to 385 a word against 308. p1 returns fewer extra names, 23 fewer on the full public split and 54 fewer on WNUT-17. Its organization precision is higher by 2.4 and 7.6 points, where WNUT-17's organization kind is `corporation`. q3 finds 16 and 27 more right names. An extra is a returned name that overlaps no gold name. Local experiment 281's loss of 1.9 points for q3 compared q3 at six pieces with p1 on the whole sentence. Local experiment 288 shows the window caused that loss, not the wording. The choice rests on cost at equal F1 (local experiment 288).
- **The whole text as every question's snippet.** Its cost grows with the square of text length. On public documents it scored 1.5 points below p1 at six pieces and cost four times as much (local experiment 285).
- **A twelve-piece window.** It gained 0.3 points with overlapping ranges and cost 6% more (local experiment 285).
- **A title option in step 2.** It offered each `person` name without its first word as a title. It gained 5.4 to 8.6 points on the new half, by kind set. It also cut `Duke of Wellington` to `of Wellington` and turned `Mona Lisa` into a person `Lisa` in every run (local experiment 279). Ian accepted titles as a known limit.
- **A one-name-or-two question.** It is a new question type. It split `Portland Oregon` and `Dallas Texas`, and it never split two first names such as `Paul John` (local experiment 279). The touching group is worth at most about 2 points on the key (local experiment 282). Ian accepted touching names as a known limit.
- **Today's strength form, the lowest tag probability times the kind probability.** It is better calibrated on the public sets: expected calibration error 3.1 against 12.1 on the full public split and 11.2 against 19.5 on WNUT-17. It also had higher average precision on the key, 83.2 against 82.3. At 0.5 it scored 84.0 on the key and 79.8 on the long documents, against 87.2 and 84.3 for the chosen score, 3.2 and 4.5 points lower. It had lower average precision on every public set, and it scored right names such as `The Who` near zero. The choice rests on F1 at the 0.5 cut and on public-set average precision (local experiment 287).
- **P(kind) alone as the score.** The product beat it by 2.8 to 3.4 average-precision points on the public sets. At 0.5 it beat it by 1.4 to 4.1 F1 on the public sets and the long documents. On the key P(kind) alone scored 87.7 against 87.2 (local experiment 287).
- **A broader step-2 wording.** It fell on the held-out half at three and five kinds, 91.7 against 92.3 and 91.8 against 92.2, and added 3.7 and 9.7 extra names a run there. At ten kinds it gained, 93.4 against 92.5, and added no extras, but the gain sat in one sentence about drinks (local experiment 284).
- **Two way-outs in step 2.** Held-out F1 fell 2.0 points at three kinds and 4.0 to 4.1 at five and ten. It broke 19 to 34 real names, such as bands and products taken as a name of some other kind. Its gain on the 100 public sentences came from dropping extras and rested on under two names (local experiment 284).
- **Hidden decoy kinds.** Step 2 offered `web address`, `email address` and `social handle` unasked and dropped names picked as them. With p1 on the whole sentence and no cut, the full public split rose from 78.6 to 79.7 and WNUT-17 from 53.4 to 55.2. It also dropped 127 real WNUT-17 names, mostly handles that the gold tags as persons. Decoys in both steps gained on the public split only (local experiment 286).
- **Averaging the generic and labelled step 1.** It doubles step 1. It gained 0.3 and 1.9 span points on one pair of already paid runs (local experiment 278), and at most 0.3 F1 by local experiment 282's estimate.

## Accepted errors

These errors remain, and the design accepts them.

- **Titles and honorifics.** `Dame Judi Dench` and `Sir Paul McCartney` keep their titles, and a key that leaves titles out counts them wrong. Ian ruled these fine.
- **Two names of one kind side by side.** `Paul John` stays one name. Step 1 splits most other touching pairs, but not two first names.
- **Weekday names with no kinds.** `Saturday` and `Sunday` come back as names. With kinds, step 2 declines them.
- **Kinds that depend on use.** `Washington` as a team or a government comes back as a place. No arm of local experiment 284 fixed capitals and teams on 40 new cases. Brands and companies, and works named after people, swap kinds. A caller's descriptions help.
- **`Drakeʼs`.** U+02BC is a letter in Unicode, so the apostrophe does not split. A fix needs a string rule for one name.
- **Nationality and language words with no kinds.** Step 1 misses some. A fix adds a step for at most 1.6 points.
- **Pieces of web addresses, email addresses and handles.** On web and social text they come back as names: 55 of the 245 extra names on the public split, and about 440 on WNUT-17 before the cut, both with p1 on the whole sentence (local experiment 281). Asked for as kinds, whole addresses came back only 52% to 83% of the time, because pieces split at marks: `t.co`, `https`, a lone `@` (local experiment 286).
- **`Help!` in relation texts.** The model picks `Help` over `Help!` in the relation texts, and the edges on it attach to `Help`.

## Consequences

- Ticket 0147 builds all three steps, the window, the output shape and the guard on the command line, the question file, the libraries and SQL, and `audit`.
- The design issue's word lists, `confirm`, `boundary`, `--window` and its strength formula are not built.

The design issue's rows, R0 to R8:

- R0, the ADR of Ian's rulings, is replaced by ADR 0056.
- R1, the keys and their grading, is built by ticket 0164 and stays as it landed.
- R2, R3 and R4 retire. Word rules, `confirm` and windowed pieces with `--window` are not built.
- R4b, `recognize --jobs` on one text, stays and builds on ADR 0056.
- R5, stated relations, is built by ticket 0147.
- R6 retires. ADR 0056 removes `keep`, `infixes` and `boundary`, and ticket 0147 carries `--max-text-bytes` itself.
- R7, batching texts, stays and builds on ADR 0056.
- R8, the manual page, stays and builds on ADR 0056.

- Standalone `relate` keeps its planner. Moving it to yes/no pairs under Ian's ruling of 2026-09-26 stays a separate ticket.
- `audit` grades `recognize` as `specification/audit.md` says today. It reads each name's `strength` and the run cut and prints counts, precision, recall, F1, the suggested bar and the crossed line. Calibration, AUC and the coverage curve stay null, because `strength` is a ranking score and not a probability. `--write` puts a steady bar into the recognize question file. One rule is new: a line with an empty kind set grades every said and key name as the kind `ENTITY`.
- Ticket 0165 teaches `diff` to read `recognize` and `relate` once ticket 0147 fixes the output shape.
- Every recognize request body and question digest changes, so every recognize recording is made again.

## What Ian can overturn

Ian's rulings of 2026-09-26:

1. Three steps with BILOU boundaries and no word rules.
2. Titles, two first names side by side and the other accepted errors staying as they are.
3. The kinds rulings of item 10.

Owner calls, adopted with the ruling:

4. The q3 step-1 wording over p1, from local experiment 288. It ties p1 on F1 on both full public sets and costs 20% fewer tokens. p1 trades that cost for fewer extra names and better organization precision. The six-piece window, from local experiment 285.
5. The edge question sharing step 2's request.
6. The field name `text`, and `strength` as P(kind) times P(span) with a default cut of 0.5.
7. The guard's name `--max-text-bytes`, its default of 600,000 bytes, and making the design issue's hard cap a default the caller can raise.
8. Rejecting the span check of local experiment 283 by the simplicity rule.
9. No hidden decoy kinds, from local experiment 286.
10. `none2` with one way out, over a broader wording and two way-outs, from local experiment 284.
