# A batch sends each record twice and its question with every record

Status: Open. Filed 2026-09-26 from the ten-function efficiency audit under ADR 0054, local experiment 275. Round 2 added the same day measures long records, repeats and record orders. Owner: the batching queue, before ticket 0146 lands. Does not block 0.1 by itself. Ticket 0146 fixes the batched request bytes, so a later change misses every cached batch.

## What happens today

The batch planner puts every record in the evidence and quotes it again in its own question.

- `crates/thinkthen/src/core/batch.rs` lines 433 and 434 build the evidence as `{"records":[…]}` over every record of the batch.
- Lines 417 to 423 build each record's question as `The text is <record>. <question>`. The record travels a second time, and the whole question travels once for every record.
- Ticket 0146's settings table already says it: "a batch carries each record twice, in the evidence and in its quoted question".

No one measured this form against a form that sends less. The batching record compares widths and wrapper shapes, and every arm kept both the list and the quote.

## Why it breaks ADR 0054

- Item 1 and the audit pattern "evidence sent more than once". The list repeats text that each question already quotes. The backend bills every question's instructions in full, so the question text is also billed once for every record.
- Item 5. The planner's form became the default for `decide`, `filter` and `rank` in ticket 0146, and for `choose`, `tag`, `score` and `annotate` in B8 to B10. It was never measured against the simplest alternative that sends less.

## Measured

Local experiment 275, model `jev-1.13.0`, two rounds. Keys come from the Beatles Bench song table. Yes/no tasks cut at 0.7.

### Round 2: `decide`, `filter` and `rank`, repeats and orders

Four keyed yes/no tasks over 306 records. Three take the song titles, 306 to a request. The fourth takes a generated card of 250 to 288 words for each song. Each card states the song's release date among later distractor years, such as remasters and covers. Cards go 51 to a request, so the planner's doubled records stay under the backend's token limit.

The forms:

- **Planner.** Today's `core/batch.rs` form: the records list, and each question quotes its record and repeats the question.
- **Quoted.** No list. The evidence is one fixed sentence, and each question quotes its record and the question as today.
- **Question in the evidence, quoted.** The evidence is `{"question": QUESTION, "records": […]}`. Each question reads `The question holds for the text "<record>".` Each record still travels twice.
- **Question in the evidence, by position.** The same evidence. Each question reads `The question holds for the record at records[K].` Each record and the question travel once.

Each batched form ran three times on the same bytes and once on each of three shuffled record orders. The unbatched request ran once. Right answers of 306, as the lowest and highest of the six runs, then the spread over the three repeats and over the three orders:

| Task | Unbatched | Planner | Quoted | Question in evidence, quoted | Question in evidence, by position |
| --- | --- | --- | --- | --- | --- |
| Title, "It appears on the album Abbey Road" | 286 | 266 to 277; 0 and 11 | 283 to 287; 2 and 3 | 294 to 297; 1 and 1 | 287 to 291; 1 and 4 |
| Title, "Ringo Starr sings lead on it" | 289 | 292 to 297; 0 and 5 | 290 to 292; 2 and 0 | 294 to 299; 1 and 5 | 292; 0 and 0 |
| Title, "It was released before 1965" | 253 | 258 to 283; 7 and 12 | 255 to 259; 1 and 4 | 249 to 273; 1 and 7 | 228 to 236; 1 and 2 |
| Card, "first released before 1965" | 306 | 306; 0 and 0 | 306; 0 and 0 | 271 to 295; 6 and 24 | 232 to 257; 3 and 25 |

Input tokens a run:

| Records | Unbatched | Planner | Quoted | Question in evidence, quoted | By position |
| --- | --- | --- | --- | --- | --- |
| Titles | 88,933 | 13,799 | 11,468 | 8,624 | 8,636 |
| Cards | 191,071 | 219,182 | 114,427 | 214,442 | 112,273 |

- The planner form costs more than one request a record on long records. It bills each card twice, 15% over the unbatched run and 92% over the quoted form.
- The planner form moves with its neighbours. Its order spread was 11 and 12 answers on two title tasks, against a repeat spread of 0 to 7.
- The quoted form held within 4 answers over repeats and orders on every task. It never fell below the unbatched run by more than 3 answers. It cost 17% less than the planner on titles and 48% less on cards.
- Putting the question in the evidence won on two title tasks. It lost 11 to 35 answers on the cards, all of them false yeses, and its order spread reached 24. It fails on the records that batching must carry.
- Referring to a record by position lost everywhere by missing yeses: 18 of 18 missed at worst on Abbey Road, and 46 of 77 on the cards. It confirms experiment 261's finding that a question must quote its record.

The cards state their answer plainly, so three forms reached 306. The card task shows that a form holds on long records. It does not rank forms that all hold.

### Round 1: `choose` and `tag`

One request of 306 titles, or two of 153. Right answers of 306 over three same-bytes runs, and input tokens a run.

| Task | Unbatched | Planner | Quoted | Question in evidence, quoted |
| --- | --- | --- | --- | --- |
| `choose` first album of 14 options | 161, 164; 128,407 | 172 to 176; 53,537 | 156 to 163; 51,210 | 169 to 172; 48,389 |
| `tag` lead singers, exact label sets | 88, 93; 131,773 | 140 to 143; 65,717 | 111 to 116; 63,390 | 93 to 110; 35,167 |

The planner form wins on `choose` and on `tag`, beyond the noise.

## Recommended fix

Under ADR 0054 items 3 and 5, the quoted form is the default for `decide`, `filter` and `rank`.

1. Build batches for `decide`, `filter` and `rank` in the quoted form before ticket 0146 fixes the batched bytes. The evidence is one fixed sentence, and each question quotes its record and the question. It is the cheapest form that held on all four tasks. It stayed within the noise of the unbatched run, it does not move with record order, and it halves the bill on long records.
2. State the one measured loss plainly. On the release-year title task, the planner form scored 18 to 26 answers higher on the table's own order, but only 258 to 270 across shuffled orders, against the quoted form's 255 to 259. A user who wants the neighbour effect gets it from `--context`, which ADR 0048 already gives.
3. Drop the question-in-evidence forms. They lose beyond the noise on long records.
4. Keep the planner form for `choose` and `tag` until B8 and B9 measure them on long records. Their round 1 win came from short titles, where the doubled record costs little.
5. The quoted form also removes the trust caution of ADR 0053 item 5 for these three verbs. A record no longer sits in every other record's evidence. Ticket 0146's page and test 9 should say so and should report shuffled orders beside same-bytes repeats.

Ian can overturn this. The planner form is a ruling of ADR 0048 as built by ticket 0144, so the switch needs his word or the batching owner's ticket.

## Blocks 0.1

No. It should be settled before ticket 0146 lands, because each later change to the batch form misses every cached batch.
