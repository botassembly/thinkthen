# Evidence for the three-step recognize design

Filed 2026-09-26. This record copies the measurements that ADR 0056 and ticket 0147 cite. Each section names the local experiment that measured it. The experiments stay unpushed on the owner's machine, so this record is the repository's copy. One model answered every paid call: `jev-1.13.0` behind `jev-latest`. Cost uses the recorded input price of $0.042 a million input tokens.

## Terms

- **The key** is the 200-sentence key of 372 names. "267" is its first 100 sentences and 168 names. "277 new" is its second 100 sentences and 204 names. Key sentences average 7.4 words.
- **T and H** are local experiment 282's halves of the key. Within each key category, in id order, odd positions tune (T) and even positions are held out (H).
- **The baseline** is local experiment 270's rule-based design: word rules, a trim, a kind question and a confirm question. It is the design issue's planned method, not the build on main.
- **UNER** is the Universal NER v1 English EWT test split: 2,077 sentences, 21,533 words and 1,088 names (449 PER, 322 ORG, 317 LOC), under CC BY-SA 4.0.
- **WNUT-17** is the WNUT-17 test set: 1,287 sentences, 23,394 words and 1,079 names over six kinds, under CC BY 4.0.
- **q3** is local experiment 279's shorter step-1 wording, which ticket 0147 quotes and ships. **p1** is local experiment 278's longer step-1 wording. **none2** is the step-2 wording ticket 0147 quotes.
- Each public-set figure names its step-1 wording and snippet. F1 is on exact Unicode scalar offsets and kind unless marked. "Mean (min to max)" is over three runs. A "95% range" is a bootstrap range over sentences of one run, 2,000 draws.

## 1. Local experiment 278: three steps against the baseline

Labelled step 1 with p1 and the whole text as snippet, then step 2 with none2. The baseline ran once. The design's F1 is min to max over three runs.

| Kinds | Key | Baseline F1 | Design F1 |
| --- | --- | --- | --- |
| 0 | 267 | 85.4 | 88.2 to 88.8 |
| 0 | 277 new | 79.6 | 87.1 to 87.8 |
| 1, person | 267 | 91.5 | 87.8 to 90.5 |
| 1, person | 277 new | 67.3 | 72.2 to 72.9 |
| 3 | 267 | 86.5 | 87.2 to 88.4 |
| 3 | 277 new | 71.5 | 83.0 to 84.7 |
| 5 | 267 | 88.3 | 87.0 to 88.1 |
| 5 | 277 new | 72.0 | 83.6 to 84.8 |
| 10 | 267 | 87.5 | 87.6 to 89.0 |
| 10 | 277 new | 72.9 | 84.7 to 85.4 |

Mean gains on 277 new: +5.3 at person, +12.4 at three kinds, +12.0 at five, +12.0 at ten and +7.9 with no kinds. On 267: +3.0 with no kinds, +1.2 at three, -0.7 at five and +0.9 at ten.

Step-1 snippet, generic wording, span F1:

| Snippet | Runs | 267 | 277 new | Tokens a word |
| --- | --- | --- | --- | --- |
| Two pieces each side | 1 | 86.7 | 85.4 | 355 |
| Six pieces each side | 1 | 86.8 | 87.3 | 361 |
| Whole text | 3 | 88.2 to 88.8 | 87.1 to 87.8 | 364 |

Stray `The` names: none in any three-step run. Experiment 270's three-way begin, inside or outside question returned 19 over three runs.

Step 3 on local experiment 265's 30 relation sentences, with 27 stated and 5 unstated edges, under the rules `sang=person:song`, `wrote=person:song`, `appears_on=song:album` and `recorded_at=album:place`:

| Names from | Stated edges found | Unstated edges passed |
| --- | --- | --- |
| The key's names, 3 runs | 26 of 27 each run | 0 |
| Steps 1 and 2, 3 runs | 22 of 27 each run | 0 |

Four of the five pipeline misses come from `Help!` found as `Help`. Averaging generic and labelled step-1 probabilities on one pair of paid runs gained 0.3 and 1.9 span points.

## 2. Local experiment 279: the edge question, descriptions and rejected arms

The edge question, wording a, on local experiment 278's recorded names. Mean (min to max):

| Kinds | Key | Baseline | 278 | 278 plus edge |
| --- | --- | --- | --- | --- |
| 0 | 267 | 85.4 | 88.4 (88.2-88.8) | 91.7 (91.2-92.3) |
| 0 | 277 new | 79.6 | 87.5 (87.1-87.8) | 88.0 (87.6-88.2) |
| 1, person | 267 | 91.5 | 89.6 (87.8-90.5) | 91.7 (89.8-92.6) |
| 1, person | 277 new | 67.3 | 72.7 (72.2-72.9) | 72.7 (72.2-72.9) |
| 3 | 267 | 86.5 | 87.7 (87.2-88.4) | 92.2 (91.2-93.2) |
| 3 | 277 new | 71.5 | 83.9 (83.0-84.7) | 83.9 (83.0-84.7) |
| 5 | 267 | 88.3 | 87.6 (87.0-88.1) | 91.0 (90.6-91.2) |
| 5 | 277 new | 72.0 | 84.0 (83.6-84.8) | 84.0 (83.6-84.8) |
| 10 | 267 | 87.5 | 88.4 (87.6-89.0) | 91.8 (91.2-92.1) |
| 10 | 277 new | 72.9 | 84.9 (84.7-85.4) | 85.2 (85.1-85.4) |

The same key with q3 at six pieces each side, step 2 with none2, then the edge question, with no cut. Mean (min to max):

| Kinds | Key | q3 plus edge |
| --- | --- | --- |
| 0 | 267 | 90.3 (89.8-90.9) |
| 0 | 277 new | 88.0 (87.7-88.4) |
| 1, person | 267 | 92.6 (92.6-92.6) |
| 1, person | 277 new | 77.1 (75.5-78.4) |
| 3 | 267 | 90.9 (90.6-91.3) |
| 3 | 277 new | 83.9 (83.3-84.4) |
| 5 | 267 | 89.4 (88.8-90.5) |
| 5 | 277 new | 83.6 (83.2-83.9) |
| 10 | 267 | 89.2 (88.7-89.9) |
| 10 | 277 new | 84.0 (83.2-84.5) |

Input tokens a word over the 200 key texts, run 1, steps 1 and 2: 284 with q3 against 364 with p1 on the whole sentence at no kinds, 331 against 386 at `person`, 341 against 419 at three kinds, 353 against 431 at five and 374 against 451 at ten.

The edge question fixed 6 or 7 names a run and broke none. It fixed `U.S.`, `U.N.`, `Washington, D.C.`, `Martin Luther King Jr.` and `Acme Widgets Inc.`. It did not fix `Help!` in the relation texts.

A 1,018-word invented text with a 65-name hand key, q3 wording, step-1 requests of 40 pieces with six pieces each side. Its step-2 and edge questions showed 30 pieces each side of the name, set in the experiment's long-text tool:

| Arm | Kinds | Runs | F1 with edge | Tokens a word |
| --- | --- | --- | --- | --- |
| Six-piece window | 0 | 3 | 92.2 (91.3-92.6) | 258 |
| Whole text as request state | 0 | 3 | 89.7 (89.2-89.9) | 294 |
| Whole text everywhere | 0 | 1 | 91.2 | 1,723 |
| Six-piece window plus step 2 | 5 | 3 | 93.7 (92.9-94.5) | 292 |

Rejected arms, mean F1 of three runs. The controls are the best design of that part: local experiment 278 plus the edge question for the trim, and the q3 design plus the edge question for the step-2 options.

| Arm | Key | Kinds | Control | Arm |
| --- | --- | --- | --- | --- |
| White-space pieces plus a model trim | 277 new | 0 | 88.0 | 83.0 |
| White-space pieces plus a model trim | 277 new | 1, person | 72.7 | 64.1 |
| White-space pieces plus a model trim | 277 new | 5 | 84.0 | 77.9 |
| White-space pieces plus a model trim | 277 new | 10 | 85.2 | 78.5 |
| Title option in step 2 | 277 new | 1, person | 77.1 | 85.7 |
| Title option in step 2 | 277 new | 3 | 83.9 | 89.3 |
| Title option in step 2 | 277 new | 5 | 83.6 | 89.2 |
| Title option in step 2 | 267 | 3 | 90.9 | 89.3 |
| Caller descriptions in step 2 | 277 new | 3 | 83.9 | 86.1 |
| Caller descriptions in step 2 | 277 new | 5 | 83.6 | 86.3 |
| Caller descriptions in step 2 | 267 | 5 | 89.4 | 89.7 |

The title option cut `Duke of Wellington` to `of Wellington` in every run, and at `person` it turned `Mona Lisa` into a person `Lisa` in every run. A one-name-or-two question fixed `Portland Oregon`, `Dallas Texas` and `Austin Texas` and never split `Paul John`, `Sarah Emily` or `Emma James`.

## 3. Local experiment 274: labelled detection on main

Main's detection question with one kind labels every detected name with that kind. On the 267 half, mean of three runs, main scored F1 27.0 at `person`, 32.0 at `place` and 27.4 at `work`, at 17% to 20% precision. It returned about 120 names of other kinds a run.

Labelled detection alone names the caller's kind in the detection question and asks no kind question. At bare `work`, one run, it scored 51.4. Main's detection plus a kind question that can answer `none` scored 67.5. On the song-and-album sentences at `person`, labelled detection alone scored 13 points under that kind question.

## 4. Local experiments 280 and 281: public test sets

Local experiment 280 scored the design on 100 sampled UNER sentences: 78.0 to 78.6 over two runs, against 66.0 for the baseline. Local experiment 280 used p1 on the whole sentence. Local experiment 281 scored the full sets once each.

UNER, one run each:

| Arm | Overall P / R / F1 | 95% range | ORG precision | Names returned | Right | Wrong | Extras | Tokens a word |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Baseline | 78.7 / 76.7 / 77.7 | 75.3 to 80.1 | 61.5 | | | | 117 | 255 |
| Design, p1 with the whole sentence | 72.3 / 86.0 / 78.6 | 76.3 to 80.9 | 56.4 | 1,295 | 936 | 359 | 245 | 396 |
| q3 with six pieces | 68.8 / 86.6 / 76.7 | 74.3 to 79.1 | 51.1 | 1,369 | 942 | 427 | 301 | 299 |

Right is a returned name with the gold span and kind. Right over 1,088 gives recall, and right over names returned gives precision. An extra is a returned name that overlaps no gold name, as local experiment 281's scorer counts it. The other wrong names overlap a gold name with the wrong edges or the wrong kind.

Paired bootstrap: design minus baseline +0.8 (-1.2 to +3.1). q3 at six pieces minus the design -1.9 (-3.3 to -0.5). That comparison changes the wording and the snippet at once. Local experiment 288 shows the snippet caused the loss, not the wording.

The design's 245 extras on UNER include 55 pieces of web or email addresses and 16 lone marks. Published: XLM-R Large fine-tuned on 12,543 labelled English EWT sentences scores 85.8 (Mayhew et al., NAACL 2024).

WNUT-17, p1 on the whole sentence, one run, six kinds: 45.9 precision, 63.9 recall, 53.4 F1, 95% range 50.3 to 56.4. About 190 false names are handles tagged as person and about 250 are pieces of web addresses. Published: the best 2017 shared-task system 41.86 (Derczynski et al., W-NUT 2017); zero-shot gpt-3.5-turbo 46.61 on 300-sentence samples (Xie et al., EMNLP 2023); fine-tuned BERTweet, RoBERTa large and XLM-R large 56.5 to 57.1 (Nguyen et al., EMNLP 2020).

## 5. Local experiment 282: error groups

Upper bounds at five kinds if every error in a group were fixed: touching names +2.1 F1, averaged step-1 views +0.3, an inner-name question after a decline +1.6. Nationality and language words at no kinds: at most +1.6.

## 6. Local experiment 283: a span check

On the key's held-out half, pooled over five kind sets, per run:

| Arm | Run 1 | Run 2 | Run 3 |
| --- | --- | --- | --- |
| Recorded control | 87.9 | 88.4 | 88.2 |
| Paired control, the edge question | 87.4 | 88.2 | 88.0 |
| Span check with the convention sentence | 88.3 | 89.1 | 88.9 |

The span check gained 0.9 against the paired control in every run. Run 1 sat inside the recorded control's range, and the held-out gain rested on two names. It cost 3 to 4% more tokens.

## 7. Local experiment 284: step 2 on the key's own names

Every arm labels the key's names plus the design's recorded extra names, so step 1 cannot move the result. Mean (min to max):

| Kinds | Arm | F1 H | Extras a run |
| --- | --- | --- | --- |
| 3 | none2 | 92.3 (92.1-92.4) | 37.3 |
| 3 | Broader wording | 91.7 (91.7-91.7) | 41.0 |
| 3 | Descriptions | 92.3 (92.1-92.5) | 40.7 |
| 3 | Two way-outs | 90.3 (90.1-90.6) | 30.0 |
| 5 | none2 | 92.2 (91.7-92.5) | 31.7 |
| 5 | Broader wording | 91.8 (91.5-92.1) | 41.3 |
| 5 | Descriptions | 94.0 (93.8-94.4) | 32.7 |
| 5 | Two way-outs | 88.1 (87.7-88.6) | 26.7 |
| 10 | none2 | 92.5 (91.8-93.0) | 32.0 |
| 10 | Broader wording | 93.4 (93.0-93.6) | 32.0 |
| 10 | Descriptions | 94.0 (93.6-94.2) | 30.0 |
| 10 | Two way-outs | 88.5 (88.0-88.7) | 29.0 |

The keep rule asks every held-out run to beat the control's best held-out run, with and without two sentences on drinks and planets, no loss on the 100 UNER sentences, and at most one more extra a run. Descriptions passed it at five kinds only. At five kinds they fixed 13 names and broke 2. At three kinds they gave no held-out gain and 3.3 more extras. At ten kinds the gain fell inside the noise once the two sentences were left out. The key's descriptions are its own label lines, so part of the lift restates the key. On the full UNER split, over name lists frozen from local experiment 281's whole-sentence p1 run, descriptions scored 82.1 against 82.3. The broader wording fell on the held-out half at three and five kinds and added 3.7 and 9.7 extras a run there. At ten kinds it gained, 93.4 against 92.5, with no added extras, and the gain sat in the drinks sentence. Two way-outs fell 2.0 held-out points at three kinds and 4.0 to 4.1 at five and ten, and broke 19 to 34 real names. Their gain on the 100 UNER sentences came from dropping extras.

On 40 new metonymy and venue sentences at five kinds, none2 scored 60.2 and descriptions 70.5. Capitals and teams came back as places in every arm.

Step 3 on 60 new relation sentences with 70 stated edges, from the key's own names: 66, 68 and 67 found over three runs, one inferred edge passed a run, and no negated or hypothetical edge passed.

## 8. Local experiment 285: windows on public documents

Six whole UNER test documents of 371 to 678 words, 3,128 words and 297 names in all.

| Arm | Runs | Labelled F1 | Tokens a word |
| --- | --- | --- | --- |
| Whole text | 1 | 81.6 | 1,563 |
| p1, six pieces | 3 | 83.1 (83.0-83.2) | 395 |
| p1 on one sentence at a time, local experiment 281's recording on gold sentence breaks | 1 | 85.6 | 409 |
| p1, twelve pieces | 3 | 83.4 (83.2-83.7) | 420 |
| q3, six pieces | 3 | 83.9 (83.5-84.3) | 322 |

Twelve pieces against six gained 0.3 with overlapping ranges and cost 6% more. q3 against p1 at six pieces gained 0.8 with separate ranges. One sentence at a time sat 2.5 points above p1 at six pieces, on one run. Errors within six pieces of a request edge were 29 to 30% of all errors for p1 at six pieces, and 30% of pieces sit there.

## 9. Local experiment 286: web kinds

Decoys `web address`, `email address`, `social handle` and `punctuation or symbol`. One run each. The control is local experiment 281's p1 run on the whole sentence, with no cut.

| Set | Arm | F1 | Against control | Paired 95% range | Real names lost |
| --- | --- | --- | --- | --- | --- |
| UNER | Control | 78.6 | | | |
| UNER | Decoys in step 2 | 79.7 | +1.1 | +0.5 to +1.7 | 4 |
| UNER | Decoys in both steps | 80.8 | +2.2 | +1.4 to +3.2 | 8 |
| WNUT-17 | Control | 53.4 | | | |
| WNUT-17 | Decoys in step 2 | 55.2 | +1.8 | +0.1 to +3.5 | 127 |
| WNUT-17 | Decoys in both steps | 53.8 | +0.4 | -1.0 to +1.7 | 60 |

Of the 127 WNUT-17 names lost in step 2, 112 are handles the gold tags as persons.

The web kinds asked for directly, from a hand check of samples:

| Kind | Checked | Whole | Right kind |
| --- | --- | --- | --- |
| Web address | 50 | 52% | 98% |
| Email address | 36 | 83% | 100% |
| Social handle | 50 | 66% | 92% |

Most other returns are pieces such as `t.co`, `https` or a lone `@`.

## 10. Local experiment 287: the name score

Offline over the saved replies of local experiments 279, 281 and 285. No model call. P(kind) is step 2's probability of the chosen kind. The lowest-tag form is today's `strength`: the lowest step-1 tag probability in the name times P(kind). P(span) is the forward-backward probability of exactly that stretch. Each tag probability is first floored at one in a million, with no renormalization. A valid path's weight is the product of its floored tag probabilities. P(span) is the summed weight of the valid paths that tag exactly that stretch as one name, divided by the summed weight of all valid paths. AP is average precision. ECE is expected calibration error over ten bins.

| Set | P(kind): AP, ECE | Lowest tag × P(kind): AP, ECE | P(span) × P(kind): AP, ECE |
| --- | --- | --- | --- |
| Key, 5 kinds, p1 on the whole sentence, 3 runs | 80.9, 8.1 | 83.2, 15.2 | 82.3, 7.4 |
| UNER, p1 on the whole sentence, 1 run | 77.9, 20.4 | 80.1, 3.1 | 80.6, 12.1 |
| WNUT-17, p1 on the whole sentence, 1 run | 49.1, 33.2 | 50.5, 11.2 | 52.5, 19.5 |
| Long documents, p1 at six pieces, 3 runs | 78.7, 17.1 | 80.8, 8.6 | 81.9, 6.7 |

F1 at a cut of 0.5, against no cut:

| Set | No cut | P(kind) | Lowest tag × P(kind) | P(span) × P(kind) |
| --- | --- | --- | --- | --- |
| Key, 5 kinds, p1 on the whole sentence, 3 runs | 87.3 | 87.7 | 84.0 | 87.2 |
| UNER, p1 on the whole sentence, 1 run | 78.6 | 78.9 | 80.3 | 81.0 |
| WNUT-17, p1 on the whole sentence, 1 run | 53.4 | 56.0 | 59.4 | 60.1 |
| Long documents, p1 at six pieces, 3 runs | 83.1 | 82.9 | 79.8 | 84.3 |

On UNER, P(span) × P(kind) at 0.5 against no cut: paired bootstrap +1.6 to +3.4. The cut removes 127 names, 105 wrong and 22 right. Names scored 0.5 to 0.6 were right 26% to 76% of the time, by set. The edge question's pick probability added nothing. With no kinds on the key, P(span) at 0.5 scored 89.9 against 89.7 with no cut.

## 11. Local experiment 288: p1 against q3 at six pieces on the full public sets

Both arms used the shipped setup except for the step-1 wording. Step 1 showed six pieces each side, in requests of at most 40 pieces, then a Viterbi decode with a floor of one in a million. Step 2 used none2 and showed six pieces each side of the name, one request a name. The `strength` cut was P(kind) × P(span) at 0.5. The edge question went to each name that survived the cut and had more than one option. Each arm ran once on each set. Scoring follows local experiment 281. On WNUT-17, organization precision is `corporation` precision.

| Set | Arm | F1 | 95% range | Precision | Recall | Organization precision | Extras | Tokens a word |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| UNER | p1 | 79.9 | 77.6 to 82.2 | 77.2 | 82.8 | 60.8 | 170 | 383 |
| UNER | q3 | 80.1 | 77.8 to 82.3 | 76.3 | 84.3 | 58.4 | 193 | 308 |
| WNUT-17 | p1 | 59.6 | 56.6 to 62.2 | 60.2 | 58.9 | 52.3 | 297 | 385 |
| WNUT-17 | q3 | 59.1 | 56.2 to 61.9 | 57.0 | 61.4 | 44.7 | 351 | 308 |

Paired bootstrap of F1, p1 minus q3, on the same resampled sentences: UNER -0.2 (-1.3 to +1.0), with 38% of draws favouring p1. WNUT-17 +0.4 (-0.8 to +1.7), with 76% of draws favouring p1.

F1 ties on both sets. q3 uses 20% fewer input tokens. p1 returns 23 fewer extras on UNER and 54 fewer on WNUT-17, with organization precision higher by 2.4 and 7.6 points. q3 finds 16 more right names on UNER and 27 more on WNUT-17.

Before the cut and without the edge question, F1 was 76.2 for p1 and 75.1 for q3 on UNER, and 52.3 and 52.8 on WNUT-17. The edge question added up to 0.6 points on UNER and nothing on WNUT-17.

Against local experiment 287's whole-sentence p1 figures at the same cut, 81.0 on UNER and 60.1 on WNUT-17, the six-piece p1 run sits about 1 point lower on UNER and 0.5 lower on WNUT-17. That comparison crosses runs. Local experiment 281's loss of 1.9 for q3 at six pieces against p1 on the whole sentence came from the snippet, not the wording.
