# Evidence for the batching and recognize designs

Filed 2026-09-26. This record copies the measurements that `sdlc/issues/2026-09-26-batching-design.md` and `sdlc/issues/2026-09-26-recognize-design.md` cite. Each table names the workspace experiment that measured it. The experiments stay unpushed in the workspace, so this record is the repository's copy. One model answered every paid call: `jev-1.13.0` behind `jev-latest`. Cost uses the recorded input price of $0.042 a million input tokens. Output tokens are free under the vendor's price list.

Sections 8, 9 and 13 hold arithmetic done for the designs over saved replies, dry runs and saved inputs. No new call was made for them.

## 1. Experiment 208: rows packed into one request, 2026-09-20

1,000 SMS messages, question `The message is spam.`, one `noul` question per row. The single-row baseline is experiment 206.

| Arm | Rows a request | Requests | Input tokens | Wall seconds | Accuracy | Recall |
| --- | --- | --- | --- | --- | --- | --- |
| Baseline, 206 | 1 | 979 | 289,260 | none recorded | 0.968 | 0.941 |
| A05 | 5 | 200 | 113,996 | 10.89 | 0.949 | 0.978 |
| A10 | 10 | 100 | 86,396 | 5.77 | 0.968 | 0.978 |
| A20 | 20 | 50 | 73,096 | 3.21 | 0.941 | 0.860 |
| A40 | 40 | 25 | 66,446 | 1.63 | 0.882 | 0.434 |

At 40 rows the first fifth of each request scored 0.985 and the fourth fifth 0.795. At 10 rows every fifth scored 0.960 to 0.975.

Regrouping the same 1,000 rows into other tens (arm S10 against A10):

| Figure | Value |
| --- | --- |
| Answers that crossed 0.5 | 34 of 1,000 |
| Mean move in probability | 0.047 |
| Largest move | 0.66 |
| Moved more than 0.1 | 119 of 1,000 |
| S10 accuracy | 0.964, against 0.968 |

500 BoolQ records at 10 a request scored 0.890 against 0.886 single-row, for 2.0 times fewer tokens.

## 2. Experiment 260: packing shapes, 2026-09-25

Noise floor. The same 1,000 SMS requests sent twice crossed 0.5 for 5 rows, with a mean move of 0.007 and no move over 0.1.

Row shapes at 10 a request, 228 Beatles yes/no questions:

| Arm | Shape | Right of 228 | Tokens a question |
| --- | --- | --- | --- |
| Single-row | one title a request | 155 | 294.8 |
| YA | `rows: [{"text": ...}]` | 158 | 72.7 |
| YB | `rows: [{"title": ...}]` | 160 | 73.0 |
| YC | one TSV `table` string | 154 | 64.7 |

No shape differs from single-row or another shape beyond chance (exact paired tests, p 0.49 to 1.0).

Pick-one at widths 1, 5 and 10, 200 `choose` questions: single-row 141 right. Every packed arm scored 120 to 137, and the loss began at width 1 under the `rows` wrapper.

Shared context, 196 open-book questions: one request a question scored 187 right for 2,394,007 input tokens. The catalog sent once for 10 questions scored 188 for 266,183.

Size, 100 BoolQ rows at 10 a request: accuracy fell from 0.92 near 2,100 tokens a request to 0.79 near 26,900.

## 3. Experiment 261: pick-one wording, 77 questions

| Arm | Width | Per-row instruction | Right of 77 |
| --- | --- | --- | --- |
| Single-row | 1 | the question alone | 56 |
| W0-10 | 10 | ``For the record `rows[3]`: <question>`` | 55 |
| W2-10 | 10 | `<question>` alone, no quote | 30 |
| W3-01 | 1 | `The text is "<input>", from rows[3].text. <question>` | 55 |
| W3-10 | 10 | the same | 58 |

## 4. Experiment 262: pick-one at width 10, 200 questions

The preregistered bar is the single-row score less 3 points: 0.705 less 0.03, or 135 of 200.

| Arm | Right of 200 |
| --- | --- |
| S0, single-row, recorded | 141 |
| S1, the same bodies sent again | 139 |
| W3-01 | 132 |
| W3-10, one grouping | 146 |
| W3-10b, another grouping | 140 |

Both width-10 groupings pass the bar.

## 5. The 2026-09-22 wire probe

`sdlc/issues/closed/2026-09-22-wire-probe-can-one-request-carry-many-states.md`, shape 4. A request whose `state` was a plain JSON list of two strings answered 200 with one answer at 0.50 and billed both strings. A plain list as evidence therefore gets one silent answer.

## 6. Experiment 268: filter timing, 306 short titles

Step 5 sent one request at a time, `--jobs 1`, with each title carrying 0 to 70 catalog lines as context. Each size ran 40 records a run, twice, and size 0 ran four times. Each run's first request opened a new connection and is left out. That leaves 78 requests a size and 156 at size 0. Server time is the service's own time, read from the `x-envoy-upstream-service-time` reply header.

| Catalog lines in the record | Median record bytes | Median input tokens | Context tokens over size 0 | Requests | Server time, median | Round trip, median | Round trip, 90th percentile |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | 18 | 293 | 0 | 156 | 58 ms | 143 ms | 179 ms |
| 2 | 214 | 370 | 77 | 78 | 57 ms | 135 ms | 182 ms |
| 7 | 678 | 554 | 261 | 78 | 61 ms | 151 ms | 187 ms |
| 23 | 2,150 | 1,145 | 852 | 78 | 61.5 ms | 135 ms | 163 ms |
| 27 | 2,518 | 1,290 | 997 | 78 | 59 ms | 140 ms | 170 ms |
| 70 | 6,476 | 2,878 | 2,585 | 78 | 69 ms | 149 ms | 191 ms |

Server time stayed at 57 to 61.5 ms up to about 1,000 context tokens and reached 69 ms at 2,585. A least-squares fit gives 0.4 ms a thousand input tokens for the service and 0.7 ms for the whole request.

One larger data point exists. Experiment 271 sent the whole 306-song catalog as context in one request, 22,091 input tokens, and it answered in 0.36 s in each of three runs, by section 7. It is one point, not a slope, and it times a whole request of 306 questions.

The limit: nothing above 2,585 context tokens was measured one request at a time. A catalog of tens of thousands of tokens is unmeasured. A filled request at the 96,000-byte ceiling can carry that much. Ticket S1 in the batching design measures context time at the sizes a filled request really carries, up to the ceiling. No page or slide claims a context-time figure until S1 has measured it.

The record text added (2,871.75 − 293.2) ÷ (6,476 − 18) = 0.40 input tokens a byte. That text is catalog lines with names and dates. It is the only measured rate for plain record text, and the designs use it for evidence.

## 7. Experiment 271: 306 songs in one request, 2026-09-26

Key: 18 of 306 Beatles titles are on Abbey Road. Cut 0.7. Four requests in flight. The baseline ran through `thinkthen filter`. The other arms ran from a Python client with the quoted question `The text lists the title "<song>", a song by the Beatles. It appears on the album Abbey Road.`, titles one a line as evidence. Chunks were fixed tens in alphabetical order.

| Arm | Requests | Input tokens a run | Time a run (s) | Right | False yeses | Missed |
| --- | --- | --- | --- | --- | --- | --- |
| Baseline, today's filter | 306 | 88,933 | 13.0 / 13.9 / 14.2 | 285 / 285 / 285 | 17 / 17 / 16 | 4 / 4 / 5 |
| A1, quoted question, one title a request | 306 | 90,629 | 13.5 / 11.8 / 10.7 | 290 / 292 / 290 | 12 / 10 / 12 | 4 / 4 / 4 |
| A10, chunks of 10 | 31 | 19,656 | 1.40 / 1.32 / 1.73 | 283 / 283 / 287 | 22 / 22 / 18 | 1 / 1 / 1 |
| A50 | 7 | 13,462 | 0.41 / 0.44 / 0.52 | 282 / 280 / 281 | 23 / 25 / 24 | 1 / 1 / 1 |
| A100 | 4 | 12,687 | 0.38 / 0.67 / 0.30 | 284 / 283 / 281 | 22 / 23 / 25 | 0 / 0 / 0 |
| A306, one request | 1 | 11,913 | 0.31 / 0.30 / 0.39 | 278 / 276 / 278 | 27 / 29 / 27 | 1 / 1 / 1 |
| B306, catalog once, one request | 1 | 22,091 | 0.36 / 0.36 / 0.36 | 301 / 299 / 302 | 3 / 3 / 2 | 2 / 4 / 2 |
| Catalog entry with each song, one a request, 5 takes | 306 | about 97,000, estimated | 8.9 to 22.6 at 16 in flight | 301 to 303 | 1 to 2 | 2 to 4 |

Runs of one arm disagreed on 6 to 8 songs. The request floor bills about 250 input tokens, and each quoted question about 33.

The service's question limit:

| Questions | Input tokens | Result |
| --- | --- | --- |
| 612 | 21,871 | 200 in 0.52 s |
| 1,224 | 41,787 | 200 in 0.63 s |
| 1,800 | 60,499 | 200 in 0.86 s |
| 2,448 | about 80,000, estimated | 400 `max_tokens_exceeded` |
| 7,000 short questions | 64,955 | 200 in 2.15 s |
| 8,000 short questions | about 74,000, estimated | 400 `max_tokens_exceeded` |

No count limit appeared. The only limit found is about 65,536 input tokens a request. The catalog of B306 carried about 11,500 tokens, under the 32,000-token evidence limit.

## 8. Calibration safety over experiment 271's saved replies

Computed for the batching design from 271's saved replies at the cut of 0.7. Calibration error follows `specification/audit.md`: ten equal bins over the confidence in the answer given, the sum of `|Σ right − Σ confidence|` over bins, divided by the answers. It was computed in Python, not by `thinkthen audit`.

| Arm | Calibration error, three runs |
| --- | --- |
| A1, one a request | 0.185 / 0.194 / 0.185 |
| A10 | 0.115 / 0.115 / 0.134 |
| A50 | 0.160 / 0.136 / 0.142 |
| A100 | 0.104 / 0.096 / 0.087 |
| A306 | 0.065 / 0.065 / 0.080 |
| B306 | 0.069 / 0.062 / 0.076 |

Answers that crossed 0.7, and the mean absolute change in probability, over the 306 songs:

| Pair | Crossed | Mean change |
| --- | --- | --- |
| A1 repeats, same bytes | 2 to 4 | 0.018 to 0.020 |
| A10 repeats, same bytes | 4 to 6 | 0.020 |
| A1 against A10, run by run | 17 to 23 | 0.113 to 0.119 |

271 ran one grouping, so it has no shuffled-order pair. 208's regrouped tens crossed 3.4% of answers with a mean move of 0.047.

## 9. Content cuts over the 306 titles

The batching design fills each batch to the request limit. It closes a batch early after a record whose content hash is 0 mod 4,096. The content hash is the SHA-256 of the record in compact JSON, first 8 bytes, big-endian. Simulated over experiment 268's `songs.txt` in its alphabetical order, with bodies built in the design's section 1 shape and `jev-latest` as the model:

| Form | Body bytes | Titles on a content cut | Batches |
| --- | --- | --- | --- |
| Titles as evidence | 55,490 | none | 1 |
| Catalog as context, from experiment 271's B306 request | 62,748 | none | 1 |

Both bodies sit under the 96,000-byte ceiling. The plain body holds 55,490 ÷ 306 = 181 bytes a title, so a request at the ceiling holds about 96,000 ÷ 181 = 529 short titles. At mod 1,024 the 72nd title would fall on a cut, and the list would form 2 batches. Experiment 271's own bodies held 52,417 bytes for A306 and 78,358 for B306. B306's catalog, its evidence, held 28,462 bytes.

At `--batch 10` no title falls on a cut, so the list forms fixed runs of 10: 31 batches, the grouping experiment 271 measured.

The earlier rule, a cut at 0 mod 10 and a cap of 20, formed 34 batches of 1 to 20 records. A title inserted after the 100th line changed 1 batch of the 34.

## 10. Experiment 265: plain sentences, recognize and relations

30 sentences, with `sang=person:song` and `wrote=person:song`.

Under today's relation wording, stated edges with correct names scored 0.89 to 1.0. The five unstated edges, true in the world, scored 0.78, 0.74, 0.55, 0.60 and 0.68 in run one. In runs two and three, wrote(Ringo Starr, Octopus's Garden) scored exactly 0.80.

| Relation cut | Recall | Precision |
| --- | --- | --- |
| 0.5 | 77.8% | 56.8% |
| 0.7 | 77.8% | 63.6% |
| 0.8 | 77.8% | 72.4% |
| 0.9 | 74.1% | 71.4% |

A cut of 0.8 removed every unstated edge in run one and let one through in runs two and three.

Asked directly, one request a sentence, "Does the text itself state that SOURCE RELATION TARGET?":

| Kind of edge | Probes | Yes probability |
| --- | --- | --- |
| Stated, true in the world | 6 | 0.84 to 0.99 |
| Stated, false in the world (c25 to c27) | 3 | 0.97 to 0.98 |
| Unstated | 6 | 0.02 to 0.05 |

## 11. Experiment 267: word rules over a 100-sentence key

100 hand-written sentences, 168 names, offsets in Unicode code points.

| Splitter | Key names reachable | Words asked |
| --- | --- | --- |
| Today's | 141 of 168 | 772 |
| Proposed default rules | 166 of 168 | 818, 6% more |
| Proposed rules with `-` as an infix | 168 of 168 | 8% more than today |

The two the default rules miss are `London` in `London-based` and `Microsoft` in `anti-Microsoft`.

## 12. Experiment 270: recognize design measured on the 267 key

Five bare kinds: `person place organisation work thing`. Three runs each.

| Variant | Recall | Precision | Touching pairs split | Requests a case |
| --- | --- | --- | --- | --- |
| Today | 70.8% (357/504) | 76.3% (357/468) | 0 of 18 | 1.00 |
| Word rules | 82.7% (417/504) | 86.3% (417/483) | 0 of 18 | 1.00 |
| Word rules and `confirm` | 87.5% (441/504) | 88.6% (441/498) | 12 of 18 | 1.09 |
| Word rules and a begin question | 88.5% (446/504) | 86.8% (446/514) | 12 of 18 | 1.00 |

A cut tuned on one half moved test-half recall and precision by at most 2.5 points, in both directions.

## 13. recognize cost per word, from experiment 270

Today's 100 cases bill 266,502 input tokens a run over 772 words: 2,665 tokens a case and 345.2 a word. The 345 already holds each case's fixed part and its sentence as evidence, spread over 7.72 words.

A least-squares fit over the 100 cases, input tokens against words, gives about 315 tokens a request and 304 a word. Adding the sentence's bytes as a second term gives 305 a request and 298 a word. The designs use 300 a request for the fixed part and 300 a word for its two questions, and add the evidence at 0.40 tokens a byte, by section 6.

The 100 dry-run bodies hold 889,901 bytes for 772 words: 1,153 bytes a word, with each case's sentence inside. A fit gives 74 bytes a request and about 1,150 a word once the word's own bytes in its snippets are counted.

Each kind question listed the five kinds in 82 bytes. Three kinds shorten each word's kind question by about 30 bytes. The token effect is unmeasured, so three-kind figures below run slightly high.

**The window.** No experiment measured a bounded window of evidence. Experiment 270's key sentences run 1 to 14 words and 4 to 74 bytes, 37.8 bytes on average. Every measured case therefore falls wholly inside a window of 14 or more words on each side, and its answers carry over unchanged. The recognize design sets a stated default of 200 words on each side of a piece, configurable. At 6 bytes a word that window adds at most 2,400 bytes, about 960 tokens, to a request of about 25,000.

**Words a request with the window.** A piece of P words carries at most (P + 400) × 6 bytes of evidence. A request holds P words when 1,150 × P + (P + 400) × 6 + 74 ≤ 96,000. P = 80 gives 92,000 + 2,880 + 74 = 94,954 bytes. P = 81 gives 96,110, over the ceiling. So a request holds 80 words, and its evidence is at most 2,880 bytes, or 1,152 tokens at 0.40 a byte. A text shorter than the window is its own evidence.

The designs plan a text at about 6 bytes a word and 6% more words under the new rules. Each request then costs at most 300 + 1,152 = 1,452 tokens beside its words. Word rows:

| Text | Words after the rules | Text bytes | Words a request | Word requests | Input tokens, at most | Cost |
| --- | --- | --- | --- | --- | --- | --- |
| 60 words | 64 | about 360 | 64, the whole text | 1 | 300 + 144 + 64 × 300 = 19,644 | $0.0008 |
| 1,000 words | 1,060 | about 6,000 | 80 | 14 | 14 × 1,452 + 1,060 × 300 = 338,328 | $0.014 |
| 10,000 words | 10,600 | about 60,000 | 80 | 133 | 133 × 1,452 + 10,600 × 300 = 3,373,116 | $0.14 |
| The hard cap, 600,000 bytes | 106,000 | 600,000 | 80 | 1,325 | 1,325 × 1,452 + 106,000 × 300 = 33,723,900 | $1.42 |

The 60-word text fits one request: 64 × 1,150 + 360 + 74 = 74,034 bytes. 1,060 ÷ 80 = 13.25 gives 14 requests. 10,600 ÷ 80 = 132.5 gives 133. The first and last pieces carry a window on one side only, so the rows are upper bounds. `confirm` requests add a few questions and one window each, and the rows leave them out. Ten times the text now costs about 10 times the tokens.

Under the earlier whole-text rule every request carried the whole text. The same working gave 77 words a request, 14 requests and 355,800 tokens for 1,000 words, and 31 words a request, 342 requests and 11,490,600 tokens, $0.48, for 10,000 words.

Section 7's requests of 21,871 and 41,787 input tokens answered in 0.52 and 0.63 s. A full word request carries about 25,000. At 4 requests in flight, 14 word requests take 4 rounds, 133 take 34, and 1,325 take 332.
