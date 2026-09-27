# ADR 0056: Recognize is three steps: BILOU boundaries, labels, then stated relations

- Status: Accepted 2026-09-26 by Ian's ruling. Ian can overturn it. Ticket 0147 builds it
- Date: 2026-09-26

This ADR replaces the method in `sdlc/issues/2026-09-26-recognize-design.md` sections 1 to 4 and the pieces and windows of its section 6. It keeps that design's stated relations, its `--jobs` ruling, its batch rules and its hard cap. It keeps Ian's four kinds rulings of 2026-09-26 that ticket 0147 recorded. Local experiments 278 to 283 and 285 hold the measurements.

## Context

`recognize` on main asks two questions per word. One asks whether the word is part of a name. The other asks which kind. Word rules decide what a word is, and a run of name words becomes one name. The design issue planned five word lists, a trim rule, a keep rule and a `confirm` question on top of that. It rejected a BILOU scheme because experiment 270's three-way begin, inside or outside question lost titles that start with `The`.

Ian ruled on 2026-09-26: "No rule-based systems. Only use the agent to figure out where the boundaries of the starting and ending of entities are." He also ruled: "A simple approach is worth at least 3-5% over a complex, brittle idea. Don't overfit!"

Local experiment 278 then measured a five-option BILOU question per piece with no word rules. It beat the rule-based baseline on the new half of the 200-sentence key by 5.3 to 12.4 F1 points, and it tied on the first half. It returned no stray `The` name in any run. Experiment 270's three-way question returned 19 over three runs.

Ian ruled on the result: "Dame Judi Dench, a doctor, a junior, and all these other things are fine as possible cases. The model can't do better as long as we're splitting smartly, getting solid counts, labeling them, joining them, and getting the clean entity name that spans location: the start, the offset, the length, and the kind. Then I'm good. I just want to make sure it's doing a reasonably good job, it's simple, and it's the three-step process using BILU labeling and relationships."

## Decision

### Step 1: boundaries

1. **The text splits into pieces.** White space separates pieces. Each punctuation or symbol character, Unicode general category P or S, is a piece of its own. A combining mark, variation selector or joiner right after a symbol stays with it. There are no word lists, no trim and no keep rule.
2. **Each piece gets one pick-one question.** Its options are `BEGIN`, `INSIDE`, `END`, `SINGLE` and `OUT`. The question names the caller's kinds. With no kinds it names no kind. The wording is local experiment 278's p1 form, labelled when the caller gives kinds and generic when not. The snippet shows six pieces on each side of the marked piece.
3. **Names come from the most likely valid sequence.** A Viterbi decode over the five tags keeps only valid sequences: `OUT` and `SINGLE` stand alone, and a `BEGIN` runs through any `INSIDE` pieces to one `END`. A probability under one in a million counts as one in a million. The decode runs over the whole text after every step-1 request returns, so a name may cross a request edge. No repair follows the decode.

### Step 2: labels and edges

4. **Each found name gets one pick-one kind question.** The options are the caller's kinds in order, then `none of these`. A name whose answer is `none of these` is dropped. With no kinds this question is not asked, and every name takes the kind `ENTITY`.
5. **A caller may describe each kind in one line.** `--kind KIND=DESCRIPTION` already carries it, and the question file's `recognize.kinds` object already holds it. The description goes into the step-2 option for that kind. Step 1 names the kinds only. Descriptions added 2.2 to 2.7 points on the new half and changed nothing on the first half (local experiment 279). Bare kinds stay the default.
6. **The same request asks one edge question per found name.** Its options are the name as found, the name with a touching punctuation or symbol piece added at either end, and the name with a punctuation or symbol piece removed from either end. The options come only from piece adjacency. A name with one option gets no edge question. The picked stretch becomes the name, and it keeps its step-2 kind. The wording is local experiment 279's edge wording a.
7. **Three choices stay open.** The coordinator fills each before ticket 0147's review.
   - **The step-2 wording.** Local experiment 278's `none2` wording is the measured default. Local experiment 284 tests a new wording on the key's own names, and a second way out. Any way-out label it adds is reserved as a kind, as `none of these` is.
   - **Built-in kinds for web text.** Local experiment 286 tests the built-in kinds `web address`, `email address` and `social handle`. Ian's rule: a caller who asks for one of these kinds gets those names with that kind. Otherwise the questions still offer the three kinds, and a name picked as one of them is dropped.
   - **The name score and its default cut.** Local experiment 287 compares three definitions offline: the step-2 kind probability, the step-1 span confidence, and their product. It picks the one best calibrated against the key, and it picks the default cut. With no kinds only the step-1 part exists.

### Step 3: relations

8. **Only pairs a relation rule allows are asked.** Each ordered pair of kept names whose kinds match some rule's source and target gets one yes/no question: "Does the text itself state that i1 READS i2?", where READS is the rule's reading. A text's pair questions share one request, with the whole text as evidence. A request splits only at ADR 0040's ceiling and at 400 questions. A pair at or above the relation cut of 0.5 becomes an edge. Names with equal text and kind are asked once, and the edge names the first.
9. **The kinds rulings stand.** No kinds means every name has the kind `ENTITY` and only detection runs. A bare rule `knows` means `knows=*:*`. `*:ORG`, `PERSON:*` and the alias `ANY` work, and `*` and `ANY` expand only to the kinds of kept names. `none of these`, `ENTITY` and `ANY` are reserved kind names in any ASCII case.

### Windows, output and the guard

10. **Every text uses one fixed window of six pieces each side.** A step-1 request holds at most 40 piece questions. Its evidence is the original text from six pieces before its first piece to six pieces after its last. A step-2 request covers the names whose first piece sits in one step-1 request. Its evidence runs from six pieces before its first name to six pieces after its last, and each question shows six pieces each side. No overlap or merge rule runs. The same rule holds at every text length, so a paragraph and a book follow one path.
11. **Each name prints its text, place, kind and score.** Bare output is `{"entities":[{"text":…,"start":…,"end":…,"length":…,"kind":…,"probability":…}]}`. `start` and `end` count Unicode scalar values into the original text. `end` is exclusive, and `length` is `end` minus `start`. `probability` is one calibrated score per name, defined by item 7. `--threshold` keeps a name whose score is at or above the cut, so a caller trades missed names against false ones and `audit` can suggest the cut. `probability` replaces `strength`, and it matches the name `relate` gives an edge's score. `--details` keeps every tag, kind, edge and pair probability.
12. **A text over 600,000 bytes is refused by default.** It exits 2 before any request, and the message names the size and the limit and echoes no text. The guard caps spending. A caller raises it with `--max-text-bytes N`.

## Measured results

Figures are F1 on exact offsets and kind, as the mean and range of three runs unless marked. "267" is the key's first 100 sentences and "277 new" its second 100.

| Measure | Result | Local experiment |
| --- | --- | --- |
| Three steps against the rule-based baseline, new half | +5.3 at `person`, +12.4 at three kinds, +12.0 at five, +12.0 at ten, +7.9 with no kinds | 278 |
| Three steps against the baseline, first half | +3.0 with no kinds, +1.2 at three, -0.7 at five, +0.9 at ten | 278 |
| Edge question added, first half | +4.5 at three kinds, +3.4 at five, +3.3 with no kinds. It fixed `U.S.`, `Jr.` and `Inc.` and broke no name | 279 |
| Edge question added, `person` on the first half | 91.7 (89.8 to 92.6), against the baseline's 91.5 | 279 |
| Full public test split, Universal NER English EWT, 2,077 sentences and 1,088 names, one run | 78.6 (76.3 to 80.9), against 77.7 for the rule-based baseline. The published fine-tuned XLM-R Large scores 85.8 (Mayhew et al., NAACL 2024), trained on 12,543 labelled sentences | 281 |
| WNUT-17 test set, six kinds, one run | 53.4 entity F1 (50.3 to 56.4). The best 2017 shared-task system scored 41.86 and zero-shot gpt-3.5 46.61. Fine-tuned encoders score 56.5 to 57.1 | 281 |
| Six public documents of 371 to 678 words | p1 at six pieces 83.1 (83.0 to 83.2) at 395 input tokens a word. The whole text 81.6 at 1,563 tokens a word | 285 |
| Relations on 30 sentences with 27 stated edges | 22 of 27 stated edges found from the pipeline's own names, 26 of 27 from the key's names. None of the five unstated edges passed | 278 |

The design matches the rule-based baseline's accuracy on the full public split with no string rule. Errors land near request edges at the rate pieces sit there, so the window costs no measured accuracy (local experiment 285).

## Rejected

- **Experiment 270's three-way begin, inside or outside question.** It returned 19 lone `The` names over three runs and lost titles that start with `The`.
- **Word rules and a trim step (local experiment 276's design).** They are string rules, which Ian ruled out. White-space pieces with a model trim lost 5 to 9 points on the new half (local experiment 279).
- **Labelled detection alone, with no step 2.** At one bare kind it lost beyond the noise (local experiment 274). A labelled step 1 still passes names of unasked kinds (local experiment 279), so step 2 declines them.
- **A span check that widens the edge question (local experiment 283).** Its best arm gained 0.9 points on the held-out half against its paired control. Run 1 sat inside the control's range, the gain rested on two names, and it cost 3 to 4% more tokens. The simplicity rule decided it.
- **The shorter q3 step-1 wording.** It cost 25% fewer tokens. On the full public split it lost 1.9 points against p1 (paired range -3.3 to -0.5), mostly in organization precision (local experiment 281). Its gain of 0.8 points in local experiment 285 rested on 297 names.
- **The whole text as every question's snippet.** Its cost grows with the square of text length. On public documents it scored 1.5 points below p1 at six pieces and cost four times as much (local experiment 285).
- **A twelve-piece window.** It gained nothing measurable and cost 6% more.
- **A title option in step 2.** It offered each `person` name without its first word as a title. It gained 5.3 to 8.6 points on the new half. It also cut `Duke of Wellington` to `of Wellington` and turned `Mona Lisa` into a person `Lisa` in every run (local experiment 279). Ian accepted titles as a known limit.
- **A one-name-or-two question.** It is a new question type. It split `Portland Oregon` and `Dallas Texas`, and it never split two first names such as `Paul John` (local experiment 279). The touching group is worth at most about 2 points on the key (local experiment 282). Ian accepted touching names as a known limit.
- **Averaging the generic and labelled step 1.** It doubles step 1. It gained 0.3 and 1.9 span points on one pair of already paid runs (local experiment 278), and at most 0.3 F1 by local experiment 282's estimate.

## Accepted errors

These errors remain, and the design accepts them.

- **Titles and honorifics.** `Dame Judi Dench` and `Sir Paul McCartney` keep their titles, and a key that leaves titles out counts them wrong. Ian ruled these fine.
- **Two names of one kind side by side.** `Paul John` stays one name. Step 1 splits most other touching pairs, but not two first names.
- **Weekday names with no kinds.** `Saturday` and `Sunday` come back as names. With kinds, step 2 declines them.
- **Kinds that depend on use.** `Washington` as a team or a government comes back as a place. Brands and companies, and works named after people, swap kinds. A caller's descriptions help.
- **`Drakeʼs`.** U+02BC is a letter in Unicode, so the apostrophe does not split. A fix needs a string rule for one name.
- **Nationality and language words with no kinds.** Step 1 misses some. A fix adds a step for at most 1.6 points.
- **Pieces of web addresses, email addresses and handles.** On web and social text they come back as names: 55 of 245 false names on the public split, and about 440 on WNUT-17 (local experiment 281). Local experiments 284 and 286 test fixes, by item 7.
- **`Help!` in relation texts.** The model picks `Help` over `Help!` in the relation texts, and the edges on it attach to `Help`.

## Consequences

- Ticket 0147 builds all three steps, the window, the output shape and the guard on the command line, the question file, the libraries and SQL, and `audit`.
- The design issue's word lists, `confirm`, `boundary`, `--window` and strength formula are not built. Its tickets R2, R3 and R4 retire. Its R4b (`--jobs` on one text), R7 (batching texts) and R8 (the manual page) stay, and each builds on this ADR.
- Standalone `relate` keeps its planner. Moving it to yes/no pairs under Ian's ruling of 2026-09-26 stays a separate ticket.
- `audit` keeps full support for `recognize`. It reads each name's `probability` and the run cut, prints calibration, AUC, the coverage curve, the suggested bar and the crossed line, and `--write` puts a steady bar into the recognize question file. Only the field name changes from `strength`.
- Ticket 0165 teaches `diff` to read `recognize` and `relate` once ticket 0147 fixes the output shape.
- Every recognize request body and question digest changes, so every recognize recording is made again.

## What Ian can overturn

Ian's rulings of 2026-09-26:

1. Three steps with BILOU boundaries and no word rules.
2. Titles, two first names side by side and the other accepted errors staying as they are.
3. The kinds rulings of item 9.

Owner calls, adopted with the ruling:

4. The p1 step-1 wording over q3, from local experiment 281, and the six-piece window, from local experiment 285.
5. The edge question sharing step 2's request.
6. The field names `text` and `probability`.
7. The guard's name `--max-text-bytes` and its default of 600,000 bytes.
8. Rejecting the span check of local experiment 283 by the simplicity rule.
9. The step-2 wording, the built-in web kinds, and the name score and its cut, once local experiments 284, 286 and 287 fill item 7.
