# Packing and batching: what they buy, what they cost, and the setting

Status: Open. The dedicated issue for packing and batching, on Ian's request of 2026-09-25. It replaces `2026-09-25-pack-rows-for-tables-and-frames-a-plan-to-measure-then-build.md`, renamed here.

## In one paragraph

Today every row is its own request to Jev. Each request pays a fixed part near 256 input tokens, so short rows cost mostly overhead, and the vendor's rate limit caps a table near 20 rows a second. Packing puts up to 10 rows, each with its own question, into one request. It cuts yes/no cost about four times and keeps accuracy. It has three costs. A row's answer can change with the rows packed beside it. Pick-one questions lose accuracy when packed today. Long packs read worse. Sending a shared context once per pack, such as a reference document, is the largest saving measured: nine times fewer tokens with no loss.

## The trade-offs

| Choice | What it buys | What it costs | Evidence |
| --- | --- | --- | --- |
| One row per request (today) | Each answer depends only on its row. Replay and the cache key are simple. | The fixed part on every row. About 20 rows a second at most. | `2026-09-21-one-state-per-request-caps-table-scale-classification.md` |
| Pack yes/no rows, up to 10 | 3.3 to 4.6 times fewer tokens. About ten times fewer requests. | About 3% of answers near the bar change with their neighbours, six times Jev's own noise. | Experiments 208 and 260 |
| Pack more than 10 | A little more saving | Accuracy slips at 20 and collapses at 40. Late rows read worse. | Experiment 208 |
| Pack pick-one rows | 2.3 times fewer tokens | 2 to 10 points of accuracy with today's wording | Experiment 260; rewording under test in 261 |
| Shared context once per pack | Nine times fewer tokens on the Beatles open-book run, $0.51 to about $0.06 per 1,000 questions | None measured: 188 of 196 against 187 | Experiment 260 |
| Large packs | Fewer requests | Accuracy falls from 0.92 near 2,000 tokens a request to 0.79 near 27,000, well under the 32,000-token limit | Experiment 260 |

## The setting

Packing is a configuration setting that every surface reads the same way, not a fixed choice in the code. Ian ruled this on 2026-09-25.

- **What it sets.** `pack`: how many rows share one request, from 1 (no packing) to 10. `pack_tokens`: the size at which a pack closes early. `context`: a shared text sent once per pack.
- **Where it is set.** Everywhere the other engine settings are set: the command option and configuration file, the `THINKTHEN_` environment, the library engine builder, SQL `SET` and settings, and the question file, so a question that must stay unpacked can say so.
- **Which answer types it covers.** Yes/no first. Pick-one, score, and tag only after a packed width-1 request matches single-row accuracy for that type.
- **The default** is for the ADR, from the evidence here.

## What the documentation carries

- A site page, "Packing and batching", in the how-to form of ADR 0011, with the trade-off table above in plain words, the setting and where to set it, and one worked example at `pack` 1 and 10 with its token counts.
- The size-and-cost page named in `2026-09-21-size-cost-and-other-backends-what-the-manual-and-the-tests-must-carry.md` links to it.
- The talk carries one slide on it, from the marketing repository.

## What is measured

- One request per record is the rule on every surface (`specification/annotate.md`, `sdlc/planning/databases/README.md`). Each request pays a fixed part near 256 input tokens. At the documented 1,200 requests a minute that caps a table near 20 rows a second and near $1.08 per 100,000 short rows (`2026-09-21-one-state-per-request-caps-table-scale-classification.md`).
- The endpoint accepts many rows in one request as a structured `state`: one `condition`, a `rows` list, and one `noul` question per row named `r0` to `rN` (`2026-09-22-wire-probe-can-one-request-carry-many-states.md`). A plain list `state` returns one silent answer and must never be sent.
- Experiment 208 (`~/workspace/experiments/208-thinkthen-row-packing/RESULTS.md`), yes/no questions only:
  - 1,000 SMS rows at 10 per request: accuracy 0.968, equal to one per request, for 3.3 times fewer tokens and ten times fewer requests. The run took 5.8 seconds.
  - 500 BoolQ rows at 10 per request: 2.0 times fewer tokens, accuracy within half a point. BoolQ rows are five times longer than SMS rows and did not break sooner. Row count breaks packing, not row length.
  - At 20 rows accuracy slipped. At 40 recall fell from 0.941 to 0.434, and rows late in the list read worse.
  - Regrouping the same rows at 10 per request moved 3.4% of answers across 0.5. A row's neighbours change its answer. The run did not measure plain run-to-run noise, and a separate measure puts it near 26% of repeats moving (`2026-09-21-the-same-request-answers-differently-twice-measured.md`).
- The limits: about 32,000 tokens of text and about 64,000 for the whole request (`2026-09-21-size-cost-and-other-backends-what-the-manual-and-the-tests-must-carry.md`). Ticket 0079 splits requests under backend limits.
- Every reply reports its billed `input_tokens`. No public tokenizer exists, and ticket 0080 bans guessing tokens from bytes in printed output.

## Proposed design

1. **A pack width, capped at 10.** An engine setting `pack` from 1 to 10 for `decide`, `filter`, and the SQL and data-frame scalars that call them. The surfaces that send many rows at once use it: DuckDB chunks, `thinkthen_warm`, PostgreSQL arrays, and data-frame columns. Whether the default stays at 1 is for the ADR, after the experiments.
2. **A size budget beside the count.** A pack closes at 10 rows or at a byte budget, whichever comes first. The budget starts well under the 32,000-token text limit. The engine learns a bytes-per-token ratio for each question from the `input_tokens` its own replies report, so no tokenizer is needed. A size refusal splits the pack in half through ticket 0079's path.
3. **Fixed groups.** Distinct texts sort by a hash of the text before grouping, so the same table packs the same way each run. The same rows then get the same neighbours, and a rerun matches its recording.
4. **A per-row cache entry.** Each row's answer is stored under its own key, which carries the question, the text, and the pack width. A later query reuses row answers and packs only the misses. A packed answer never serves a query that asked for width 1, and the reverse holds too.
5. **Shared context pays once.** A long shared context, such as the song catalog in Beatles Bench open-book runs (about 12,000 tokens a call), is sent once per pack. At 10 rows per pack that cuts an open-book pass near tenfold, from about $0.51 per 1,000 questions.

## Experiments before the ADR

Each costs cents and needs Ian's authorization through `sdlc/scripts/live`.

1. **Width 1, repeated.** Send the same 1,000 SMS rows singly a second time. This separates the model's own noise from the neighbour effect that 208 could not split.
2. **Other answer types.** Pack `choose`, `score`, and `tag` at 1, 5, and 10 per request. 208 packed only yes/no.
3. **Beatles Bench.** Run the Beatles-only questions packed at 10 per request against the single-row run, closed book and open book. This gives the talk's own numbers and tests the shared-context saving.
4. **The size edge.** Pack long rows at 10 per request up to the text limit to find where size matters.

## What Ian can overturn

The cap of 10, the default width, and whether packing is worth an answer that depends on its neighbours.

## Results, experiment 260

Ran on 2026-09-25 under Ian's authorization, through `sdlc/scripts/live`. Model `jev-1.13.0`. Declared 2,870,000 tokens and billed 1,785,631 input tokens, 7.5 US cents. Every request answered 200. No answer came back missing, misnamed, or the wrong count. The full record is `~/workspace/experiments/260-row-packing-shapes/RESULTS.md`, with the preregistration, bodies, and replies beside it.

- **Noise floor (experiment 1).** The same 1,000 SMS rows sent singly a second time crossed the 0.5 cut for 5 rows and moved none by more than 0.1. Regrouping at 10 per request crossed it for 34 (experiment 208). Jev's own noise is small. The neighbour effect is real, about six times the noise.
- **Row shapes (experiment 3, closed book).** All 228 Beatles-only yes/no questions at 10 per request. Rows as `{"text"}` scored 0.693, named columns 0.702, one TSV table string 0.675, single-row 0.680. No gap is beyond chance. Packing cut tokens 4.1 times, and the table cut them 4.6 times.
- **Pick-one answers (experiment 2).** 200 Beatles `choose` questions, each row with its own `choice` question and options. Jev answered each by name. Every packed arm scored below single-row (0.705), from 0.685 down to 0.600. The loss starts at one row a request, so the per-row wrapper wording causes it. Width 10 cut tokens 2.3 times. `score` and `tag` were not tested.
- **Shared context (experiment 3, open book).** The song catalog sent once per 10 questions scored 188 of 196 against 187 for one question per request. Input tokens fell from 2,394,007 to 266,183, nine times fewer, about 0.057 dollars per 1,000 questions.
- **Size edge (experiment 4).** 100 BoolQ rows at 10 per request, passages padded with other passages. Accuracy fell from 0.92 at about 2,000 tokens a request to 0.86 at 10,000, 0.83 at 20,000, and 0.79 at 27,000. The same long rows sent singly scored 0.86. No request was refused.

What this means for the design:

1. Keep the cap of 10 for yes/no. The row shape does not matter for accuracy, so the engine can pick the cheapest encoding.
2. Do not pack `choose` until a width-1 packed request matches single-row accuracy. The per-row instruction needs rewording and remeasuring first.
3. A shared context sent once per pack buys the largest saving measured, nine times on open book.
4. The byte budget matters. Accuracy slid well under the 32,000-token text limit. A first budget near 10,000 tokens a request fits this evidence.

Ian can overturn the reading of each point. Each rests on one day, one grouping per arm, and samples of 100 to 1,000.

## Results, experiment 261

Ran on 2026-09-25 under Ian's authorization, through `sdlc/scripts/live`. Model `jev-1.13.0`. Declared 260,000 tokens and billed 162,822 input tokens, 0.7 US cents. Every request answered 200 with every answer named and typed. The full record is `~/workspace/experiments/261-packed-choice-wording/RESULTS.md`.

The question: does rewording the per-row instruction remove the pick-one loss 260 found? Items: the two biggest Beatles `choose` templates in 260's sample, first album (56) and release month (21), 77 in all. Single-row got 56 of 77. Four wordings ran at one and ten rows a request:

| Wording | Per-row instruction | Width 1 | Width 10 |
| --- | --- | --- | --- |
| W0, 260's | ``For the record `rows[i]`: <question>`` | 50 | 55 |
| W1 | `For the text in rows[i].text: <question>` | 50 | 55 |
| W2 | `<question>` alone | 54 | 30 |
| W3 | `The text is "<input>", from rows[i].text. <question>` | 55 | 58 |

- W3, the row's text quoted into its own instruction, won at both widths. It matched single-row at width 1 and at width 10, and it flipped fewest answers against single-row. Width 10 cut tokens 2.3 times. The quote costs about 5 percent more tokens than W0.
- W2 shows that the question name does not act as a pointer. Ten look-alike questions with no row reference scored 30 of 77.
- Choice answers are noisy. W0 at width 1 sent the same bytes as 260's CA01 and flipped 8 of 77 top options. So a gap of a few items is noise, and no wording beats single-row beyond chance.

What this means for the design: packed `choice` quotes each row's text into its own instruction (W3). Before the engine ships it, rerun W3 on a larger sample at widths 1 and 10, with a repeat to size the noise. Ian can overturn the choice of W3 and the size of that check.
