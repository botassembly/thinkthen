# ADR 0055: Batches of decide, filter and rank send each record once

- Status: Accepted 2026-09-26 by Ian's ruling. Ian can overturn it. Ticket 0146 builds it
- Date: 2026-09-26

This ADR amends ADR 0048 item 1 and ADR 0053 item 5 for `decide`, `filter` and `rank`. ADR 0048 says that changing one of its items takes a new ADR. This is that ADR. `sdlc/issues/2026-09-26-a-batch-sends-each-record-twice-and-its-question-with-every-record.md` holds the argument. Local experiment 275 holds the measurements.

## Context

ADR 0048 item 1 puts every record of a batch into the evidence as `{"records":[R1,…,RN]}`. Each record's question then quotes the record again. A batch therefore carries each record twice. The backend bills every question's instructions in full, so it also bills the user's question once for every record.

ADR 0054 item 5 says a default is not settled until it has been measured against the simplest alternative that asks less. Nobody had measured the batch form against a form that drops the records list. Local experiment 275 measured it on 2026-09-26.

The experiment asked four keyed yes/no questions over the 306 Beatles titles with model `jev-1.13.0`, cut at 0.7. Three tasks took the song titles, 306 to a request. The fourth took a generated card of 250 to 288 words for each song, 51 to a request. Each card states the song's release date among later distractor years. Each batched form ran three times on the same bytes and once on each of three shuffled record orders. The unbatched request, one record a request, ran once.

The quoted form sends the fixed sentence `Each question quotes the text it asks about.` as the evidence. Each question reads `The text is ` then the record as a JSON value, then `. `, then the user's question unchanged. That question is ADR 0048 item 1's question, byte for byte.

Right answers of 306 are given as the lowest and highest of the six batched runs. The repeat spread covers the three same-bytes runs. The order spread covers the three shuffled orders. Input tokens are for one whole run.

| Task | Unbatched | Today's form | Quoted |
| --- | --- | --- | --- |
| Title, "It appears on the album Abbey Road" | 286; 88,933 tokens | 266 to 277; repeats 0, orders 11; 13,799 tokens | 283 to 287; repeats 2, orders 3; 11,468 tokens |
| Title, "Ringo Starr sings lead on it" | 289; 88,933 tokens | 292 to 297; repeats 0, orders 5; 13,799 tokens | 290 to 292; repeats 2, orders 0; 11,468 tokens |
| Title, "It was released before 1965" | 253; 89,545 tokens | 258 to 283; repeats 7, orders 12; 14,411 tokens | 255 to 259; repeats 1, orders 4; 12,080 tokens |
| Card, "first released before 1965" | 306; 191,071 tokens | 306; repeats 0, orders 0; 219,182 tokens | 306; repeats 0, orders 0; 114,427 tokens |

The quoted form stayed within 4 answers over repeats and orders on every task. It never fell more than 3 answers below the unbatched run. It cost 16 to 17% less than today's form on titles and 48% less on cards. Today's form cost 15% more than the unbatched run on cards, because it bills each card twice. Today's form also moved with record order by 11 and 12 answers on two title tasks.

The release-year title task shows the one measured loss. Today's form scored 276 to 283 on the table's own order, above the quoted form. The same form scored 258 to 270 over shuffled orders, so it swung from 258 to 283 with order. The quoted form scored 255 to 259 over all six runs. Under ADR 0054 item 3 a gain that order alone erases is noise, and the cheaper form is the default.

The experiment also measured `choose` and `tag` on short titles only. Today's form won both beyond the noise there. Nobody has measured them on long records.

## Decision

1. **The evidence of a plain batch is one fixed sentence.** For `decide`, `filter` and `rank`, a batch without `--context` sends `Each question quotes the text it asks about.` as its evidence, as plain text. It no longer sends `{"records":[…]}`. Each record still appears once, inside its own question, exactly as ADR 0048 item 1 quotes it. Equal evidence inside one batch is still asked once.
2. **A context changes nothing.** With `--context`, the evidence is the context text, and records appear only in their questions, as ADR 0048 item 1 already says. A caller who wants shared background keeps `--context` for it.
3. **A batch of one stays byte-identical.** Without a context, a batch whose records all share one evidence is not batched. The planner sends the record as the evidence and the question unquoted, which is today's request. This ADR changes only the evidence of a batch of two or more distinct records. `--batch 1`, `THINKTHEN_BATCH=1` and a question file's `"batch": 1` therefore keep every recording, cache entry and fixture valid.
4. **Other verbs keep their form.** ADR 0048 item 7 lists the batching verbs. `choose` and `tag` keep ADR 0048 item 1's records list until their batching tickets, B8 and B9, measure them on long records. `score`, `annotate` and `recognize` are unchanged. `find` and `relate` are unchanged, since item 7 already leaves them out of record batching.
5. **Records of one batch are no longer each other's evidence.** ADR 0053 item 5 says the records of one batch are evidence for each other. For `decide`, `filter` and `rank` without a context, a record no longer sits in its neighbours' evidence, so that caution no longer applies to them. It still applies to `choose` and `tag` until they change form. A context remains shared evidence for every record of the batch.
6. **The speed target and test 9 stay.** ADR 0048 item 12's target stays: `filter` over the 306 titles in under half a second at the default throttle of 4. Batching test 9 and the page report shuffled record orders beside same-bytes repeats.

## Consequences

- Ticket 0146 fixes the batched request bytes. It builds this form before it lands, so no cached batch of today's form needs to survive.
- The evidence of a plain batch no longer grows with its records. A batch still closes at ADR 0048 item 2's limits, but more records fit under the byte ceiling.
- `records.md` and `decide.md` drop the trust statement for these three verbs and keep it for `choose` and `tag`.
- The question-in-evidence forms that local experiment 275 measured stay unused. They lost 11 to 35 answers on cards.

## What this amends

| Where | What changes |
| --- | --- |
| ADR 0048 item 1 | For `decide`, `filter` and `rank`, a plain batch's evidence is the fixed sentence of item 1 above |
| ADR 0053 item 5 | The trust statement no longer applies to `decide`, `filter` and `rank` without a context |
| Batch issue of 2026-09-26 | `decide`, `filter` and `rank` are settled. `choose` and `tag` stay open |

## What Ian can overturn

Ian's ruling of 2026-09-26:

1. The quoted form as the default batch form for `decide`, `filter` and `rank`.

The batch issue's recommendations, adopted with the ruling:

2. The exact fixed sentence, taken unchanged from the measured arm.
3. `choose` and `tag` keeping today's form until B8 and B9 measure them on long records.

## Amendment

2026-09-26: Item 5's caution also stays for `score`, because item 4 keeps its records list.
